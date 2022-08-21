use etrace::some_or;
use rustc_middle::mir::{Location, Statement, Terminator, TerminatorKind};
use rustc_mir_dataflow::ResultsVisitor;
use rustc_span::{def_id::DefId, Span};

use super::domain::MustMutexSet;
use crate::dataflow::get_function_call;

#[derive(Default, Debug)]
pub struct Visitor {
    pub return_state: Option<MustMutexSet>,
    pub propagation: Vec<(DefId, MustMutexSet)>,
    pub span_mutex: Vec<(Span, MustMutexSet)>,
}

impl<'mir, 'tcx> ResultsVisitor<'mir, 'tcx> for Visitor {
    type FlowState = MustMutexSet;

    fn visit_terminator_before_primary_effect(
        &mut self,
        state: &Self::FlowState,
        terminator: &'mir Terminator<'tcx>,
        _location: Location,
    ) {
        match &terminator.kind {
            TerminatorKind::Call { .. } | TerminatorKind::Return => {
                let span = terminator.source_info.span;
                self.span_mutex.push((span, state.clone()));
            }
            _ => (),
        }

        if matches!(&terminator.kind, TerminatorKind::Return) {
            self.return_state = Some(state.clone());
        }

        let func = some_or!(get_function_call(terminator), return);
        self.propagation.push((func, state.clone()));
    }

    fn visit_statement_before_primary_effect(
        &mut self,
        state: &Self::FlowState,
        statement: &'mir Statement<'tcx>,
        _location: Location,
    ) {
        let span = statement.source_info.span;
        self.span_mutex.push((span, state.clone()));
    }
}
