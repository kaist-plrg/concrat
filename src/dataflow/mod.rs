use std::collections::HashMap;

use rustc_hir::{Expr, HirId};
use rustc_index::bit_set::BitSet;
use rustc_lint::LateContext;
use rustc_middle::mir::{Operand, Terminator, TerminatorKind};
use rustc_span::def_id::DefId;

pub mod intra;
pub mod pass;

pub use pass::run;

use crate::util::{
    expr_to_path, span_to_string, type_of, type_to_string, unwrap_ptr_from_type, ExprPath, Id,
};

#[derive(Debug)]
pub struct Arg {
    expr: String,
    path: Option<ExprPath>,
    typ: String,
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
    pub node_mutex: BitSet<Id>,
    pub ret_mutex: BitSet<Id>,
    pub propagation_mutex: BitSet<Id>,
    pub propagation: HashMap<DefId, BitSet<Id>>,
    pub propagation_raw: Vec<(DefId, BitSet<Id>)>,
    pub access: HashMap<ExprPath, (BitSet<Id>, bool)>,
    pub access_raw: Vec<(ExprPath, BitSet<Id>, bool)>,
}

impl FunctionSummary {
    pub fn new(
        entry_mutex: BitSet<Id>,
        node_mutex: BitSet<Id>,
        ret_mutex: BitSet<Id>,
        propagation_raw: Vec<(DefId, BitSet<Id>)>,
        access_raw: Vec<(ExprPath, BitSet<Id>, bool)>,
    ) -> Self {
        let len = entry_mutex.domain_size();
        let mut propagation = HashMap::new();
        for (def_id, v) in &propagation_raw {
            let ov = propagation
                .entry(*def_id)
                .or_insert_with(|| BitSet::new_filled(len));
            ov.intersect(v);
        }
        let mut access = HashMap::new();
        for (path, v, w) in &access_raw {
            let (ov, ow) = access
                .entry(path.clone())
                .or_insert_with(|| (BitSet::new_filled(len), false));
            ov.intersect(v);
            *ow |= w;
        }
        Self {
            entry_mutex,
            node_mutex,
            ret_mutex,
            propagation_mutex: BitSet::new_empty(len),
            propagation,
            propagation_raw,
            access,
            access_raw,
        }
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
