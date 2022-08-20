use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
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
use rustc_middle::mir::BasicBlock;
use rustc_span::def_id::DefId;

use super::{
    intra::{available_guards, live_guards, AnalysisContext},
    visitor::Visitor,
    Arg, FunctionCodeSummary, FunctionSummary,
};
use crate::{
    analysis::{compute_mutex_line, AnalysisSummary},
    callback::{compile_with, LatePass},
    graph::{compute_sccs, inverse, post_order, transitive_closure},
    util::{
        current_function, def_id_to_item_name, expr_to_path, function_params, resolve_path,
        span_lines, span_to_string, type_of, type_to_string, unwrap_call, unwrap_cast_recursively,
        unwrap_ptr_from_type, ExprPath, ExprPathProj, Id,
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
    functions: BTreeMap<DefId, FunctionCodeSummary>,
    mutexes_per_struct: BTreeMap<String, BTreeSet<String>>,
    thread_entries: BTreeSet<DefId>,
    globs: BTreeSet<String>,
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
    fn possible_mutexes(
        &self,
        call_graph: &BTreeMap<DefId, BTreeSet<DefId>>,
    ) -> BTreeSet<ExprPath> {
        let mut abs_states = BTreeMap::new();
        let mut work_list = VecDeque::new();
        for (f, summary) in &self.functions {
            abs_states.insert(*f, summary.mutexes.clone());
            work_list.push_back(*f);
        }

        let inv_cg = inverse(call_graph);
        while let Some(f) = work_list.pop_front() {
            let st = abs_states.get(&f).unwrap().clone();
            let summary = self.functions.get(&f).unwrap();
            let preds = inv_cg.get(&f).unwrap();
            let succs = call_graph.get(&f).unwrap();

            for f1 in preds.union(succs) {
                let f1_summary = self.functions.get(f1).unwrap();
                let f1_st = abs_states.get_mut(f1).unwrap();
                let f1_st_len = f1_st.len();
                let mut add = |path: ExprPath| {
                    let mut v = path.projections.clone();
                    let len = v.len();
                    v.sort();
                    v.dedup();
                    if len - v.len() < 3 {
                        f1_st.insert(path);
                    }
                };

                if preds.contains(f1) {
                    let params = &summary.params;
                    for (_, callee, _, args) in &f1_summary.calls {
                        if *callee != f {
                            continue;
                        }
                        for path in &st {
                            if let Some(p) = path.clone().param_to_arg_aliasing(params, args) {
                                add(p);
                            } else if self.globs.contains(&path.base) {
                                add(path.clone());
                            }
                        }
                    }
                }

                if succs.contains(f1) {
                    let params = &f1_summary.params;
                    for (_, callee, _, args) in &summary.calls {
                        if callee != f1 {
                            continue;
                        }
                        for path in &st {
                            if let Some(p) = path.clone().arg_to_param_aliasing(args, params) {
                                add(p);
                            } else if self.globs.contains(&path.base) {
                                add(path.clone());
                            }
                        }
                    }
                }

                if f1_st_len < f1_st.len() && !work_list.contains(f1) {
                    work_list.push_back(*f1);
                }
            }
        }

        abs_states
            .drain_filter(|_, _| true)
            .flat_map(|x| x.1)
            .collect()
    }

    fn thread_functions(&self, call_graph: &BTreeMap<DefId, BTreeSet<DefId>>) -> BTreeSet<DefId> {
        if self.thread_entries.is_empty() {
            return BTreeSet::new();
        }
        let graph = transitive_closure(call_graph.clone());
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
                let summary = FunctionCodeSummary {
                    params: function_params(ctx, *bid),
                    ..Default::default()
                };
                self.functions.insert(def_id, summary);
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
        let summary = some_or!(self.functions.get_mut(&curr), return);
        if let Some(path) = expr_to_path(ctx, e) {
            let typ = type_to_string(unwrap_ptr_from_type(type_of(ctx, e.hir_id)));
            summary.add_path(path.clone(), typ);
            if !path.is_variable() || self.globs.contains(&path.base) {
                summary.add_access(e.span, path, false);
            }
        }
        match &e.kind {
            ExprKind::Call(f, arg_exprs) => {
                let args: Vec<_> = arg_exprs.iter().map(|arg| Arg::new(ctx, arg)).collect();

                let mut add_mutex = |i: usize| {
                    let mutex = args[i].path.clone().unwrap();
                    summary.add_mutex(mutex);
                };

                let f_name = span_to_string(ctx, f.span);
                match f_name.as_str() {
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
                                summary.add_init_or_destroy(path);
                            }
                        }
                    }
                    _ => (),
                }

                if let Some(Res::Def(DefKind::Fn, def_id)) = resolve_path(ctx, f) {
                    summary.add_call(e.span, def_id, f_name, args);
                }
            }
            ExprKind::Assign(e, _, _) | ExprKind::AssignOp(_, e, _) => {
                let mut path = some_or!(expr_to_path(ctx, e), return);
                while !path.is_variable() || self.globs.contains(&path.base) {
                    summary.add_access(e.span, path.clone(), true);
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

        // call graph
        let call_graph: BTreeMap<_, BTreeSet<_>> = self
            .functions
            .iter()
            .map(|(f, s)| {
                (
                    *f,
                    s.calls
                        .iter()
                        .map(|x| &x.1)
                        .filter(|f| self.functions.contains_key(f))
                        .cloned()
                        .collect(),
                )
            })
            .collect();
        // find strongly connected components
        let (component_graph, component_elems) = compute_sccs(&call_graph);
        // compute post order traversal
        let inv_component_graph = inverse(&component_graph);
        let po = post_order(&component_graph, &inv_component_graph);

        // find possible mutex expressions
        let mutexes: BTreeSet<_> = self.possible_mutexes(&call_graph);
        // expression-to-id map
        let mutexes: BTreeMap<_, _> = mutexes
            .iter()
            .enumerate()
            .map(|(i, s)| (s.clone(), i))
            .collect();
        // id-to-expression map
        let inv_mutexes: BTreeMap<_, _> = mutexes.iter().map(|(s, i)| (*i, s.clone())).collect();

        // function-to-summary map
        let mut function_summary_map: BTreeMap<DefId, FunctionSummary> = BTreeMap::new();

        // post order traversal of call graph
        for component in po.iter().flatten() {
            let mut funcs: Vec<_> = component_elems
                .get(component)
                .unwrap()
                .iter()
                .cloned()
                .collect();

            let mut entry_mutexes = vec![];
            let mut ret_mutexes = vec![];
            let mut propagations = vec![];
            let mut span_mutexes = vec![];
            for _ in &funcs {
                entry_mutexes.push(BitSet::new_empty(mutexes.len()));
                ret_mutexes.push(BitSet::new_filled(mutexes.len()));
                propagations.push(vec![]);
                span_mutexes.push(vec![]);
            }

            let span_args_map = self
                .functions
                .values()
                .flat_map(|s| {
                    s.calls
                        .iter()
                        .map(|(span, _, _, args)| (*span, args.clone()))
                        .collect::<Vec<_>>()
                })
                .collect();
            loop {
                for (i, def_id) in funcs.iter().enumerate() {
                    let summary = FunctionSummary::mutex_only(
                        entry_mutexes[i].clone(),
                        ret_mutexes[i].clone(),
                    );
                    function_summary_map.insert(*def_id, summary);
                }

                let mut changed = false;

                for (i, def_id) in funcs.iter().enumerate() {
                    // analysis context
                    let body = ctx.tcx.optimized_mir(def_id);
                    let ana_ctx = AnalysisContext::new(
                        &mutexes,
                        &inv_mutexes,
                        &function_summary_map,
                        &self.functions,
                        &span_args_map,
                        body,
                        ctx,
                    );

                    // live guard analysis
                    let mut results = live_guards(ana_ctx.clone()).into_results_cursor(body);
                    results.seek_to_block_start(BasicBlock::from_usize(0));
                    let entry_mutex = results.get().clone();

                    if entry_mutexes[i] != entry_mutex {
                        entry_mutexes[i] = entry_mutex;
                        changed = true;
                    }

                    // available guard analysis
                    let results = available_guards(ana_ctx, entry_mutexes[i].clone());
                    let mut visitor = Visitor::default();
                    results.visit_reachable_with(body, &mut visitor);
                    let Visitor {
                        return_state,
                        propagation,
                        span_mutex,
                    } = visitor;
                    let ret_mutex =
                        return_state.unwrap_or_else(|| BitSet::new_empty(mutexes.len()));

                    if ret_mutexes[i] != ret_mutex {
                        ret_mutexes[i] = ret_mutex;
                        changed = true;
                    }

                    propagations[i] = propagation;
                    span_mutexes[i] = span_mutex;
                }

                if !changed {
                    break;
                }
            }

            for ((((def_id, entry_mutex), ret_mutex), mut propagation), span_mutex) in funcs
                .drain(..)
                .zip(entry_mutexes.drain(..))
                .zip(ret_mutexes.drain(..))
                .zip(propagations.drain(..))
                .zip(span_mutexes.drain(..))
            {
                // guards propagated by function calls
                propagation.retain(|(f, _)| self.functions.contains_key(f));
                // guards held for each access
                let accesses = &self.functions.get(&def_id).unwrap().accesses;
                let access: Vec<(ExprPath, BitSet<Id>, bool)> = if accesses.is_empty() {
                    vec![]
                } else {
                    span_mutex
                        .iter()
                        .flat_map(|(span, v)| {
                            accesses
                                .iter()
                                .filter_map(|(s, path, w)| {
                                    if s.overlaps(*span) {
                                        Some((path.clone(), v.clone(), *w))
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect()
                };
                // create summary
                function_summary_map.insert(
                    def_id,
                    FunctionSummary::new(entry_mutex, ret_mutex, propagation, access, span_mutex),
                );
            }
        }

        // find root nodes
        let iter_roots: BTreeSet<_> = inv_component_graph
            .iter()
            .filter(|(_, preds)| preds.is_empty())
            .flat_map(|(n, _)| component_elems.get(n).unwrap())
            .cloned()
            .collect();
        // initialize work list with reverse post order traversal
        let mut work_list: VecDeque<_> = VecDeque::new();
        // initialize abstract states
        let mut abs_states: BTreeMap<DefId, BitSet<Id>> = BTreeMap::new();
        for func in self.functions.keys() {
            let init_st = if iter_roots.contains(func) {
                work_list.push_back(*func);
                function_summary_map.get(func).unwrap().entry_mutex.clone()
            } else {
                BitSet::new_filled(mutexes.len())
            };
            abs_states.insert(*func, init_st);
        }

        // compute fixed point
        while let Some(func) = work_list.pop_front() {
            let propagation = &function_summary_map.get(&func).unwrap().propagation;
            let st = abs_states.get(&func).unwrap().clone();
            let succs = call_graph.get(&func).unwrap();
            for succ in succs {
                // find arguments
                let argss: Vec<_> = self
                    .functions
                    .get(&func)
                    .unwrap()
                    .calls
                    .iter()
                    .filter(|x| x.1 == *succ)
                    .map(|x| &x.3)
                    .collect();

                // compute possible prefixes of propagated mutexes
                let mut possible_prefixes = argss
                    .iter()
                    .map(|v| {
                        v.iter()
                            .filter_map(|arg| arg.path.clone())
                            .collect::<BTreeSet<_>>()
                    })
                    .reduce(|mut os, ns| {
                        os.retain(|a| ns.contains(a));
                        os
                    })
                    .unwrap();
                for g in &self.globs {
                    possible_prefixes.insert(ExprPath::new(g.clone(), vec![]));
                }

                // compute held mutexes
                let empty = BitSet::new_empty(mutexes.len());
                let mut ms0 = propagation.get(succ).unwrap_or(&empty).clone();
                ms0.union(&st);

                let mut ms = BitSet::new_empty(mutexes.len());
                for i in ms0.iter() {
                    let m = inv_mutexes.get(&i.index()).unwrap();
                    let propagated = possible_prefixes
                        .iter()
                        .any(|p| m == p || m.strip_prefix(p).is_some());
                    if propagated {
                        ms.insert(i);
                    }
                }

                // consider aliasing
                let params = &self.functions.get(succ).unwrap().params;
                let ms = argss
                    .iter()
                    .map(|args| {
                        let mut ams = BitSet::new_empty(ms.domain_size());
                        for i in ms.iter() {
                            let m = inv_mutexes.get(&i.index()).unwrap().clone();
                            if let Some(m) = m.clone().arg_to_param_aliasing(args, params) {
                                ams.insert(Id::new(*mutexes.get(&m).unwrap()));
                            } else if self.globs.contains(&m.base) {
                                ams.insert(i);
                            };
                        }
                        ams
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
            // assert!(abs_st.superset(&summary.entry_mutex));
            abs_st.subtract(&summary.entry_mutex);
            summary.propagation_mutex = abs_st;
        }

        // accesses to global variables
        let mut global_access: BTreeMap<ExprPath, Vec<(DefId, BitSet<Id>, bool)>> = BTreeMap::new();
        // accesses to struct fields
        let mut struct_access: Vec<(ExprPath, DefId, BitSet<Id>, bool)> = vec![];

        // classify accesses
        for (def_id, summary) in &function_summary_map {
            let prop = &summary.propagation_mutex;
            for (path, v, w) in &summary.access {
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
        let thread_functions = self.thread_functions(&call_graph);
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
                        if m.is_struct() || index != m.index() {
                            None
                        } else {
                            Some((i, m, x))
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
        let mut init_or_destroy_map: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
        for (def_id, summary) in &mut self.functions {
            for path in &summary.init_or_destroy {
                let ty = some_or!(summary.path_types.get(path), continue);
                init_or_destroy_map
                    .entry(ty.clone())
                    .or_default()
                    .insert(*def_id);
            }
        }

        // group struct field accesses by type and field name
        let mut struct_access_per_type: BTreeMap<_, Vec<_>> = BTreeMap::new();
        for (mut path, def_id, v, w) in struct_access {
            let path_types = &self.functions.get(&def_id).unwrap().path_types;
            // find longest prefix whose type has mutex
            let opt = loop {
                let last = some_or!(path.pop(), break None);
                let last = match last {
                    ExprPathProj::Field(f) => f,
                    _ => break None,
                };
                let typ = path_types.get(&path).unwrap();
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
                    let held: BTreeSet<_> = v
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
            let mut counts: BTreeMap<String, usize> = BTreeMap::new();
            for (_, _, ms, _) in &accesses {
                for m in ms {
                    let x = counts.entry(m.clone()).or_default();
                    *x += 1;
                }
            }
            let cand_opt = counts.drain_filter(|_, _| true).max_by_key(|(_, x)| *x);
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
            let empty = BTreeSet::new();
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
                let propagation: BTreeMap<_, _> = propagation
                    .iter()
                    .map(|(succ, v)| (def_id_to_item_name(ctx.tcx, *succ), iv2mv(v)))
                    .collect();
                let access: Vec<_> = access
                    .iter()
                    .map(|(path, v, w)| (path, iv2mv(v), w))
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
