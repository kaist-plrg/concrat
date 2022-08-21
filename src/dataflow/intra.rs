use std::collections::BTreeMap;

use etrace::some_or;
use rustc_lint::LateContext;
use rustc_middle::mir::{self, BasicBlock, Body, Location, Terminator};
use rustc_mir_dataflow::{Analysis, AnalysisDomain, Backward, CallReturnPlaces, Forward, Results};
use rustc_span::{def_id::DefId, Span};

use super::{
    domain::{Domain, MayMutexSetPair, MustMutexSetTriple},
    get_function_call, Arg, FunctionCodeSummary, FunctionSummary,
};

#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct AnalysisContext<'a, 'tcx> {
    function_mutex_map: &'a BTreeMap<DefId, FunctionSummary>,
    functions: &'a BTreeMap<DefId, FunctionCodeSummary>,
    calls: &'a BTreeMap<Span, Vec<Arg>>,
    body: &'a Body<'tcx>,
    ctx: &'a LateContext<'tcx>,
}

impl<'a, 'tcx> AnalysisContext<'a, 'tcx> {
    pub fn new(
        function_mutex_map: &'a BTreeMap<DefId, FunctionSummary>,
        functions: &'a BTreeMap<DefId, FunctionCodeSummary>,
        calls: &'a BTreeMap<Span, Vec<Arg>>,
        body: &'a Body<'tcx>,
        ctx: &'a LateContext<'tcx>,
    ) -> Self {
        Self {
            function_mutex_map,
            functions,
            calls,
            body,
            ctx,
        }
    }

    fn terminator_effect(&self, domain: &mut impl Domain, terminator: &Terminator<'_>) {
        let f = some_or!(get_function_call(terminator), return);
        let args = some_or!(self.calls.get(&terminator.source_info.span), return);
        let arg = |i: usize| args[i].path.clone().unwrap();
        match self.ctx.tcx.def_path_str(f).as_str() {
            "main::pthread_mutex_lock"
            | "main::pthread_mutex_trylock"
            | "main::pthread_spin_lock"
            | "main::pthread_spin_trylock" => {
                domain.lock(arg(0));
            }
            "main::pthread_mutex_unlock" | "main::pthread_spin_unlock" => {
                domain.unlock(arg(0));
            }
            "main::pthread_cond_wait" | "main::pthread_cond_timedwait" => {
                domain.wait(arg(1));
            }

            "main::pthread_rwlock_rdlock" | "main::pthread_rwlock_tryrdlock" => {
                domain.lock_rd(arg(0));
            }
            "main::pthread_rwlock_wrlock" | "main::pthread_rwlock_trywrlock" => {
                domain.lock_wr(arg(0));
            }
            "main::pthread_rwlock_unlock" => {
                domain.unlock_rw(arg(0));
            }

            _ => (),
        }
        if let Some(summary) = self.function_mutex_map.get(&f) {
            let FunctionSummary {
                entry_mutex,
                ret_mutex,
                ..
            } = summary;
            let params = &self.functions.get(&f).unwrap().params;
            let entry_mutex = entry_mutex
                .clone()
                .map(|m| m.param_to_arg_aliasing(params, args).into_ok_or_err());
            let ret_mutex = ret_mutex
                .clone()
                .map(|m| m.param_to_arg_aliasing(params, args).into_ok_or_err());
            domain.custom(entry_mutex, ret_mutex);
        }
    }
}

pub fn live_guards<'a, 'tcx>(
    ctx: AnalysisContext<'a, 'tcx>,
) -> Results<'tcx, LiveGuards<'a, 'tcx>> {
    let tcx = ctx.ctx.tcx;
    let body = ctx.body;
    LiveGuards { ctx }
        .into_engine(tcx, body)
        .iterate_to_fixpoint()
}

#[allow(missing_debug_implementations)]
pub struct LiveGuards<'a, 'tcx> {
    ctx: AnalysisContext<'a, 'tcx>,
}

impl AnalysisDomain<'_> for LiveGuards<'_, '_> {
    type Direction = Backward;
    type Domain = MayMutexSetPair;

    const NAME: &'static str = "live guards";

    fn bottom_value(&self, _: &Body<'_>) -> Self::Domain {
        MayMutexSetPair::bottom()
    }

    fn initialize_start_block(&self, _: &Body<'_>, _: &mut Self::Domain) {}
}

impl Analysis<'_> for LiveGuards<'_, '_> {
    fn apply_statement_effect(
        &self,
        _state: &mut Self::Domain,
        _statement: &mir::Statement<'_>,
        _location: Location,
    ) {
    }

    fn apply_terminator_effect(
        &self,
        state: &mut Self::Domain,
        terminator: &Terminator<'_>,
        _location: Location,
    ) {
        self.ctx.terminator_effect(state, terminator);
    }

    fn apply_call_return_effect(
        &self,
        _state: &mut Self::Domain,
        _block: BasicBlock,
        _return_places: CallReturnPlaces<'_, '_>,
    ) {
    }
}

pub fn available_guards<'a, 'tcx>(
    ctx: AnalysisContext<'a, 'tcx>,
    start: MayMutexSetPair,
) -> Results<'tcx, AvailableGuards<'a, 'tcx>> {
    let tcx = ctx.ctx.tcx;
    let body = ctx.body;
    AvailableGuards { ctx, start }
        .into_engine(tcx, body)
        .iterate_to_fixpoint()
}

#[allow(missing_debug_implementations)]
pub struct AvailableGuards<'a, 'tcx> {
    ctx: AnalysisContext<'a, 'tcx>,
    start: MayMutexSetPair,
}

impl AnalysisDomain<'_> for AvailableGuards<'_, '_> {
    type Direction = Forward;
    type Domain = MustMutexSetTriple;

    const NAME: &'static str = "available guards";

    fn bottom_value(&self, _: &Body<'_>) -> Self::Domain {
        MustMutexSetTriple::bottom()
    }

    fn initialize_start_block(&self, _: &Body<'_>, state: &mut Self::Domain) {
        *state = MustMutexSetTriple::new(self.start.clone());
    }
}

impl Analysis<'_> for AvailableGuards<'_, '_> {
    fn apply_statement_effect(
        &self,
        _state: &mut Self::Domain,
        _statement: &mir::Statement<'_>,
        _location: Location,
    ) {
    }

    fn apply_terminator_effect(
        &self,
        state: &mut Self::Domain,
        terminator: &Terminator<'_>,
        _location: Location,
    ) {
        self.ctx.terminator_effect(state, terminator);
    }

    fn apply_call_return_effect(
        &self,
        _state: &mut Self::Domain,
        _block: BasicBlock,
        _return_places: CallReturnPlaces<'_, '_>,
    ) {
    }
}
