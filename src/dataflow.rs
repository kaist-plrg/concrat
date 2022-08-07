use std::collections::{HashMap, HashSet};

use etrace::some_or;
use rustc_hir::{
    def::{DefKind, Res},
    Expr, ExprKind, Item, ItemKind,
};
use rustc_index::{bit_set::BitSet, vec::Idx};
use rustc_lint::{LateContext, LateLintPass, LintPass};
use rustc_middle::mir::{self, BasicBlock, Body, Location, Operand, Terminator, TerminatorKind};
use rustc_mir_dataflow::{
    fmt::DebugWithContext, lattice::Dual, Analysis, AnalysisDomain, Backward, CallReturnPlaces,
    Forward, GenKill, GenKillAnalysis,
};
use rustc_span::def_id::DefId;

use crate::{
    callback::{compile_with, LatePass},
    graph::compute_sccs,
    util::{current_function, normalize_path, resolve_path, span_to_string},
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
                    println!("{:?}", crate::util::expr_to_path(ctx, &args[0]));
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
                let analysis = LiveGuards {
                    mutexes: &mutexes,
                    function_mutex_map: &function_mutex_map,
                    ctx,
                    body,
                };
                let mut results = analysis
                    .into_engine(tcx, body)
                    .iterate_to_fixpoint()
                    .into_results_cursor(body);
                results.seek_to_block_start(BasicBlock::from_usize(0));
                let start = results.get().clone();
                let analysis = AvailableGuards {
                    mutexes: &mutexes,
                    function_mutex_map: &function_mutex_map,
                    start: &start,
                    ctx,
                    body,
                };
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

fn get_function_call<'tcx>(
    ctx: &LateContext<'tcx>,
    body: &Body<'tcx>,
    tm: &Terminator<'tcx>,
) -> Option<(DefId, Vec<String>)> {
    if let TerminatorKind::Call { func, args, .. } = &tm.kind {
        let func = if let Operand::Constant(func) = func {
            if let rustc_middle::ty::TyKind::FnDef(def_id, _) = func.literal.ty().kind() {
                *def_id
            } else {
                return None;
            }
        } else {
            return None;
        };
        let mut arguments = vec![];
        for arg in args {
            if let Operand::Move(arg) = arg {
                arguments.push(normalize_path(&span_to_string(
                    ctx,
                    body.local_decls[arg.local].source_info.span,
                )));
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

struct LiveGuards<'a, 'tcx> {
    mutexes: &'a HashMap<String, usize>,
    function_mutex_map: &'a HashMap<DefId, (BitSet<Id>, BitSet<Id>)>,
    body: &'a Body<'tcx>,
    ctx: &'a LateContext<'tcx>,
}

impl<'tcx> AnalysisDomain<'tcx> for LiveGuards<'_, '_> {
    type Direction = Backward;
    type Domain = BitSet<Id>;

    const NAME: &'static str = "live guards";

    fn bottom_value(&self, _: &Body<'tcx>) -> Self::Domain {
        BitSet::new_empty(self.mutexes.len())
    }

    fn initialize_start_block(&self, _: &Body<'tcx>, _: &mut Self::Domain) {}
}

impl<'tcx> GenKillAnalysis<'tcx> for LiveGuards<'_, 'tcx> {
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
            match self.ctx.tcx.def_path_str(f).as_str() {
                "main::pthread_mutex_lock" => {
                    let idx = *self.mutexes.get(&args[0]).unwrap();
                    trans.kill(Id(idx));
                }
                "main::pthread_mutex_unlock" => {
                    let idx = *self.mutexes.get(&args[0]).unwrap();
                    trans.gen(Id(idx));
                }
                _ => (),
            }
            if let Some((start, end)) = self.function_mutex_map.get(&f) {
                trans.kill_all(end.iter());
                trans.gen_all(start.iter());
            }
        }
    }

    fn call_return_effect(
        &self,
        _trans: &mut impl GenKill<Self::Idx>,
        _block: BasicBlock,
        _return_places: CallReturnPlaces<'_, 'tcx>,
    ) {
    }
}

struct AvailableGuards<'a, 'tcx> {
    mutexes: &'a HashMap<String, usize>,
    function_mutex_map: &'a HashMap<DefId, (BitSet<Id>, BitSet<Id>)>,
    start: &'a BitSet<Id>,
    body: &'a Body<'tcx>,
    ctx: &'a LateContext<'tcx>,
}

impl<'tcx> AnalysisDomain<'tcx> for AvailableGuards<'_, '_> {
    type Direction = Forward;
    type Domain = Dual<BitSet<Id>>;

    const NAME: &'static str = "available guards";

    fn bottom_value(&self, _: &Body<'tcx>) -> Self::Domain {
        Dual(BitSet::new_filled(self.mutexes.len()))
    }

    fn initialize_start_block(&self, _: &Body<'tcx>, state: &mut Self::Domain) {
        state.0.clear();
        for i in self.start.iter() {
            state.0.insert(i);
        }
    }
}

impl<'tcx> GenKillAnalysis<'tcx> for AvailableGuards<'_, 'tcx> {
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
            match self.ctx.tcx.def_path_str(f).as_str() {
                "main::pthread_mutex_lock" => {
                    let idx = *self.mutexes.get(&args[0]).unwrap();
                    trans.gen(Id(idx));
                }
                "main::pthread_mutex_unlock" => {
                    let idx = *self.mutexes.get(&args[0]).unwrap();
                    trans.kill(Id(idx));
                }
                _ => (),
            }
            if let Some((start, end)) = self.function_mutex_map.get(&f) {
                trans.kill_all(start.iter());
                trans.gen_all(end.iter());
            }
        }
    }

    fn call_return_effect(
        &self,
        _trans: &mut impl GenKill<Self::Idx>,
        _block: BasicBlock,
        _return_places: CallReturnPlaces<'_, 'tcx>,
    ) {
    }
}
