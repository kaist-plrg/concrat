use std::{
    fmt::Display,
    path::{Path, PathBuf},
    process::Command,
};

use etrace::some_or;
use rustc_hir::{def::Res, Expr, ExprKind, HirId, Node, UnOp};
use rustc_index::vec::Idx;
use rustc_lint::{LateContext, LintContext};
use rustc_middle::ty::{Ty, TyCtxt, TyKind, TypeAndMut, TypeckResults};
use rustc_mir_dataflow::fmt::DebugWithContext;
use rustc_span::{def_id::DefId, Span};

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, PartialOrd, Ord)]
pub struct Id(usize);

impl Idx for Id {
    fn new(idx: usize) -> Self {
        Self(idx)
    }

    fn index(self) -> usize {
        self.0
    }
}

impl<T> DebugWithContext<T> for Id {}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ExprPath {
    pub base: String,
    pub projections: Vec<ExprPathProj>,
}

impl ExprPath {
    pub fn new(base: String, projections: Vec<ExprPathProj>) -> Self {
        Self { base, projections }
    }

    pub fn add_suffix(&mut self, proj: ExprPathProj) {
        self.projections.push(proj);
    }

    pub fn add_base(&mut self, base: String) {
        let mut v = vec![];
        v.append(&mut self.projections);
        let old_base = std::mem::replace(&mut self.base, base);
        self.projections.push(ExprPathProj::Field(old_base));
        self.projections.append(&mut v);
    }

    pub fn is_variable(&self) -> bool {
        self.projections.is_empty()
    }

    pub fn is_array(&self) -> bool {
        matches!(self.projections.get(0), Some(ExprPathProj::Index(_)))
    }

    pub fn is_struct(&self) -> bool {
        matches!(self.projections.get(0), Some(ExprPathProj::Field(_)))
    }

    pub fn set_base(&mut self, path: &ExprPath) {
        self.base = path.base.clone();
        let mut v = vec![];
        v.append(&mut self.projections);
        self.projections = path.projections.clone();
        self.projections.append(&mut v);
    }

    pub fn suffix(&self) -> Option<ExprPath> {
        if self.is_variable() {
            return None;
        }
        let new_base = match &self.projections[0] {
            ExprPathProj::Field(x) | ExprPathProj::Index(x) => x,
        };
        Some(Self::new(new_base.clone(), self.projections[1..].to_vec()))
    }

    pub fn strip_prefix(&self, path: &ExprPath) -> Option<ExprPath> {
        if self.base != path.base {
            return None;
        }
        let new_self = self.suffix()?;
        if path.is_variable() {
            Some(new_self)
        } else {
            let new_path = path.suffix()?;
            new_self.strip_prefix(&new_path)
        }
    }

    pub fn pop(&mut self) -> Option<ExprPathProj> {
        self.projections.pop()
    }
}

impl Display for ExprPath {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.base)?;
        for p in &self.projections {
            write!(fmt, "{}", p)?;
        }
        Ok(())
    }
}

impl core::fmt::Debug for ExprPath {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, fmt)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ExprPathProj {
    Field(String),
    Index(String),
}

impl Display for ExprPathProj {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Field(f) => write!(fmt, ".{}", f),
            Self::Index(i) => write!(fmt, "[{}]", i),
        }
    }
}

pub fn compile_args(input: &Path, dep: &Path) -> Vec<String> {
    let dep = dep.to_str().unwrap();
    let dependency = format!("dependency={}", dep);
    let asm_casts = format!(
        "c2rust_asm_casts={}/libc2rust_asm_casts-5452fb8e557bf4f0.rlib",
        dep
    );
    let bitfields = format!(
        "c2rust_bitfields={}/libc2rust_bitfields-dea51de3183f0659.rlib",
        dep
    );
    vec![
        "create-initial-program",
        input.to_str().unwrap(),
        "--sysroot",
        sys_root().as_str(),
        "--crate-type",
        "lib",
        "-A",
        "warnings",
        "-L",
        &dependency,
        "--extern",
        &asm_casts,
        "--extern",
        &bitfields,
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}

fn sys_root() -> String {
    std::env::var("SYSROOT")
        .ok()
        .map(PathBuf::from)
        .or_else(|| {
            let home = std::env::var("RUSTUP_HOME")
                .or_else(|_| std::env::var("MULTIRUST_HOME"))
                .ok();
            let toolchain = std::env::var("RUSTUP_TOOLCHAIN")
                .or_else(|_| std::env::var("MULTIRUST_TOOLCHAIN"))
                .ok();
            toolchain_path(home, toolchain)
        })
        .or_else(|| {
            Command::new("rustc")
                .arg("--print")
                .arg("sysroot")
                .output()
                .ok()
                .and_then(|out| String::from_utf8(out.stdout).ok())
                .map(|s| PathBuf::from(s.trim()))
        })
        .or_else(|| option_env!("SYSROOT").map(PathBuf::from))
        .or_else(|| {
            let home = option_env!("RUSTUP_HOME")
                .or(option_env!("MULTIRUST_HOME"))
                .map(ToString::to_string);
            let toolchain = option_env!("RUSTUP_TOOLCHAIN")
                .or(option_env!("MULTIRUST_TOOLCHAIN"))
                .map(ToString::to_string);
            toolchain_path(home, toolchain)
        })
        .map(|pb| pb.to_string_lossy().to_string())
        .unwrap()
}

fn toolchain_path(home: Option<String>, toolchain: Option<String>) -> Option<PathBuf> {
    home.and_then(|home| {
        toolchain.map(|toolchain| {
            let mut path = PathBuf::from(home);
            path.push("toolchains");
            path.push(toolchain);
            path
        })
    })
}

pub fn span_to_string(ctx: &LateContext<'_>, span: Span) -> String {
    let source_map = ctx.sess().source_map();
    source_map.span_to_snippet(span).unwrap()
}

pub fn current_function(ctx: &LateContext<'_>) -> Option<DefId> {
    let bid = some_or!(&ctx.enclosing_body, return None);
    let hir = ctx.tcx.hir();
    Some(hir.local_def_id(hir.body_owner(*bid)).to_def_id())
}

pub fn typeck<'tcx>(ctx: &LateContext<'tcx>) -> Option<&'tcx TypeckResults<'tcx>> {
    let bid = some_or!(&ctx.enclosing_body, return None);
    Some(ctx.tcx.typeck_body(*bid))
}

pub fn resolve_path(ctx: &LateContext<'_>, expr: &Expr<'_>) -> Option<Res> {
    if let ExprKind::Path(p) = &expr.kind {
        let typeck_res = some_or!(typeck(ctx), return None);
        Some(typeck_res.qpath_res(p, expr.hir_id))
    } else {
        None
    }
}

pub fn def_id_to_item_name(tcx: TyCtxt<'_>, def_id: DefId) -> String {
    match tcx.hir().get_if_local(def_id).unwrap() {
        Node::Item(i) => i.ident.to_string(),
        _ => unreachable!(),
    }
}

pub fn normalize_path(p: &str) -> String {
    p.replace("&mut ", "")
        .split(&[' ', '-', '>', '.', '(', ')', '*', '&'])
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(".")
}

pub fn expr_to_path(ctx: &LateContext<'_>, expr: &Expr<'_>) -> Option<ExprPath> {
    match &expr.kind {
        ExprKind::Box(e)
        | ExprKind::Unary(UnOp::Deref, e)
        | ExprKind::Cast(e, _)
        | ExprKind::DropTemps(e)
        | ExprKind::AddrOf(_, _, e) => expr_to_path(ctx, e),
        ExprKind::MethodCall(m, args, _) => match m.ident.to_string().as_str() {
            "offset" => {
                let index = &args[1];
                match args[0].kind {
                    ExprKind::MethodCall(m, args, _) => match m.ident.to_string().as_str() {
                        "as_mut_ptr" => {
                            let mut base = expr_to_path(ctx, &args[0])?;
                            let index = unwrap_cast_recursively(index);
                            let index = span_to_string(ctx, index.span);
                            base.add_suffix(ExprPathProj::Index(index));
                            Some(base)
                        }
                        _ => None,
                    },
                    _ => None,
                }
            }
            _ => None,
        },
        ExprKind::Field(e, f) => {
            let mut base = expr_to_path(ctx, e)?;
            base.add_suffix(ExprPathProj::Field(span_to_string(ctx, f.span)));
            Some(base)
        }
        ExprKind::Index(e, i) => {
            let mut base = expr_to_path(ctx, e)?;
            let index = unwrap_cast_recursively(i);
            let index = span_to_string(ctx, index.span);
            base.add_suffix(ExprPathProj::Index(index));
            Some(base)
        }
        ExprKind::Path(_) => Some(ExprPath::new(span_to_string(ctx, expr.span), vec![])),
        _ => None,
    }
}

pub fn type_of<'a, 'b>(ctx: &'a LateContext<'b>, hir_id: HirId) -> Ty<'b> {
    ctx.tcx.typeck(hir_id.owner).node_type(hir_id)
}

pub fn unwrap_ptr_from_type(ty: Ty<'_>) -> Ty<'_> {
    match ty.kind() {
        TyKind::RawPtr(TypeAndMut { ty, .. }) | TyKind::Ref(_, ty, _) => unwrap_ptr_from_type(*ty),
        _ => ty,
    }
}

pub fn type_to_string(ty: Ty<'_>) -> String {
    ty.to_string().replace("main::", "")
}

pub fn join(mut v: Vec<String>, sep: &str) -> String {
    v.drain(..).intersperse(sep.to_string()).collect()
}

pub fn unwrap_cast_recursively<'tcx>(e: &'tcx Expr<'tcx>) -> &'tcx Expr<'tcx> {
    match &e.kind {
        ExprKind::Cast(e, _) => unwrap_cast_recursively(e),
        _ => e,
    }
}
