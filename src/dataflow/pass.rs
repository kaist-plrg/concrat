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
    Arg,
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
        let functions: HashSet<_> = self.call_graph.iter().map(|(f, _)| f).cloned().collect();
        for callees in self.call_graph.values_mut() {
            callees.retain(|f| functions.contains(f));
        }
        let (component_graph, component_elems) = compute_sccs(&self.call_graph);
        let inv_component_graph = inverse(&component_graph);
        let po = post_order(&component_graph, &inv_component_graph);

        println!("{:#?}", self.accesses);
        println!("{:#?}", self.params);
        println!("{:#?}", self.args_per_type);
        println!("{:#?}", self.call_graph);
        println!("{:#?}", component_elems);
        println!("{:?}", component_graph);

        let mutexes: HashSet<_> = self.possible_mutexes();
        let mutexes: HashMap<_, _> = mutexes
            .iter()
            .enumerate()
            .map(|(i, s)| (s.clone(), i))
            .collect();
        let inv_mutexes: HashMap<_, _> = mutexes.iter().map(|(s, i)| (*i, s.clone())).collect();
        let i2m = |i: Id| inv_mutexes.get(&i.index()).unwrap().clone();

        let mut function_mutex_map: HashMap<DefId, (BitSet<Id>, BitSet<Id>)> = HashMap::new();
        let mut call_mutex_map: HashMap<(DefId, DefId), BitSet<Id>> = HashMap::new();
        let mut function_access_map: HashMap<DefId, _> = HashMap::new();
        for component in po.iter().flatten() {
            let funcs = component_elems.get(component).unwrap();
            assert_eq!(funcs.len(), 1);
            let def_id = funcs.iter().next().unwrap();
            assert!(!self.call_graph.get(def_id).unwrap().contains(def_id));

            let tcx = ctx.tcx;
            let body = tcx.optimized_mir(def_id);
            let ana_ctx = AnalysisContext::new(
                &mutexes,
                &inv_mutexes,
                &function_mutex_map,
                &self.params,
                &self.calls,
                body,
                ctx,
            );

            let mut results = live_guards(ana_ctx.clone());
            results.seek_to_block_start(BasicBlock::from_usize(0));
            let start = results.get().clone();

            let mut results = available_guards(ana_ctx, &start);
            results.seek_to_block_end(BasicBlock::from_usize(body.basic_blocks().len() - 1));
            let end = results.get().0.clone();

            let mut func_call_mutex_map: HashMap<_, Vec<_>> = HashMap::new();
            for (block, bbd) in body.basic_blocks().iter_enumerated() {
                let tm = some_or!(&bbd.terminator, continue);
                let func = some_or!(get_function_call(tm), continue);
                if !functions.contains(&func) {
                    continue;
                }
                let statement_index = bbd.statements.len();
                let location = Location {
                    block,
                    statement_index,
                };
                results.seek_before_primary_effect(location);
                let v = results.get().0.clone();
                func_call_mutex_map.entry(func).or_default().push(v);
            }
            for (func, mut vs) in func_call_mutex_map {
                let v = vs
                    .drain(..)
                    .reduce(|mut a, b| {
                        a.intersect(&b);
                        a
                    })
                    .unwrap();
                call_mutex_map.insert((*def_id, func), v);
            }

            if let Some(accesses) = self.accesses.get(def_id) {
                let mut access_map: HashMap<ExprPath, Vec<(BitSet<Id>, bool)>> = HashMap::new();
                for (block, bbd) in body.basic_blocks().iter_enumerated() {
                    for (statement_index, stmt) in bbd.statements.iter().enumerate() {
                        let location = Location {
                            block,
                            statement_index,
                        };
                        results.seek_before_primary_effect(location);
                        let v = results.get().0.clone();
                        let span = stmt.source_info.span;
                        for (s, path, write) in accesses {
                            if s.overlaps(span) {
                                access_map
                                    .entry(path.clone())
                                    .or_default()
                                    .push((v.clone(), *write));
                            }
                        }
                    }
                }
                function_access_map.insert(*def_id, access_map);
            }

            function_mutex_map.insert(*def_id, (start, end));
        }

        let iter_roots: HashSet<_> = inv_component_graph
            .iter()
            .filter(|(_, preds)| preds.is_empty())
            .flat_map(|(n, _)| component_elems.get(n).unwrap())
            .cloned()
            .collect();
        let mut work_list: VecDeque<_> = po
            .iter()
            .rev()
            .flatten()
            .flat_map(|n| component_elems.get(n).unwrap())
            .cloned()
            .collect();
        let mut abs_states: HashMap<DefId, BitSet<Id>> = HashMap::new();
        for func in &work_list {
            let init_st = if iter_roots.contains(func) {
                function_mutex_map.get(func).unwrap().0.clone()
            } else {
                BitSet::new_filled(mutexes.len())
            };
            abs_states.insert(*func, init_st);
        }
        while let Some(func) = work_list.pop_front() {
            let st = abs_states.get(&func).unwrap().clone();
            let succs = self.call_graph.get(&func).unwrap();
            for succ in succs {
                let mut ms = call_mutex_map.get(&(func, *succ)).unwrap().clone();
                ms.union(&st);
                let succ_st = abs_states.get_mut(succ).unwrap();
                if succ_st.intersect(&ms) && !work_list.contains(succ) {
                    work_list.push_back(*succ);
                }
            }
        }

        for ((caller, callee), v) in &call_mutex_map {
            let caller = def_id_to_item_name(ctx.tcx, *caller);
            let callee = def_id_to_item_name(ctx.tcx, *callee);
            let v: Vec<_> = v.iter().map(i2m).collect();
            println!("{} {} {:?}", caller, callee, v);
        }
        println!();

        let mut res: Vec<_> = function_mutex_map.iter().collect();
        res.sort_by_key(|(def_id, _)| *def_id);
        for (def_id, (start, end)) in res {
            let f = def_id_to_item_name(ctx.tcx, *def_id);
            let start: Vec<_> = start.iter().map(i2m).collect();
            let end: Vec<_> = end.iter().map(i2m).collect();
            let abs_st: Vec<_> = abs_states.get(def_id).unwrap().iter().map(i2m).collect();
            println!("{} {:?} {:?} {:?}", f, start, end, abs_st);
        }

        println!("{:#?}", function_access_map);
    }
}
