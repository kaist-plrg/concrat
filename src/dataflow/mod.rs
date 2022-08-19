use std::collections::BTreeMap;

use rustc_hir::{Expr, HirId};
use rustc_index::bit_set::BitSet;
use rustc_lint::LateContext;
use rustc_middle::mir::{Operand, Terminator, TerminatorKind};
use rustc_span::{def_id::DefId, Span};

pub mod intra;
pub mod pass;
pub mod visitor;

pub use pass::run;

use crate::util::{
    expr_to_path, span_to_string, type_of, type_to_string, unwrap_ptr_from_type, ExprPath, Id,
};

#[derive(Debug)]
pub struct Arg {
    path: Option<ExprPath>,
    typ: String,
    #[allow(unused)]
    expr: String,
    #[allow(unused)]
    hir_id: HirId,
}

impl Arg {
    fn new<'tcx>(ctx: &LateContext<'tcx>, expr: &Expr<'tcx>) -> Self {
        let path = expr_to_path(ctx, expr);
        let typ = type_to_string(unwrap_ptr_from_type(type_of(ctx, expr.hir_id)));
        let hir_id = expr.hir_id;
        let expr = span_to_string(ctx, expr.span);
        Self {
            expr,
            typ,
            path,
            hir_id,
        }
    }
}

#[derive(Debug)]
pub struct FunctionSummary {
    pub entry_mutex: BitSet<Id>,
    pub ret_mutex: BitSet<Id>,
    pub propagation_mutex: BitSet<Id>,
    pub propagation: BTreeMap<DefId, BitSet<Id>>,
    pub propagation_raw: Vec<(DefId, BitSet<Id>)>,
    pub access: Vec<(ExprPath, BitSet<Id>, bool)>,
    pub span_mutex: Vec<(Span, BitSet<Id>)>,
}

impl FunctionSummary {
    pub fn new(
        entry_mutex: BitSet<Id>,
        ret_mutex: BitSet<Id>,
        propagation_raw: Vec<(DefId, BitSet<Id>)>,
        access: Vec<(ExprPath, BitSet<Id>, bool)>,
        span_mutex: Vec<(Span, BitSet<Id>)>,
    ) -> Self {
        let len = entry_mutex.domain_size();
        let mut propagation = BTreeMap::new();
        for (def_id, v) in &propagation_raw {
            let ov = propagation
                .entry(*def_id)
                .or_insert_with(|| BitSet::new_filled(len));
            ov.intersect(v);
        }
        Self {
            entry_mutex,
            ret_mutex,
            propagation_mutex: BitSet::new_empty(len),
            propagation,
            propagation_raw,
            access,
            span_mutex,
        }
    }

    pub fn mutex_only(entry_mutex: BitSet<Id>, ret_mutex: BitSet<Id>) -> Self {
        Self::new(entry_mutex, ret_mutex, vec![], vec![], vec![])
    }
}

fn get_function_call(tm: &Terminator<'_>) -> Option<DefId> {
    if let TerminatorKind::Call {
        func: Operand::Constant(func),
        ..
    } = &tm.kind
    {
        if let rustc_middle::ty::TyKind::FnDef(def_id, _) = func.literal.ty().kind() {
            Some(*def_id)
        } else {
            None
        }
    } else {
        None
    }
}
