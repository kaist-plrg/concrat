use std::collections::{HashMap, HashSet};

use rustc_hir::{
    def::{DefKind, Res},
    Expr, ExprKind, Item, ItemKind,
};
use rustc_index::{bit_set::BitSet, vec::Idx};
use rustc_lint::{LateContext, LateLintPass, LintPass};
use rustc_middle::mir::{self, Body, Location, Operand, Terminator, TerminatorKind};
use rustc_mir_dataflow::{
    fmt::DebugWithContext, Analysis, AnalysisDomain, Backward, CallReturnPlaces, GenKill,
    GenKillAnalysis,
};
use rustc_span::def_id::DefId;

use crate::{
    callback::{compile_with, LatePass},
    graph::compute_sccs,
    util::{current_function, resolve_path, span_to_string},
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
                        self.call_graph.get_mut(&curr).unwrap().insert(def_id);
                    }
                }
                let f = span_to_string(ctx, f.span);
                if f == "pthread_mutex_lock" || f == "pthread_mutex_unlock" {
                    self.mutexes.push(span_to_string(ctx, args[0].span));
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
        println!("{:#?}", self.call_graph);
        let id_map: HashMap<_, _> = functions.iter().enumerate().map(|(i, f)| (i, *f)).collect();
        let inv_id_map: HashMap<_, _> = id_map.iter().map(|(i, f)| (*f, *i)).collect();
        let edges = self
            .call_graph
            .iter()
            .flat_map(|(caller, callees)| {
                callees.iter().map(|callee| {
                    (
                        Id(*inv_id_map.get(caller).unwrap()),
                        Id(*inv_id_map.get(callee).unwrap()),
                    )
                })
            })
            .collect();
        let sccs = compute_sccs(functions.len(), edges);
        let mut component_elems: HashMap<_, Vec<_>> = HashMap::new();
        for i in 0..(functions.len()) {
            let scc = sccs.scc(Id(i));
            let f = *id_map.get(&i).unwrap();
            component_elems.entry(scc).or_default().push(f);
        }
        let component_graph: HashMap<_, _> = sccs
            .all_sccs()
            .map(|node| (node, sccs.successors(node).to_vec()))
            .collect();
        println!("{:#?}", component_elems);
        println!("{:?}", component_graph);
        if !self.call_graph.is_empty() {
            return;
        }
        self.mutexes.sort();
        self.mutexes.dedup();
        let mutexes: HashMap<_, _> = self
            .mutexes
            .iter()
            .enumerate()
            .map(|(i, s)| (s.clone(), i))
            .collect();
        for def_id in functions {
            println!("{:?}", def_id);
            let tcx = ctx.tcx;
            let body = tcx.optimized_mir(def_id);
            let mut results = LiveGuards {
                mutexes: mutexes.clone(),
                ctx,
                body,
            }
            .into_engine(tcx, body)
            .iterate_to_fixpoint()
            .into_results_cursor(body);
            for (bb, bbd) in body.basic_blocks().iter_enumerated() {
                results.seek_to_block_start(bb);
                let state = results.get();
                println!("{:?}", bb);
                println!("{:?}", state);
                for stmt in &bbd.statements {
                    println!("{:?}", stmt);
                }
                if let Some(tm) = &bbd.terminator {
                    println!("{:?}", tm);
                }
            }
            println!();
        }
    }
}

fn get_function_call(
    ctx: &LateContext<'_>,
    body: &Body<'_>,
    tm: &Terminator<'_>,
) -> Option<(String, Vec<String>)> {
    if let TerminatorKind::Call { func, args, .. } = &tm.kind {
        let func = if let Operand::Constant(func) = func {
            span_to_string(ctx, func.span)
        } else {
            return None;
        };
        let mut arguments = vec![];
        for arg in args {
            if let Operand::Move(arg) = arg {
                arguments.push(span_to_string(
                    ctx,
                    body.local_decls[arg.local].source_info.span,
                ));
            }
        }
        Some((func, arguments))
    } else {
        None
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, PartialOrd, Ord)]
struct Id(usize);

impl Idx for Id {
    fn new(idx: usize) -> Self {
        Self(idx)
    }

    fn index(self) -> usize {
        self.0
    }
}

impl<T> DebugWithContext<T> for Id {}

struct LiveGuards<'a, 'b, 'tcx> {
    mutexes: HashMap<String, usize>,
    body: &'a Body<'tcx>,
    ctx: &'b LateContext<'tcx>,
}

impl<'tcx> AnalysisDomain<'tcx> for LiveGuards<'_, '_, '_> {
    type Direction = Backward;
    type Domain = BitSet<Id>;

    const NAME: &'static str = "live guards";

    fn bottom_value(&self, _: &Body<'tcx>) -> Self::Domain {
        BitSet::new_empty(self.mutexes.len())
    }

    fn initialize_start_block(&self, _: &Body<'tcx>, _: &mut Self::Domain) {}
}

impl<'tcx> GenKillAnalysis<'tcx> for LiveGuards<'_, '_, '_> {
    type Idx = Id;

    fn statement_effect(
        &self,
        _trans: &mut impl GenKill<Self::Idx>,
        _statement: &mir::Statement<'tcx>,
        _location: Location,
    ) {
    }

    fn terminator_effect(
        &self,
        trans: &mut impl GenKill<Self::Idx>,
        terminator: &Terminator<'tcx>,
        _location: Location,
    ) {
        if let Some((f, args)) = get_function_call(self.ctx, self.body, terminator) {
            match f.as_str() {
                "pthread_mutex_lock" => {
                    let idx = *self.mutexes.get(&args[0]).unwrap();
                    trans.kill(Id(idx));
                }
                "pthread_mutex_unlock" => {
                    let idx = *self.mutexes.get(&args[0]).unwrap();
                    trans.gen(Id(idx));
                }
                _ => (),
            }
        }
    }

    fn call_return_effect(
        &self,
        _trans: &mut impl GenKill<Self::Idx>,
        _block: mir::BasicBlock,
        _return_places: CallReturnPlaces<'_, 'tcx>,
    ) {
    }
}
