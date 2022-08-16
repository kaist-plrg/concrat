use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
};

use etrace::some_or;
use rustc_hir::{
    def::{DefKind, Res},
    Expr, ExprKind, Item, ItemKind, VariantData,
};
use rustc_index::{bit_set::BitSet, vec::Idx};
use rustc_lint::{LateContext, LateLintPass, LintPass};
use rustc_middle::mir::{BasicBlock, Location, TerminatorKind};
use rustc_span::{def_id::DefId, Span};

use super::{
    get_function_call,
    intra::{available_guards, live_guards, AnalysisContext},
    Arg, FunctionSummary,
};
use crate::{
    analysis::{compute_mutex_line, AnalysisSummary},
    callback::{compile_with, LatePass},
    graph::{compute_sccs, inverse, post_order, transitive_closure},
    util::{
        current_function, def_id_to_item_name, def_id_to_span, expr_to_path, function_params,
        resolve_path, span_lines, span_to_string, type_of, type_to_string, unwrap_call,
        unwrap_cast_recursively, unwrap_ptr_from_type, ExprPath, ExprPathProj, Id,
    },
};

static VERBOSE: AtomicBool = AtomicBool::new(false);
static SUMMARY: Mutex<Option<AnalysisSummary>> = Mutex::new(None);

fn verbose() -> bool {
    VERBOSE.load(Ordering::Relaxed)
}

pub fn run(args: Vec<String>, verbose: bool) -> AnalysisSummary {
    VERBOSE.store(verbose, Ordering::Relaxed);
    let exit_code = compile_with(args, vec![GlobalPass::new]);
    assert_eq!(exit_code, 0);
    SUMMARY.lock().unwrap().take().unwrap()
}

#[derive(Default, Debug)]
struct GlobalPass {
    mutexes: HashMap<DefId, HashSet<ExprPath>>,
    params: HashMap<DefId, Vec<(String, String)>>,
    args_per_type: HashMap<String, HashSet<ExprPath>>,
    calls: HashMap<Span, Vec<Arg>>,
    accesses: HashMap<DefId, Vec<(Span, ExprPath, bool)>>,
    call_graph: HashMap<DefId, HashSet<DefId>>,
    path_types: HashMap<(DefId, ExprPath), String>,
    mutexes_per_struct: HashMap<String, HashSet<String>>,
    thread_entries: HashSet<DefId>,
    init_or_destory: HashMap<DefId, HashSet<ExprPath>>,
    globs: HashSet<String>,
}

impl GlobalPass {
    #[allow(clippy::new_ret_no_self)]
    fn new() -> Box<LatePass> {
        Box::new(Self::default())
    }
}

impl LintPass for GlobalPass {
    fn name(&self) -> &'static str {
        "GlobalPass"
    }
}

impl GlobalPass {
    fn possible_mutexes(&self) -> HashSet<ExprPath> {
        let mut mutexes: HashSet<_> = self.mutexes.values().flatten().cloned().collect();
        for (def_id, ms) in &self.mutexes {
            let params = self.params.get(def_id).unwrap();
            for m in ms {
                if m.is_variable() {
                    continue;
                }
                let (_, t) = some_or!(params.iter().find(|(p, _)| p == &m.base), continue);
                let args = some_or!(self.args_per_type.get(t), continue);
                for arg in args {
                    let mut m = m.clone();
                    m.set_base(arg);
                    mutexes.insert(m);
                }
                for (x, t1) in self.params.values().flatten() {
                    if t == t1 {
                        let mut m = m.clone();
                        m.set_base(&ExprPath::new(x.clone(), vec![]));
                        mutexes.insert(m);
                    }
                }
            }
        }
        mutexes
    }

    fn thread_functions(&self) -> HashSet<DefId> {
        if self.thread_entries.is_empty() {
            return HashSet::new();
        }
        let graph = transitive_closure(self.call_graph.clone());
        let mut thread_entries = self.thread_entries.clone();
        for f in self
            .thread_entries
            .iter()
            .flat_map(|f| graph.get(f).unwrap())
        {
            thread_entries.insert(*f);
        }
        thread_entries
    }
}

impl<'tcx> LateLintPass<'tcx> for GlobalPass {
    fn check_item(&mut self, ctx: &LateContext<'tcx>, i: &'tcx Item<'tcx>) {
        match &i.kind {
            ItemKind::Fn(_, _, bid) => {
                let def_id = i.def_id.to_def_id();
                self.call_graph.insert(def_id, HashSet::new());
                self.params.insert(def_id, function_params(ctx, *bid));
            }
            ItemKind::Static(_, _, _) => {
                self.globs.insert(i.ident.to_string());
            }
            ItemKind::Struct(VariantData::Struct(fs, _), _) => {
                for f in fs.iter() {
                    let ty = span_to_string(ctx, f.ty.span);
                    if ty.contains("pthread_mutex_t") || ty.contains("pthread_spinlock_t") {
                        self.mutexes_per_struct
                            .entry(i.ident.to_string())
                            .or_default()
                            .insert(f.ident.to_string());
                    }
                }
            }
            _ => (),
        }
    }

    fn check_expr(&mut self, ctx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) {
        let curr = some_or!(current_function(ctx), return);
        if let Some(path) = expr_to_path(ctx, e) {
            let typ = type_to_string(unwrap_ptr_from_type(type_of(ctx, e.hir_id)));
            self.path_types.insert((curr, path.clone()), typ);
            if !path.is_variable() || self.globs.contains(&path.base) {
                self.accesses
                    .entry(curr)
                    .or_default()
                    .push((e.span, path, false));
            }
        }
        match &e.kind {
            ExprKind::Call(f, arg_exprs) => {
                if let Some(Res::Def(DefKind::Fn, def_id)) = resolve_path(ctx, f) {
                    if let Some(v) = self.call_graph.get_mut(&curr) {
                        v.insert(def_id);
                    }
                }

                let args: Vec<_> = arg_exprs.iter().map(|arg| Arg::new(ctx, arg)).collect();

                let f = span_to_string(ctx, f.span);
                let mut add_mutex = |i: usize| {
                    self.mutexes
                        .entry(curr)
                        .or_default()
                        .insert(args[i].path.clone().unwrap())
                };
                match f.as_str() {
                    "pthread_mutex_lock"
                    | "pthread_mutex_unlock"
                    | "pthread_mutex_trylock"
                    | "pthread_spin_lock"
                    | "pthread_spin_unlock"
                    | "pthread_spin_trylock" => {
                        add_mutex(0);
                    }
                    "pthread_cond_wait" | "pthread_cond_timedwait" => {
                        add_mutex(1);
                    }
                    "pthread_create" => {
                        let t_fun = unwrap_cast_recursively(unwrap_call(&arg_exprs[2]));
                        if let Some(Res::Def(DefKind::Fn, t_fun_id)) = resolve_path(ctx, t_fun) {
                            self.thread_entries.insert(t_fun_id);
                        }
                    }
                    "pthread_mutex_init"
                    | "pthread_mutex_destroy"
                    | "pthread_spin_init"
                    | "pthread_spin_destroy" => {
                        add_mutex(0);
                        if let Some(mut path) = args[0].path.clone() {
                            if path.pop().is_some() {
                                self.init_or_destory.entry(curr).or_default().insert(path);
                            }
                        }
                    }
                    _ => (),
                }

                for arg in &args {
                    if let Some(path) = arg.path.as_ref() {
                        self.args_per_type
                            .entry(arg.typ.clone())
                            .or_default()
                            .insert(path.clone());
                    }
                }

                self.calls.insert(e.span, args);
            }
            ExprKind::Assign(e, _, _) | ExprKind::AssignOp(_, e, _) => {
                let mut path = some_or!(expr_to_path(ctx, e), return);
                while !path.is_variable() || self.globs.contains(&path.base) {
                    self.accesses
                        .entry(curr)
                        .or_default()
                        .push((e.span, path.clone(), true));
                    if path.pop().is_none() {
                        break;
                    }
                }
            }
            _ => (),
        }
    }

    fn check_crate_post(&mut self, ctx: &LateContext<'tcx>) {
        if verbose() {
            println!("{:#?}", self);
        }

        // user-defined functions
        let functions: HashSet<_> = self.call_graph.iter().map(|(f, _)| f).cloned().collect();
        // remove library functions from call graph
        for callees in self.call_graph.values_mut() {
            callees.retain(|f| functions.contains(f));
        }
        // find strongly connected components
        let (component_graph, component_elems) = compute_sccs(&self.call_graph);
        // compute post order traversal
        let inv_component_graph = inverse(&component_graph);
        let po = post_order(&component_graph, &inv_component_graph);

        // find possible mutex expressions
        let mutexes: HashSet<_> = self.possible_mutexes();
        // expression-to-id map
        let mutexes: HashMap<_, _> = mutexes
            .iter()
            .enumerate()
            .map(|(i, s)| (s.clone(), i))
            .collect();
        // id-to-expression map
        let inv_mutexes: HashMap<_, _> = mutexes.iter().map(|(s, i)| (*i, s.clone())).collect();

        // function-to-summary map
        let mut function_summary_map: HashMap<DefId, FunctionSummary> = HashMap::new();

        // post order traversal of call graph
        for component in po.iter().flatten() {
            let funcs = component_elems.get(component).unwrap();
            assert_eq!(funcs.len(), 1);
            let def_id = funcs.iter().next().unwrap();
            assert!(!self.call_graph.get(def_id).unwrap().contains(def_id));

            // analysis context
            let tcx = ctx.tcx;
            let body = tcx.optimized_mir(def_id);
            let ana_ctx = AnalysisContext::new(
                &mutexes,
                &inv_mutexes,
                &function_summary_map,
                &self.params,
                &self.calls,
                body,
                ctx,
            );

            // live guard analysis
            let mut results = live_guards(ana_ctx.clone());
            results.seek_to_block_start(BasicBlock::from_usize(0));
            let entry_mutex = results.get().clone();

            // available guard analysis
            let mut results = available_guards(ana_ctx, &entry_mutex);
            let return_opt = body.basic_blocks().iter_enumerated().find(|(_, bbd)| {
                matches!(
                    bbd.terminator.as_ref().unwrap().kind,
                    TerminatorKind::Return
                )
            });
            let ret_mutex = if let Some((return_bb, _)) = return_opt {
                results.seek_to_block_end(return_bb);
                results.get().0.clone()
            } else {
                BitSet::new_empty(entry_mutex.domain_size())
            };

            // guards propagated by function calls
            let mut propagation: Vec<(DefId, BitSet<Id>)> = vec![];
            // for each basic block
            for (block, bbd) in body.basic_blocks().iter_enumerated() {
                // get terminator
                let tm = some_or!(&bbd.terminator, continue);
                // get callee
                let func = some_or!(get_function_call(tm), continue);
                // skip library functions
                if !functions.contains(&func) {
                    continue;
                }

                // location of function call
                let statement_index = bbd.statements.len();
                let location = Location {
                    block,
                    statement_index,
                };
                // find guards held before function call
                results.seek_before_primary_effect(location);
                let v = &results.get().0;
                // update list
                propagation.push((func, v.clone()));
            }

            // guards held for each access
            let mut access: Vec<(ExprPath, BitSet<Id>, bool)> = vec![];
            // if function has any access
            if let Some(accesses) = self.accesses.get(def_id) {
                // for each basic block
                for (block, bbd) in body.basic_blocks().iter_enumerated() {
                    // for each statement
                    for (statement_index, stmt) in bbd.statements.iter().enumerate() {
                        // find guards held before each statement
                        let location = Location {
                            block,
                            statement_index,
                        };
                        results.seek_before_primary_effect(location);
                        let v = results.get().0.clone();

                        // get span of statement
                        let span = stmt.source_info.span;
                        // for each access
                        for (s, path, write) in accesses {
                            // if spans overlap
                            if s.overlaps(span) {
                                access.push((path.clone(), v.clone(), *write));
                            }
                        }
                    }
                }
            }

            // guards held at each span
            let mut span_mutex: Vec<(Span, BitSet<Id>)> = vec![];
            // for each basic block
            for (block, bbd) in body.basic_blocks().iter_enumerated() {
                // for each statement
                for (statement_index, stmt) in bbd.statements.iter().enumerate() {
                    // find guards held before each statement
                    let location = Location {
                        block,
                        statement_index,
                    };
                    results.seek_before_primary_effect(location);
                    let v = results.get().0.clone();
                    // get span of statement
                    let span = stmt.source_info.span;
                    // add to list
                    span_mutex.push((span, v.clone()));
                }

                // if terminator exists
                if let Some(tm) = &bbd.terminator {
                    match &tm.kind {
                        TerminatorKind::Call { .. } | TerminatorKind::Return => (),
                        _ => continue,
                    }
                    // find guards held before each statement
                    let location = Location {
                        block,
                        statement_index: bbd.statements.len(),
                    };
                    results.seek_before_primary_effect(location);
                    let v = results.get().0.clone();
                    // get span of statement
                    let span = tm.source_info.span;
                    // add to list
                    span_mutex.push((span, v.clone()));
                }
            }

            // create summary
            let summary =
                FunctionSummary::new(entry_mutex, ret_mutex, propagation, access, span_mutex);
            // save summary
            function_summary_map.insert(*def_id, summary);
        }

        // find root nodes
        let iter_roots: HashSet<_> = inv_component_graph
            .iter()
            .filter(|(_, preds)| preds.is_empty())
            .flat_map(|(n, _)| component_elems.get(n).unwrap())
            .cloned()
            .collect();
        // initialize work list with reverse post order traversal
        let mut work_list: VecDeque<_> = po
            .iter()
            .flatten()
            .rev()
            .flat_map(|n| component_elems.get(n).unwrap())
            .cloned()
            .collect();
        // initialize abstract states
        let mut abs_states: HashMap<DefId, BitSet<Id>> = HashMap::new();
        for func in &work_list {
            let init_st = if iter_roots.contains(func) {
                function_summary_map.get(func).unwrap().entry_mutex.clone()
            } else {
                BitSet::new_filled(mutexes.len())
            };
            abs_states.insert(*func, init_st);
        }

        // arg-to-param aliasing
        let alias_id = |id: Id, args: &[Arg], params: &[(String, String)]| {
            let m = inv_mutexes.get(&id.index()).unwrap().clone();
            if m.is_variable() {
                return id;
            }
            let (i, mut m) = some_or!(
                args.iter().enumerate().find_map(|(i, arg)| {
                    let m = m.strip_prefix(arg.path.as_ref()?)?;
                    Some((i, m))
                }),
                return id
            );
            let arg = &params[i].0;
            m.add_base(arg.clone());
            Id::new(*mutexes.get(&m).unwrap())
        };

        // compute fixed point
        while let Some(func) = work_list.pop_front() {
            let propagation = &function_summary_map.get(&func).unwrap().propagation;
            let st = abs_states.get(&func).unwrap().clone();
            let succs = self.call_graph.get(&func).unwrap();
            for succ in succs {
                // compute held mutexes
                let mut ms = propagation.get(succ).unwrap().clone();
                ms.union(&st);

                // consider aliasing
                let func_span = def_id_to_span(ctx.tcx, func);
                let succ_name = def_id_to_item_name(ctx.tcx, *succ);
                let prefix = format!("{}(", succ_name);
                let params = self.params.get(succ).unwrap();
                let ms = self
                    .calls
                    .iter()
                    .filter_map(|(span, args)| {
                        if span.overlaps(func_span)
                            && span_to_string(ctx, *span).starts_with(&prefix)
                        {
                            let mut ams = BitSet::new_empty(ms.domain_size());
                            for i in ms.iter() {
                                ams.insert(alias_id(i, args, params));
                            }
                            Some(ams)
                        } else {
                            None
                        }
                    })
                    .reduce(|mut ov, nv| {
                        ov.intersect(&nv);
                        ov
                    })
                    .unwrap();

                // update state
                let succ_st = abs_states.get_mut(succ).unwrap();
                if succ_st.intersect(&ms) && !work_list.contains(succ) {
                    work_list.push_back(*succ);
                }
            }
        }

        // update function summaries
        for (def_id, summary) in &mut function_summary_map {
            let mut abs_st = abs_states.get(def_id).unwrap().clone();
            assert!(abs_st.superset(&summary.entry_mutex));
            abs_st.subtract(&summary.entry_mutex);
            summary.propagation_mutex = abs_st;
        }

        // accesses to global variables
        let mut global_access: HashMap<ExprPath, Vec<(DefId, BitSet<Id>, bool)>> = HashMap::new();
        // accesses to struct fields
        let mut struct_access: Vec<(ExprPath, DefId, BitSet<Id>, bool)> = vec![];

        // classify accesses
        for (def_id, summary) in &function_summary_map {
            let prop = &summary.propagation_mutex;
            for (path, (v, w)) in &summary.access {
                let mut v = v.clone();
                v.union(prop);
                if path.is_struct() {
                    struct_access.push((path.clone(), *def_id, v, *w));
                } else {
                    global_access
                        .entry(path.clone())
                        .or_default()
                        .push((*def_id, v, *w));
                }
            }
        }

        let iv2mv = |is: &BitSet<Id>| {
            is.iter()
                .map(|i| inv_mutexes.get(&i.index()).unwrap().clone())
                .collect::<Vec<_>>()
        };

        // find functions reachable from pthread_create
        let thread_functions = self.thread_functions();
        if verbose() {
            println!("thread_functions: {:#?}", thread_functions);
        }

        let mut mutex_map: BTreeMap<String, String> = BTreeMap::new();
        let mut array_mutex_map: BTreeMap<String, String> = BTreeMap::new();
        let mut struct_mutex_map: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();

        // for each global variable access path
        for (path, mut accesses) in global_access {
            // skip read-only
            if accesses.iter().all(|(_, _, w)| !w) {
                continue;
            }

            // find candidate mutex
            let mut counts = [0usize].repeat(mutexes.len());
            for (_, v, _) in &accesses {
                for i in v.iter() {
                    counts[i.index()] += 1;
                }
            }
            let index = path.index();
            let cand_opt = counts
                .drain(..)
                .enumerate()
                .filter_map(|(i, x)| {
                    if x > 0 {
                        let m = inv_mutexes.get(&i).unwrap();
                        if index == m.index() {
                            Some((i, m, x))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .max_by_key(|(_, _, x)| *x);
            let (i, cand, x) = some_or!(cand_opt, continue);

            // function updating mutex map
            let mut add = || {
                let map = if index.is_none() {
                    &mut mutex_map
                } else {
                    &mut array_mutex_map
                };
                map.insert(path.base.clone(), cand.base.clone());
            };

            // if every access is safe, update mutex map
            if x == accesses.len() {
                add();
                continue;
            }

            // try filtering only when thread functions exist
            if thread_functions.is_empty() {
                continue;
            }

            // split accesses into safe/unsafe accesses
            let i = Id::new(i);
            let (safe, usafe): (Vec<_>, _) =
                accesses.drain(..).partition(|(_, v, _)| v.contains(i));
            assert_eq!(safe.len(), x);

            // skip read-only
            if safe.iter().all(|(_, _, w)| !w) {
                continue;
            }

            // if every unsafe access is in non-thread function, update mutex map
            if usafe.iter().all(|(f, _, _)| !thread_functions.contains(f)) {
                add();
            }
        }

        // find init or destroy functions per type
        let mut init_or_destroy_map: HashMap<_, HashSet<_>> = HashMap::new();
        for (def_id, paths) in &mut self.init_or_destory {
            for path in paths.iter() {
                let ty = some_or!(self.path_types.get(&(*def_id, path.clone())), continue);
                init_or_destroy_map
                    .entry(ty.clone())
                    .or_default()
                    .insert(*def_id);
            }
        }

        // group struct field accesses by type and field name
        let mut struct_access_per_type: HashMap<_, Vec<_>> = HashMap::new();
        for (mut path, def_id, v, w) in struct_access {
            // find longest prefix whose type has mutex
            let opt = loop {
                let last = some_or!(path.pop(), break None);
                let last = match last {
                    ExprPathProj::Field(f) => f,
                    _ => break None,
                };
                let typ = self.path_types.get(&(def_id, path.clone())).unwrap();
                if self.mutexes_per_struct.get(typ).is_some() {
                    break Some((typ.clone(), path, last));
                }
            };
            let (typ, path, field) = some_or!(opt, continue);
            struct_access_per_type
                .entry((typ, field))
                .or_default()
                .push((def_id, path, v, w));
        }

        // for each struct field access path
        for ((typ, field), mut accesses) in struct_access_per_type {
            // skip read-only
            if accesses.iter().all(|(_, _, _, w)| !w) {
                continue;
            }

            // find held mutexes that conform to path
            let mut accesses: Vec<_> = accesses
                .drain(..)
                .map(|(def_id, path, v, w)| {
                    let held: HashSet<_> = v
                        .iter()
                        .filter_map(|i| {
                            let mutex = inv_mutexes.get(&i.index()).unwrap().clone();
                            let mutex = mutex.strip_prefix(&path)?;
                            if mutex.is_variable() {
                                Some(mutex.base)
                            } else {
                                None
                            }
                        })
                        .collect();
                    (def_id, path, held, w)
                })
                .collect();

            // find candidate mutex
            let mut counts: HashMap<String, usize> = HashMap::new();
            for (_, _, ms, _) in &accesses {
                for m in ms {
                    let x = counts.entry(m.clone()).or_default();
                    *x += 1;
                }
            }
            let cand_opt = counts.drain().max_by_key(|(_, x)| *x);
            let (cand, x) = some_or!(cand_opt, continue);

            // function updating mutex map
            let mut add = || {
                struct_mutex_map
                    .entry(typ.clone())
                    .or_default()
                    .insert(field.clone(), cand.clone());
            };

            // if every access is safe, update mutex map
            if x == accesses.len() {
                add();
                continue;
            }

            // try filtering only when thread functions exist
            let empty = HashSet::new();
            let init_or_destroy = some_or!(init_or_destroy_map.get(&typ), &empty);
            if thread_functions.is_empty() && init_or_destroy.is_empty() {
                continue;
            }

            // split accesses into safe/unsafe accesses
            let (safe, usafe): (Vec<_>, _) = accesses
                .drain(..)
                .partition(|(_, _, ms, _)| ms.contains(&cand));
            assert_eq!(safe.len(), x);

            // skip read-only
            if safe.iter().all(|(_, _, _, w)| !w) {
                continue;
            }

            // if every unsafe access is in non-thread function, update mutex map
            if usafe
                .iter()
                .all(|(f, _, _, _)| !thread_functions.contains(f) || init_or_destroy.contains(f))
            {
                add();
            }
        }

        if verbose() {
            println!("{:?}", mutex_map);
            println!("{:?}", array_mutex_map);
            println!("{:?}", struct_mutex_map);

            let mut res: Vec<_> = function_summary_map.iter().collect();
            res.sort_by_key(|(def_id, _)| *def_id);
            for (def_id, summary) in res {
                let FunctionSummary {
                    entry_mutex,
                    ret_mutex,
                    propagation_mutex,
                    propagation,
                    access,
                    ..
                } = summary;
                let f = def_id_to_item_name(ctx.tcx, *def_id);
                let start: Vec<_> = iv2mv(entry_mutex);
                let end: Vec<_> = iv2mv(ret_mutex);
                let prop: Vec<_> = iv2mv(propagation_mutex);
                let propagation: HashMap<_, _> = propagation
                    .iter()
                    .map(|(succ, v)| (def_id_to_item_name(ctx.tcx, *succ), iv2mv(v)))
                    .collect();
                let access: HashMap<_, _> = access
                    .iter()
                    .map(|(path, (v, w))| (path, (iv2mv(v), w)))
                    .collect();
                println!(
                    "{} {:?} {:?} {:?} {:?} {:?}",
                    f, start, end, prop, propagation, access
                );
            }
        }

        let iv2mv = |is: &BitSet<Id>| {
            is.iter()
                .map(|i| inv_mutexes.get(&i.index()).unwrap().clone())
                .collect::<Vec<_>>()
        };
        let function_map: BTreeMap<_, _> = function_summary_map
            .iter()
            .map(|(def_id, summary)| {
                let FunctionSummary {
                    entry_mutex,
                    ret_mutex,
                    propagation_mutex,
                    span_mutex,
                    ..
                } = summary;
                let mut entry_mutex = iv2mv(entry_mutex);
                let mut ret_mutex = iv2mv(ret_mutex);
                let prop = iv2mv(propagation_mutex);
                for m in &prop {
                    entry_mutex.push(m.clone());
                    ret_mutex.push(m.clone());
                }
                let mut span_mutex_map: BTreeMap<_, Vec<ExprPath>> = BTreeMap::new();
                for (span, v) in span_mutex {
                    let mut ms = iv2mv(v);
                    span_mutex_map.entry(*span).or_default().append(&mut ms);
                    span_mutex_map
                        .entry(*span)
                        .or_default()
                        .append(&mut prop.clone());
                }
                for v in span_mutex_map.values_mut() {
                    v.sort();
                    v.dedup();
                }
                let mutex_line = compute_mutex_line(&span_mutex_map, |span| span_lines(ctx, *span));
                let f = def_id_to_item_name(ctx.tcx, *def_id);
                let summary =
                    crate::analysis::FunctionSummary::new(entry_mutex, ret_mutex, mutex_line);
                (f, summary)
            })
            .collect();
        let summary = AnalysisSummary {
            mutex_map,
            array_mutex_map,
            struct_mutex_map,
            function_map,
        };
        *SUMMARY.lock().unwrap() = Some(summary);
    }
}
