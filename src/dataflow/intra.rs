use std::collections::{BTreeMap, BTreeSet};

use etrace::some_or;
use rustc_lint::LateContext;
use rustc_middle::mir::{self, BasicBlock, Body, Location, Terminator};
use rustc_mir_dataflow::{
    Analysis, AnalysisDomain, Backward, CallReturnPlaces, Forward, GenKill, Results,
};
use rustc_span::{def_id::DefId, Span};

use super::{
    domain::{HasBottom, MayMutexSet, MustMutexSet},
    get_function_call, Arg, FunctionCodeSummary, FunctionSummary,
};
use crate::util::ExprPath;

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

    fn terminator_effect<T>(&self, trans: &mut T, terminator: &Terminator<'_>, forward: bool)
    where T: GenKill<ExprPath> + HasBottom {
        let f = some_or!(get_function_call(terminator), return);
        let args = some_or!(self.calls.get(&terminator.source_info.span), return);
        match self.ctx.tcx.def_path_str(f).as_str() {
            "main::pthread_mutex_lock"
            | "main::pthread_mutex_trylock"
            | "main::pthread_spin_lock"
            | "main::pthread_spin_trylock" => {
                let mutex = args[0].path.clone().unwrap();
                if forward {
                    trans.gen(mutex);
                } else {
                    trans.kill(mutex);
                }
            }
            "main::pthread_mutex_unlock" | "main::pthread_spin_unlock" => {
                let mutex = args[0].path.clone().unwrap();
                if forward {
                    trans.kill(mutex);
                } else {
                    trans.gen(mutex);
                }
            }
            "main::pthread_cond_wait" | "main::pthread_cond_timedwait" => {
                let mutex = args[1].path.clone().unwrap();
                trans.gen(mutex);
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
            let start = entry_mutex.0.iter().map(|m| {
                m.clone()
                    .param_to_arg_aliasing(params, args)
                    .into_ok_or_err()
            });
            let end = ret_mutex
                .clone()
                .map(|m| m.param_to_arg_aliasing(params, args).into_ok_or_err());
            if forward {
                trans.kill_all(start);
                if let MustMutexSet::Set(end) = end {
                    trans.gen_all(end);
                } else {
                    *trans = <T as HasBottom>::bottom();
                }
            } else {
                if let MustMutexSet::Set(end) = end {
                    trans.kill_all(end);
                } else {
                    *trans = <T as HasBottom>::bottom();
                }
                trans.gen_all(start);
            }
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
    type Domain = MayMutexSet;

    const NAME: &'static str = "live guards";

    fn bottom_value(&self, _: &Body<'_>) -> Self::Domain {
        MayMutexSet::bottom()
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
        self.ctx.terminator_effect(state, terminator, false);
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
    start: BTreeSet<ExprPath>,
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
    start: BTreeSet<ExprPath>,
}

impl AnalysisDomain<'_> for AvailableGuards<'_, '_> {
    type Direction = Forward;
    type Domain = MustMutexSet;

    const NAME: &'static str = "available guards";

    fn bottom_value(&self, _: &Body<'_>) -> Self::Domain {
        MustMutexSet::bottom()
    }

    fn initialize_start_block(&self, _: &Body<'_>, state: &mut Self::Domain) {
        *state = MustMutexSet::new(self.start.clone());
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
        self.ctx.terminator_effect(state, terminator, true);
    }

    fn apply_call_return_effect(
        &self,
        _state: &mut Self::Domain,
        _block: BasicBlock,
        _return_places: CallReturnPlaces<'_, '_>,
    ) {
    }
}
