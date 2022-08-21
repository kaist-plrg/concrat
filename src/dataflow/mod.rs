use std::collections::{BTreeMap, BTreeSet};

use rustc_hir::{Expr, HirId};
use rustc_lint::LateContext;
use rustc_middle::mir::{Operand, Terminator, TerminatorKind};
use rustc_span::{def_id::DefId, Span};

pub mod domain;
pub mod intra;
pub mod pass;
pub mod visitor;

pub use pass::run;

use self::domain::{MayMutexSet, MustMutexSet};
use crate::util::{
    expr_to_path, intersection, span_to_string, type_of, type_to_string, unwrap_ptr_from_type,
    ExprPath,
};

#[derive(Debug, Clone)]
pub struct Arg {
    pub path: Option<ExprPath>,
    #[allow(unused)]
    pub typ: String,
    #[allow(unused)]
    pub expr: String,
    #[allow(unused)]
    pub hir_id: HirId,
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

#[derive(Debug, Default, Clone)]
pub struct FunctionCodeSummary {
    mutexes: BTreeSet<ExprPath>,
    params: Vec<(String, String)>,
    calls: Vec<(Span, DefId, String, Vec<Arg>)>,
    accesses: Vec<(Span, ExprPath, bool)>,
    path_types: BTreeMap<ExprPath, String>,
    init_or_destroy: BTreeSet<ExprPath>,
}

impl FunctionCodeSummary {
    fn add_mutex(&mut self, mutex: ExprPath) {
        self.mutexes.insert(mutex);
    }

    fn add_call(&mut self, span: Span, callee: DefId, callee_name: String, args: Vec<Arg>) {
        self.calls.push((span, callee, callee_name, args));
    }

    fn add_access(&mut self, span: Span, path: ExprPath, write: bool) {
        self.accesses.push((span, path, write));
    }

    fn add_path(&mut self, path: ExprPath, ty: String) {
        self.path_types.insert(path, ty);
    }

    fn add_init_or_destroy(&mut self, path: ExprPath) {
        self.init_or_destroy.insert(path);
    }
}

pub type MutexSet = BTreeSet<ExprPath>;

#[derive(Debug)]
pub struct FunctionSummary {
    pub entry_mutex: MayMutexSet,
    pub ret_mutex: MustMutexSet,
    pub propagation_mutex: MutexSet,
    pub propagation: BTreeMap<DefId, MutexSet>,
    pub propagation_raw: Vec<(DefId, MutexSet)>,
    pub access: Vec<(ExprPath, MutexSet, bool)>,
    pub span_mutex: Vec<(Span, MutexSet)>,
}

impl FunctionSummary {
    pub fn new(
        entry_mutex: MayMutexSet,
        ret_mutex: MustMutexSet,
        propagation_raw: Vec<(DefId, MutexSet)>,
        access: Vec<(ExprPath, MutexSet, bool)>,
        span_mutex: Vec<(Span, MutexSet)>,
    ) -> Self {
        let mut propagation: BTreeMap<DefId, MutexSet> = BTreeMap::new();
        for (def_id, v) in &propagation_raw {
            if let Some(s) = propagation.get_mut(def_id) {
                *s = intersection(s.clone(), v);
            } else {
                propagation.insert(*def_id, v.clone());
            }
        }
        Self {
            entry_mutex,
            ret_mutex,
            propagation_mutex: BTreeSet::new(),
            propagation,
            propagation_raw,
            access,
            span_mutex,
        }
    }

    pub fn mutex_only(entry_mutex: MayMutexSet, ret_mutex: MustMutexSet) -> Self {
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
