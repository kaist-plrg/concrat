use rustc_hir::{Expr, HirId};
use rustc_lint::LateContext;
use rustc_middle::mir::{Operand, Terminator, TerminatorKind};
use rustc_span::def_id::DefId;

pub mod intra;
pub mod pass;

pub use pass::run;

use crate::util::{expr_to_path, span_to_string, type_as_string, ExprPath};

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
        let typ = type_as_string(ctx, expr.hir_id)
            .replace("&mut ", "")
            .replace("*mut ", "")
            .replace("*const ", "");
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
