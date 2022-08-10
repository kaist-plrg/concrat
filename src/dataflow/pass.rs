use std::collections::{HashMap, HashSet, VecDeque};

use etrace::some_or;
use rustc_hir::{
    def::{DefKind, Res},
    Expr, ExprKind, Item, ItemKind,
};
use rustc_index::{bit_set::BitSet, vec::Idx};
use rustc_lint::{LateContext, LateLintPass, LintPass};
use rustc_middle::mir::{BasicBlock, Location};
use rustc_span::{def_id::DefId, Span};

use super::{
    get_function_call,
    intra::{available_guards, live_guards, AnalysisContext},
    Arg, FunctionSummary,
};
use crate::{
    callback::{compile_with, LatePass},
    graph::{compute_sccs, inverse, post_order},
    util::{
        current_function, def_id_to_item_name, expr_to_path, resolve_path, span_to_string,
        type_as_string, ExprPath, Id,
    },
};

pub fn run(args: Vec<String>) {
    let exit_code = compile_with(args, vec![GlobalPass::new]);
    assert_eq!(exit_code, 0);
}

#[derive(Default)]
struct GlobalPass {
    mutexes: HashMap<DefId, HashSet<ExprPath>>,
    params: HashMap<DefId, Vec<(String, String)>>,
    args_per_type: HashMap<String, HashSet<String>>,
    calls: HashMap<Span, Vec<Arg>>,
    accesses: HashMap<DefId, Vec<(Span, ExprPath, bool)>>,
    call_graph: HashMap<DefId, HashSet<DefId>>,
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
                    m.base = arg.clone();
                    mutexes.insert(m);
                }
            }
        }
        mutexes
    }
}

impl<'tcx> LateLintPass<'tcx> for GlobalPass {
    fn check_item(&mut self, ctx: &LateContext<'tcx>, i: &'tcx Item<'tcx>) {
        match &i.kind {
            ItemKind::Fn(_, _, bid) => {
                let def_id = i.def_id.to_def_id();
                self.call_graph.insert(def_id, HashSet::new());
                let params = ctx
                    .tcx
                    .hir()
                    .body(*bid)
                    .params
                    .iter()
                    .map(|p| {
                        (
                            span_to_string(ctx, p.pat.span).replace("mut ", ""),
                            type_as_string(ctx, p.pat.hir_id)
                                .replace("&mut ", "")
                                .replace("*mut ", "")
                                .replace("*const ", ""),
                        )
                    })
                    .collect();
                self.params.insert(def_id, params);
            }
            ItemKind::Static(_, _, _) => {
                self.globs.insert(i.ident.to_string());
            }
            _ => (),
        }
    }

    fn check_expr(&mut self, ctx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) {
        let curr = some_or!(current_function(ctx), return);
        if let Some(path) = expr_to_path(ctx, e) {
            if !path.is_variable() || self.globs.contains(&path.base) {
                self.accesses
                    .entry(curr)
                    .or_default()
                    .push((e.span, path, false));
            }
        }
        match &e.kind {
            ExprKind::Call(f, args) => {
                if let Some(Res::Def(DefKind::Fn, def_id)) = resolve_path(ctx, f) {
                    if let Some(v) = self.call_graph.get_mut(&curr) {
                        v.insert(def_id);
                    }
                }

                let args: Vec<_> = args.iter().map(|arg| Arg::new(ctx, arg)).collect();

                let f = span_to_string(ctx, f.span);
                if f == "pthread_mutex_lock" || f == "pthread_mutex_unlock" {
                    self.mutexes
                        .entry(curr)
                        .or_default()
                        .insert(args[0].path.clone().unwrap());
                }

                for arg in &args {
                    self.args_per_type
                        .entry(arg.typ.clone())
                        .or_default()
                        .insert(arg.expr.clone());
                }

                self.calls.insert(e.span, args);
            }
            ExprKind::Assign(e, _, _) | ExprKind::AssignOp(_, e, _) => {
                let path = some_or!(expr_to_path(ctx, e), return);
                if !path.is_variable() || self.globs.contains(&path.base) {
                    self.accesses
                        .entry(curr)
                        .or_default()
                        .push((e.span, path, true));
                }
            }
            _ => (),
        }
    }

    fn check_crate_post(&mut self, ctx: &LateContext<'tcx>) {
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
            results.seek_to_block_end(BasicBlock::from_usize(body.basic_blocks().len() - 1));
            let ret_mutex = results.get().0.clone();

            // set of guards held by function
            let mut node_mutex = entry_mutex.clone();
            node_mutex.union(&ret_mutex);

            // guards propagated by function calls
            let mut propagation: Vec<(DefId, BitSet<Id>)> = vec![];
            // for each basic block
            for (block, bbd) in body.basic_blocks().iter_enumerated() {
                // get terminator
                let tm = some_or!(&bbd.terminator, continue);
                // get callee
                let func = some_or!(get_function_call(tm), continue);

                // find guards held before function call
                let statement_index = bbd.statements.len();
                let location = Location {
                    block,
                    statement_index,
                };
                results.seek_before_primary_effect(location);
                let v = &results.get().0;

                // update node_mutex
                node_mutex.union(v);

                // skip library functions
                if !functions.contains(&func) {
                    continue;
                }

                // arguments for this call
                let args = self.calls.get(&tm.source_info.span).unwrap();
                // parameters of callee
                let params = self.params.get(&func).unwrap();
                // consider aliasing
                let alias_id = |id: Id| {
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

                // propagated guards
                let mut nv = BitSet::new_empty(mutexes.len());
                for id in v.iter() {
                    nv.insert(alias_id(id));
                }
                // update list
                propagation.push((func, nv));
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

            // create summary
            let summary =
                FunctionSummary::new(entry_mutex, node_mutex, ret_mutex, propagation, access);
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
            .rev()
            .flatten()
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

        // compute fixed point
        while let Some(func) = work_list.pop_front() {
            let st = abs_states.get(&func).unwrap().clone();
            let succs = self.call_graph.get(&func).unwrap();
            for succ in succs {
                let propagation = &function_summary_map.get(&func).unwrap().propagation;
                let mut ms = propagation.get(succ).unwrap().clone();
                ms.union(&st);
                let succ_st = abs_states.get_mut(succ).unwrap();
                if succ_st.intersect(&ms) && !work_list.contains(succ) {
                    work_list.push_back(*succ);
                }
            }
        }

        let mut access_map: HashMap<ExprPath, Vec<(DefId, BitSet<Id>, bool)>> = HashMap::new();
        for (def_id, summary) in &function_summary_map {
            for (path, (v, w)) in &summary.access {
                access_map
                    .entry(path.clone())
                    .or_default()
                    .push((*def_id, v.clone(), *w));
            }
        }
        for (path, mut v) in access_map {
            let v = some_or!(
                v.drain(..)
                    .filter_map(|(_, v, w)| if w { Some(v) } else { None })
                    .reduce(|mut ov, v| {
                        ov.intersect(&v);
                        ov
                    }),
                continue
            );
            println!("{} {:?}", path, v);
        }

        let mut res: Vec<_> = function_summary_map.iter().collect();
        res.sort_by_key(|(def_id, _)| *def_id);
        let iv2mv = |is: &BitSet<Id>| {
            is.iter()
                .map(|i| inv_mutexes.get(&i.index()).unwrap().clone())
                .collect::<Vec<_>>()
        };
        for (def_id, summary) in res {
            let FunctionSummary {
                entry_mutex,
                node_mutex,
                ret_mutex,
                propagation,
                access,
                ..
            } = summary;
            let f = def_id_to_item_name(ctx.tcx, *def_id);
            let start: Vec<_> = iv2mv(entry_mutex);
            let mid: Vec<_> = iv2mv(node_mutex);
            let end: Vec<_> = iv2mv(ret_mutex);
            let abs_st: Vec<_> = iv2mv(abs_states.get(def_id).unwrap());
            let propagation: HashMap<_, _> = propagation
                .iter()
                .map(|(succ, v)| (def_id_to_item_name(ctx.tcx, *succ), iv2mv(v)))
                .collect();
            let access: HashMap<_, _> = access
                .iter()
                .map(|(path, (v, w))| (path, (iv2mv(v), w)))
                .collect();
            println!(
                "{} {:?} {:?} {:?} {:?} {:?} {:?}",
                f, start, mid, end, abs_st, propagation, access
            );
        }
    }
}
