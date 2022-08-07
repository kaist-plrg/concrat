use std::collections::{HashMap, HashSet};

use etrace::some_or;
use rustc_hir::{
    def::{DefKind, Res},
    Expr, ExprKind, Item, ItemKind,
};
use rustc_index::{bit_set::BitSet, vec::Idx};
use rustc_lint::{LateContext, LateLintPass, LintPass};
use rustc_middle::mir::{BasicBlock, Location};
use rustc_mir_dataflow::Analysis;
use rustc_span::def_id::DefId;

use super::{
    get_function_call,
    intra::{AnalysisContext, AvailableGuards, LiveGuards},
};
use crate::{
    callback::{compile_with, LatePass},
    graph::compute_sccs,
    util::{current_function, expr_to_path, normalize_path, resolve_path, span_to_string, Id},
};

pub fn run(args: Vec<String>) {
    let exit_code = compile_with(args, vec![GlobalPass::new]);
    assert_eq!(exit_code, 0);
}

#[derive(Default)]
struct GlobalPass {
    mutexes: Vec<String>,
    call_graph: HashMap<DefId, HashSet<DefId>>,
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

impl<'tcx> LateLintPass<'tcx> for GlobalPass {
    fn check_item(&mut self, _: &LateContext<'tcx>, i: &'tcx Item<'tcx>) {
        match &i.kind {
            ItemKind::Fn(_, _, _) => {
                let def_id = i.def_id.to_def_id();
                self.call_graph.insert(def_id, HashSet::new());
            }
            _ => (),
        }
    }

    fn check_expr(&mut self, ctx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) {
        match &e.kind {
            ExprKind::Call(f, args) => {
                if let Some(curr) = current_function(ctx) {
                    if let Some(Res::Def(DefKind::Fn, def_id)) = resolve_path(ctx, f) {
                        if let Some(v) = self.call_graph.get_mut(&curr) {
                            v.insert(def_id);
                        }
                    }
                }
                let f = span_to_string(ctx, f.span);
                if f == "pthread_mutex_lock" || f == "pthread_mutex_unlock" {
                    println!("{:?}", expr_to_path(ctx, &args[0]));
                    self.mutexes
                        .push(normalize_path(&span_to_string(ctx, args[0].span)));
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
        let id_map: HashMap<_, _> = functions.iter().enumerate().map(|(i, f)| (i, *f)).collect();
        let inv_id_map: HashMap<_, _> = id_map.iter().map(|(i, f)| (*f, *i)).collect();
        let edges = self
            .call_graph
            .iter()
            .flat_map(|(caller, callees)| {
                callees.iter().map(|callee| {
                    (
                        Id::new(*inv_id_map.get(caller).unwrap()),
                        Id::new(*inv_id_map.get(callee).unwrap()),
                    )
                })
            })
            .collect();
        let sccs = compute_sccs(functions.len(), edges);
        let mut component_elems: HashMap<_, Vec<_>> = HashMap::new();
        for i in 0..(functions.len()) {
            let scc = sccs.scc(Id::new(i));
            let f = *id_map.get(&i).unwrap();
            component_elems.entry(scc).or_default().push(f);
        }
        let component_elems = component_elems;
        let mut component_graph: HashMap<_, _> = sccs
            .all_sccs()
            .map(|node| (node, sccs.successors(node).to_vec()))
            .collect();
        println!("{:#?}", self.call_graph);
        println!("{:#?}", component_elems);
        println!("{:?}", component_graph);

        self.mutexes.sort();
        self.mutexes.dedup();
        let mutexes: HashMap<_, _> = self
            .mutexes
            .iter()
            .enumerate()
            .map(|(i, s)| (s.clone(), i))
            .collect();
        let inv_mutexes: HashMap<_, _> = self
            .mutexes
            .iter()
            .enumerate()
            .map(|(i, s)| (i, s.clone()))
            .collect();
        let mut function_mutex_map: HashMap<DefId, (BitSet<Id>, BitSet<Id>)> = HashMap::new();
        while !component_graph.is_empty() {
            let leaves: HashSet<_> = component_graph
                .drain_filter(|_, succs| succs.is_empty())
                .map(|(node, _)| node)
                .collect();
            for succs in component_graph.values_mut() {
                succs.retain(|n| !leaves.contains(n));
            }
            for node in leaves {
                let funcs = component_elems.get(&node).unwrap();
                assert_eq!(funcs.len(), 1);
                let def_id = &funcs[0];
                assert!(!self.call_graph.get(def_id).unwrap().contains(def_id));
                let tcx = ctx.tcx;
                let body = tcx.optimized_mir(def_id);
                let ana_ctx = AnalysisContext::new(&mutexes, &function_mutex_map, body, ctx);
                let analysis = LiveGuards::new(ana_ctx);
                let mut results = analysis
                    .into_engine(tcx, body)
                    .iterate_to_fixpoint()
                    .into_results_cursor(body);
                results.seek_to_block_start(BasicBlock::from_usize(0));
                let start = results.get().clone();
                let analysis = AvailableGuards::new(ana_ctx, &start);
                let mut results = analysis
                    .into_engine(tcx, body)
                    .iterate_to_fixpoint()
                    .into_results_cursor(body);
                results.seek_to_block_end(BasicBlock::from_usize(body.basic_blocks().len() - 1));
                let end = results.get().0.clone();
                for (block, bbd) in body.basic_blocks().iter_enumerated() {
                    let tm = some_or!(&bbd.terminator, continue);
                    let (func, _) = some_or!(get_function_call(ctx, body, tm), continue);
                    let statement_index = bbd.statements.len();
                    let location = Location {
                        block,
                        statement_index,
                    };
                    results.seek_before_primary_effect(location);
                    let v = results.get().0.clone();
                    println!("{:?} {:?} {:?}", def_id, func, v);
                }
                function_mutex_map.insert(*def_id, (start, end));
            }
        }
        let mut res: Vec<_> = function_mutex_map.iter().collect();
        res.sort_by_key(|(def_id, _)| *def_id);
        for (def_id, (start, end)) in &res {
            let start: Vec<_> = start
                .iter()
                .map(|i| inv_mutexes.get(&i.index()).unwrap())
                .collect();
            let end: Vec<_> = end
                .iter()
                .map(|i| inv_mutexes.get(&i.index()).unwrap())
                .collect();
            println!("{:?} {:?} {:?}", def_id, start, end);
        }
    }
}
