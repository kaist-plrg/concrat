use etrace::some_or;
use rustc_index::bit_set::BitSet;
use rustc_middle::mir::{Location, Statement, Terminator, TerminatorKind};
use rustc_mir_dataflow::{lattice::Dual, ResultsVisitor};
use rustc_span::{def_id::DefId, Span};

use crate::{dataflow::get_function_call, util::Id};

#[derive(Default, Debug)]
pub struct Visitor {
    pub return_state: Option<BitSet<Id>>,
    pub propagation: Vec<(DefId, BitSet<Id>)>,
    pub span_mutex: Vec<(Span, BitSet<Id>)>,
}

impl<'mir, 'tcx> ResultsVisitor<'mir, 'tcx> for Visitor {
    type FlowState = Dual<BitSet<Id>>;

    fn visit_terminator_before_primary_effect(
        &mut self,
        state: &Self::FlowState,
        terminator: &'mir Terminator<'tcx>,
        _location: Location,
    ) {
        match &terminator.kind {
            TerminatorKind::Call { .. } | TerminatorKind::Return => {
                let span = terminator.source_info.span;
                self.span_mutex.push((span, state.0.clone()));
            }
            _ => (),
        }

        if matches!(&terminator.kind, TerminatorKind::Return) {
            self.return_state = Some(state.0.clone());
        }

        let func = some_or!(get_function_call(terminator), return);
        self.propagation.push((func, state.0.clone()));
    }

    fn visit_statement_before_primary_effect(
        &mut self,
        state: &Self::FlowState,
        statement: &'mir Statement<'tcx>,
        _location: Location,
    ) {
        let span = statement.source_info.span;
        self.span_mutex.push((span, state.0.clone()));
    }
}
