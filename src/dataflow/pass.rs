use std::collections::{HashMap, HashSet};

use etrace::some_or;
use rustc_hir::{
    def::{DefKind, Res},
    Expr, ExprKind, Item, ItemKind,
};
use rustc_index::{bit_set::BitSet, vec::Idx};
use rustc_lint::{LateContext, LateLintPass, LintPass};
use rustc_middle::mir::{BasicBlock, Location};
use rustc_span::def_id::DefId;

use super::{
    get_function_call,
    intra::{available_guards, live_guards, AnalysisContext},
};
use crate::{
    callback::{compile_with, LatePass},
    graph::compute_sccs,
    util::{
        current_function, def_id_to_item_name, expr_to_path, join, resolve_path, span_to_string,
        type_as_string, Id,
    },
};

pub fn run(args: Vec<String>) {
    let exit_code = compile_with(args, vec![GlobalPass::new]);
    assert_eq!(exit_code, 0);
}

#[derive(Default)]
struct GlobalPass {
    mutexes: HashMap<DefId, HashSet<String>>,
    params: HashMap<DefId, Vec<(String, String)>>,
    args_per_type: HashMap<String, HashSet<String>>,
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

impl GlobalPass {
    fn possible_mutexes(&self) -> HashSet<String> {
        let mut mutexes: HashSet<_> = self.mutexes.values().flatten().cloned().collect();
        for (def_id, ms) in &self.mutexes {
            let params = self.params.get(def_id).unwrap();
            for m in ms {
                let i = some_or!(m.find('.'), continue);
                let x = &m[0..i];
                let y = &m[i..];
                let (_, t) = some_or!(params.iter().find(|(p, _)| p == x), continue);
                let args = some_or!(self.args_per_type.get(t), continue);
                for arg in args {
                    mutexes.insert(format!("{}{}", arg, y));
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

                    let args: Vec<_> = args
                        .iter()
                        .filter_map(|arg| {
                            let a = join(expr_to_path(ctx, arg)?, ".");
                            let t = type_as_string(ctx, arg.hir_id)
                                .replace("&mut ", "")
                                .replace("*mut ", "")
                                .replace("*const ", "");
                            Some((a, t))
                        })
                        .collect();

                    let f = span_to_string(ctx, f.span);
                    if f == "pthread_mutex_lock" || f == "pthread_mutex_unlock" {
                        self.mutexes
                            .entry(curr)
                            .or_default()
                            .insert(args[0].0.clone());
                    }

                    for (a, t) in args {
                        self.args_per_type.entry(t).or_default().insert(a);
                    }
                }
            }
            _ => (),
        }
    }

    fn check_crate_post(&mut self, ctx: &LateContext<'tcx>) {
        println!("{:#?}", self.params);
        println!("{:#?}", self.args_per_type);
        let functions: HashSet<_> = self.call_graph.iter().map(|(f, _)| f).cloned().collect();
        for callees in self.call_graph.values_mut() {
            callees.retain(|f| functions.contains(f));
        }
        let (mut component_graph, component_elems) = compute_sccs(&self.call_graph);
        println!("{:#?}", self.call_graph);
        println!("{:#?}", component_elems);
        println!("{:?}", component_graph);

        let mutexes: HashSet<_> = self.possible_mutexes();
        println!("{:?}", mutexes);
        let mutexes: HashMap<_, _> = mutexes
            .iter()
            .enumerate()
            .map(|(i, s)| (s.clone(), i))
            .collect();
        let inv_mutexes: HashMap<_, _> = mutexes.iter().map(|(s, i)| (*i, s.clone())).collect();

        let mut function_mutex_map: HashMap<DefId, (BitSet<Id>, BitSet<Id>)> = HashMap::new();
        let mut call_mutex_map: HashMap<(DefId, DefId), BitSet<Id>> = HashMap::new();
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
                let def_id = funcs.iter().next().unwrap();
                assert!(!self.call_graph.get(def_id).unwrap().contains(def_id));

                let tcx = ctx.tcx;
                let body = tcx.optimized_mir(def_id);
                let ana_ctx = AnalysisContext::new(
                    &mutexes,
                    &inv_mutexes,
                    &function_mutex_map,
                    &self.params,
                    body,
                    ctx,
                );

                let mut results = live_guards(ana_ctx);
                results.seek_to_block_start(BasicBlock::from_usize(0));
                let start = results.get().clone();

                let mut results = available_guards(ana_ctx, &start);
                results.seek_to_block_end(BasicBlock::from_usize(body.basic_blocks().len() - 1));
                let end = results.get().0.clone();

                let mut func_call_mutex_map: HashMap<_, Vec<_>> = HashMap::new();
                for (block, bbd) in body.basic_blocks().iter_enumerated() {
                    let tm = some_or!(&bbd.terminator, continue);
                    let (func, _) = some_or!(get_function_call(ctx, body, tm), continue);
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

                function_mutex_map.insert(*def_id, (start, end));
            }
        }

        let i2m = |i: Id| inv_mutexes.get(&i.index()).unwrap().clone();

        let mut res: Vec<_> = function_mutex_map.iter().collect();
        res.sort_by_key(|(def_id, _)| *def_id);
        for (def_id, (start, end)) in res {
            let f = def_id_to_item_name(ctx.tcx, *def_id);
            let start: Vec<_> = start.iter().map(i2m).collect();
            let end: Vec<_> = end.iter().map(i2m).collect();
            println!("{} {:?} {:?}", f, start, end);
        }

        for ((caller, callee), v) in &call_mutex_map {
            let caller = def_id_to_item_name(ctx.tcx, *caller);
            let callee = def_id_to_item_name(ctx.tcx, *callee);
            let v: Vec<_> = v.iter().map(i2m).collect();
            println!("{} {} {:?}", caller, callee, v);
        }
    }
}
