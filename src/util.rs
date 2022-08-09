use std::{
    path::{Path, PathBuf},
    process::Command,
};

use etrace::some_or;
use rustc_hir::{def::Res, Expr, ExprKind, HirId, Node, UnOp};
use rustc_index::vec::Idx;
use rustc_lint::{LateContext, LintContext};
use rustc_middle::ty::{TyCtxt, TypeckResults};
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

pub fn expr_to_path(ctx: &LateContext<'_>, expr: &Expr<'_>) -> Option<Vec<String>> {
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
                            let arr = &args[0];
                            let mut p1 = expr_to_path(ctx, arr)?;
                            let mut p2 = expr_to_path(ctx, index)?;
                            p1.append(&mut p2);
                            Some(p1)
                        }
                        _ => None,
                    },
                    _ => None,
                }
            }
            _ => None,
        },
        ExprKind::Field(e, f) => {
            let mut v = expr_to_path(ctx, e)?;
            v.push(span_to_string(ctx, f.span));
            Some(v)
        }
        ExprKind::Path(_) => Some(vec![span_to_string(ctx, expr.span)]),
        _ => None,
    }
}

pub fn type_of<'a, 'b>(ctx: &'a LateContext<'b>, hir_id: HirId) -> rustc_middle::ty::Ty<'b> {
    ctx.tcx.typeck(hir_id.owner).node_type(hir_id)
}

pub fn type_as_string(ctx: &LateContext<'_>, hir_id: HirId) -> String {
    type_of(ctx, hir_id).to_string().replace("main::", "")
}

pub fn join(mut v: Vec<String>, sep: &str) -> String {
    v.drain(..).intersperse(sep.to_string()).collect()
}
