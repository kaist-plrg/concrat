use std::collections::HashMap;

use rustc_index::{bit_set::BitSet, vec::Idx};
use rustc_lint::LateContext;
use rustc_middle::mir::{self, BasicBlock, Body, Location, Terminator};
use rustc_mir_dataflow::{
    lattice::Dual, Analysis, AnalysisDomain, Backward, CallReturnPlaces, Forward, GenKill,
    GenKillAnalysis, ResultsCursor,
};
use rustc_span::def_id::DefId;

use super::get_function_call;
use crate::util::Id;

#[allow(missing_debug_implementations)]
#[derive(Clone, Copy)]
pub struct AnalysisContext<'a, 'tcx> {
    mutexes: &'a HashMap<String, usize>,
    function_mutex_map: &'a HashMap<DefId, (BitSet<Id>, BitSet<Id>)>,
    body: &'a Body<'tcx>,
    ctx: &'a LateContext<'tcx>,
}

impl<'a, 'tcx> AnalysisContext<'a, 'tcx> {
    pub fn new(
        mutexes: &'a HashMap<String, usize>,
        function_mutex_map: &'a HashMap<DefId, (BitSet<Id>, BitSet<Id>)>,
        body: &'a Body<'tcx>,
        ctx: &'a LateContext<'tcx>,
    ) -> Self {
        Self {
            mutexes,
            function_mutex_map,
            body,
            ctx,
        }
    }
}

pub fn live_guards<'a, 'tcx>(
    ctx: AnalysisContext<'a, 'tcx>,
) -> ResultsCursor<'a, 'tcx, LiveGuards<'a, 'tcx>> {
    LiveGuards { ctx }
        .into_engine(ctx.ctx.tcx, ctx.body)
        .iterate_to_fixpoint()
        .into_results_cursor(ctx.body)
}

#[allow(missing_debug_implementations)]
pub struct LiveGuards<'a, 'tcx> {
    ctx: AnalysisContext<'a, 'tcx>,
}

impl<'tcx> AnalysisDomain<'tcx> for LiveGuards<'_, '_> {
    type Direction = Backward;
    type Domain = BitSet<Id>;

    const NAME: &'static str = "live guards";

    fn bottom_value(&self, _: &Body<'tcx>) -> Self::Domain {
        BitSet::new_empty(self.ctx.mutexes.len())
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
        if let Some((f, args)) = get_function_call(self.ctx.ctx, self.ctx.body, terminator) {
            match self.ctx.ctx.tcx.def_path_str(f).as_str() {
                "main::pthread_mutex_lock" => {
                    let idx = *self.ctx.mutexes.get(&args[0]).unwrap();
                    trans.kill(Id::new(idx));
                }
                "main::pthread_mutex_unlock" => {
                    let idx = *self.ctx.mutexes.get(&args[0]).unwrap();
                    trans.gen(Id::new(idx));
                }
                _ => (),
            }
            if let Some((start, end)) = self.ctx.function_mutex_map.get(&f) {
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

pub fn available_guards<'a, 'tcx>(
    ctx: AnalysisContext<'a, 'tcx>,
    start: &'a BitSet<Id>,
) -> ResultsCursor<'a, 'tcx, AvailableGuards<'a, 'tcx>> {
    AvailableGuards { ctx, start }
        .into_engine(ctx.ctx.tcx, ctx.body)
        .iterate_to_fixpoint()
        .into_results_cursor(ctx.body)
}

#[allow(missing_debug_implementations)]
pub struct AvailableGuards<'a, 'tcx> {
    ctx: AnalysisContext<'a, 'tcx>,
    start: &'a BitSet<Id>,
}

impl<'tcx> AnalysisDomain<'tcx> for AvailableGuards<'_, '_> {
    type Direction = Forward;
    type Domain = Dual<BitSet<Id>>;

    const NAME: &'static str = "available guards";

    fn bottom_value(&self, _: &Body<'tcx>) -> Self::Domain {
        Dual(BitSet::new_filled(self.ctx.mutexes.len()))
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
        if let Some((f, args)) = get_function_call(self.ctx.ctx, self.ctx.body, terminator) {
            match self.ctx.ctx.tcx.def_path_str(f).as_str() {
                "main::pthread_mutex_lock" => {
                    let idx = *self.ctx.mutexes.get(&args[0]).unwrap();
                    trans.gen(Id::new(idx));
                }
                "main::pthread_mutex_unlock" => {
                    let idx = *self.ctx.mutexes.get(&args[0]).unwrap();
                    trans.kill(Id::new(idx));
                }
                _ => (),
            }
            if let Some((start, end)) = self.ctx.function_mutex_map.get(&f) {
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
