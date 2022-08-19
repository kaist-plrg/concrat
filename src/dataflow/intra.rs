use std::collections::BTreeMap;

use etrace::some_or;
use rustc_index::{bit_set::BitSet, vec::Idx};
use rustc_lint::LateContext;
use rustc_middle::mir::{self, BasicBlock, Body, Location, Terminator};
use rustc_mir_dataflow::{
    lattice::Dual, Analysis, AnalysisDomain, Backward, CallReturnPlaces, Forward, GenKill,
    GenKillAnalysis, Results,
};
use rustc_span::{def_id::DefId, Span};

use super::{get_function_call, Arg, FunctionCodeSummary, FunctionSummary};
use crate::util::{ExprPath, Id};

#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct AnalysisContext<'a, 'tcx> {
    mutexes: &'a BTreeMap<ExprPath, usize>,
    inv_mutexes: &'a BTreeMap<usize, ExprPath>,
    function_mutex_map: &'a BTreeMap<DefId, FunctionSummary>,
    functions: &'a BTreeMap<DefId, FunctionCodeSummary>,
    calls: &'a BTreeMap<Span, Vec<Arg>>,
    body: &'a Body<'tcx>,
    ctx: &'a LateContext<'tcx>,
}

impl<'a, 'tcx> AnalysisContext<'a, 'tcx> {
    pub fn new(
        mutexes: &'a BTreeMap<ExprPath, usize>,
        inv_mutexes: &'a BTreeMap<usize, ExprPath>,
        function_mutex_map: &'a BTreeMap<DefId, FunctionSummary>,
        functions: &'a BTreeMap<DefId, FunctionCodeSummary>,
        calls: &'a BTreeMap<Span, Vec<Arg>>,
        body: &'a Body<'tcx>,
        ctx: &'a LateContext<'tcx>,
    ) -> Self {
        Self {
            mutexes,
            inv_mutexes,
            function_mutex_map,
            functions,
            calls,
            body,
            ctx,
        }
    }

    fn alias_id(&self, id: Id, func: &DefId, args: &[Arg]) -> Id {
        let mut m = self.inv_mutexes.get(&id.index()).unwrap().clone();
        if m.is_variable() {
            return id;
        }
        let params = &self.functions.get(func).unwrap().params;
        let (i, _) = some_or!(
            params.iter().enumerate().find(|(_, (p, _))| &m.base == p),
            return id
        );
        let arg = some_or!(args[i].path.as_ref(), return id);
        m.set_base(arg);
        Id::new(*self.mutexes.get(&m).unwrap())
    }

    fn terminator_effect(
        &self,
        trans: &mut impl GenKill<Id>,
        terminator: &Terminator<'_>,
        forward: bool,
    ) {
        let f = some_or!(get_function_call(terminator), return);
        let args = some_or!(self.calls.get(&terminator.source_info.span), return);
        match self.ctx.tcx.def_path_str(f).as_str() {
            "main::pthread_mutex_lock"
            | "main::pthread_mutex_trylock"
            | "main::pthread_spin_lock"
            | "main::pthread_spin_trylock" => {
                let idx = *self.mutexes.get(args[0].path.as_ref().unwrap()).unwrap();
                if forward {
                    trans.gen(Id::new(idx));
                } else {
                    trans.kill(Id::new(idx));
                }
            }
            "main::pthread_mutex_unlock" | "main::pthread_spin_unlock" => {
                let idx = *self.mutexes.get(args[0].path.as_ref().unwrap()).unwrap();
                if forward {
                    trans.kill(Id::new(idx));
                } else {
                    trans.gen(Id::new(idx));
                }
            }
            "main::pthread_cond_wait" | "main::pthread_cond_timedwait" => {
                let idx = *self.mutexes.get(args[1].path.as_ref().unwrap()).unwrap();
                trans.gen(Id::new(idx));
            }
            _ => (),
        }
        if let Some(summary) = self.function_mutex_map.get(&f) {
            let FunctionSummary {
                entry_mutex,
                ret_mutex,
                ..
            } = summary;
            let start = entry_mutex.iter().map(|i| self.alias_id(i, &f, args));
            let end = ret_mutex.iter().map(|i| self.alias_id(i, &f, args));
            if forward {
                trans.kill_all(start);
                trans.gen_all(end);
            } else {
                trans.kill_all(end);
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
    type Domain = BitSet<Id>;

    const NAME: &'static str = "live guards";

    fn bottom_value(&self, _: &Body<'_>) -> Self::Domain {
        BitSet::new_empty(self.ctx.mutexes.len())
    }

    fn initialize_start_block(&self, _: &Body<'_>, _: &mut Self::Domain) {}
}

impl GenKillAnalysis<'_> for LiveGuards<'_, '_> {
    type Idx = Id;

    fn statement_effect(
        &self,
        _trans: &mut impl GenKill<Self::Idx>,
        _statement: &mir::Statement<'_>,
        _location: Location,
    ) {
    }

    fn terminator_effect(
        &self,
        trans: &mut impl GenKill<Self::Idx>,
        terminator: &Terminator<'_>,
        _location: Location,
    ) {
        self.ctx.terminator_effect(trans, terminator, false);
    }

    fn call_return_effect(
        &self,
        _trans: &mut impl GenKill<Self::Idx>,
        _block: BasicBlock,
        _return_places: CallReturnPlaces<'_, '_>,
    ) {
    }
}

pub fn available_guards<'a, 'tcx>(
    ctx: AnalysisContext<'a, 'tcx>,
    start: BitSet<Id>,
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
    start: BitSet<Id>,
}

impl AnalysisDomain<'_> for AvailableGuards<'_, '_> {
    type Direction = Forward;
    type Domain = Dual<BitSet<Id>>;

    const NAME: &'static str = "available guards";

    fn bottom_value(&self, _: &Body<'_>) -> Self::Domain {
        Dual(BitSet::new_filled(self.ctx.mutexes.len()))
    }

    fn initialize_start_block(&self, _: &Body<'_>, state: &mut Self::Domain) {
        state.0.clear();
        for i in self.start.iter() {
            state.0.insert(i);
        }
    }
}

impl GenKillAnalysis<'_> for AvailableGuards<'_, '_> {
    type Idx = Id;

    fn statement_effect(
        &self,
        _trans: &mut impl GenKill<Self::Idx>,
        _statement: &mir::Statement<'_>,
        _location: Location,
    ) {
    }

    fn terminator_effect(
        &self,
        trans: &mut impl GenKill<Self::Idx>,
        terminator: &Terminator<'_>,
        _location: Location,
    ) {
        self.ctx.terminator_effect(trans, terminator, true);
    }

    fn call_return_effect(
        &self,
        _trans: &mut impl GenKill<Self::Idx>,
        _block: BasicBlock,
        _return_places: CallReturnPlaces<'_, '_>,
    ) {
    }
}
