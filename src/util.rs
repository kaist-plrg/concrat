use std::{
    collections::HashSet,
    fmt::Display,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

use etrace::some_or;
use rustc_hir::{def::Res, Expr, ExprKind, HirId, Node, UnOp};
use rustc_index::vec::Idx;
use rustc_lint::{LateContext, LintContext};
use rustc_middle::ty::{Ty, TyCtxt, TyKind, TypeAndMut, TypeckResults};
use rustc_mir_dataflow::fmt::DebugWithContext;
use rustc_span::{def_id::DefId, Span};
use serde::{
    de::{Error, Visitor},
    Deserialize, Serialize,
};

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, PartialOrd, Ord)]
#[repr(transparent)]
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

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

    pub fn replace_prefix(&self, old: &ExprPath, new: &ExprPath) -> Option<ExprPath> {
        let mut suffix = self.strip_prefix(old)?;
        suffix.add_base(String::new());
        suffix.set_base(new);
        Some(suffix)
    }

    pub fn pop(&mut self) -> Option<ExprPathProj> {
        self.projections.pop()
    }

    pub fn index(&self) -> Option<&String> {
        if let Some(ExprPathProj::Index(i)) = self.projections.get(0) {
            Some(i)
        } else {
            None
        }
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

impl Serialize for ExprPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ExprPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        deserializer.deserialize_string(PathVisitor)
    }

    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where D: serde::Deserializer<'de> {
        deserializer.deserialize_string(PathInPlaceVisitor(place))
    }
}

struct PathVisitor;

impl Visitor<'_> for PathVisitor {
    type Value = ExprPath;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("ExprPath")
    }

    fn visit_borrowed_str<E>(self, v: &str) -> Result<Self::Value, E>
    where E: Error {
        Ok(v.parse().unwrap())
    }
}

struct PathInPlaceVisitor<'a>(&'a mut ExprPath);

impl Visitor<'_> for PathInPlaceVisitor<'_> {
    type Value = ();

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("ExprPath")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where E: Error {
        *self.0 = v.parse().unwrap();
        Ok(())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

impl FromStr for ExprPath {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .replace("&mut", "")
            .replace('&', "")
            .replace('*', "")
            .replace('(', "")
            .replace(')', "")
            .replace(' ', "");
        if s.is_empty() {
            return Err(());
        }

        let mut base = None;
        let mut projections = vec![];
        let mut s = s.as_str();
        while let Some((s1, s2, b)) = find_delimiter(s) {
            if !s1.is_empty() {
                if base.is_none() {
                    base = Some(s1.to_string());
                } else {
                    projections.push(ExprPathProj::Field(s1.to_string()));
                }
            }
            if b {
                s = s2;
            } else {
                let (i, s2) = s2.split_once(']').unwrap();
                projections.push(ExprPathProj::Index(i.to_string()));
                s = s2;
            }
        }
        if !s.is_empty() {
            if base.is_none() {
                base = Some(s.to_string());
            } else {
                projections.push(ExprPathProj::Field(s.to_string()));
            }
        }
        Ok(ExprPath::new(base.unwrap(), projections))
    }
}

fn find_delimiter(s: &str) -> Option<(&str, &str, bool)> {
    let l = s.len();
    let i1 = s.find('.').unwrap_or(l);
    let i2 = s.find("->").unwrap_or(l);
    let i3 = s.find('[').unwrap_or(l);
    if i1 == i2 && i1 == i3 {
        None
    } else if i1 < i2 && i1 < i3 {
        Some((&s[..i1], &s[(i1 + 1)..], true))
    } else if i2 < i3 {
        Some((&s[..i2], &s[(i2 + 2)..], true))
    } else {
        Some((&s[..i3], &s[(i3 + 1)..], false))
    }
}

#[cfg(test)]
mod tests {
    use super::{ExprPath, ExprPathProj};

    #[test]
    fn test1() {
        let p1 = ExprPath::new("a".to_string(), vec![]);
        let p2 = ExprPath::new("a".to_string(), vec![ExprPathProj::Field("b".to_string())]);
        let p3 = ExprPath::new(
            "a".to_string(),
            vec![
                ExprPathProj::Field("b".to_string()),
                ExprPathProj::Field("c".to_string()),
            ],
        );
        let p4 = ExprPath::new("a".to_string(), vec![ExprPathProj::Index("i".to_string())]);
        let p5 = ExprPath::new(
            "a".to_string(),
            vec![
                ExprPathProj::Index("i".to_string()),
                ExprPathProj::Index("j".to_string()),
            ],
        );
        let p6 = ExprPath::new(
            "a".to_string(),
            vec![
                ExprPathProj::Field("b".to_string()),
                ExprPathProj::Index("i".to_string()),
            ],
        );
        let p7 = ExprPath::new(
            "a".to_string(),
            vec![
                ExprPathProj::Index("i".to_string()),
                ExprPathProj::Field("b".to_string()),
            ],
        );
        let p8 = ExprPath::new(
            "a".to_string(),
            vec![
                ExprPathProj::Field("b".to_string()),
                ExprPathProj::Index("i".to_string()),
                ExprPathProj::Field("c".to_string()),
                ExprPathProj::Index("j".to_string()),
            ],
        );

        assert_eq!(p1, "a".parse().unwrap());
        assert_eq!(p2, "a.b".parse().unwrap());
        assert_eq!(p2, "a->b".parse().unwrap());
        assert_eq!(p3, "a.b.c".parse().unwrap());
        assert_eq!(p3, "a.b->c".parse().unwrap());
        assert_eq!(p3, "a->b.c".parse().unwrap());
        assert_eq!(p3, "a->b->c".parse().unwrap());
        assert_eq!(p4, "a[i]".parse().unwrap());
        assert_eq!(p5, "a[i][j]".parse().unwrap());
        assert_eq!(p6, "a.b[i]".parse().unwrap());
        assert_eq!(p7, "a[i].b".parse().unwrap());
        assert_eq!(p8, "a.b[i].c[j]".parse().unwrap());

        let arr = [p1, p2, p3, p4, p5, p6, p7, p8];
        for (i, p) in arr.iter().enumerate() {
            assert_eq!(
                &serde_json::from_str::<ExprPath>(serde_json::to_string(p).unwrap().as_str())
                    .unwrap(),
                p,
                "{}",
                i + 1
            );
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

pub fn def_id_to_span(tcx: TyCtxt<'_>, def_id: DefId) -> Span {
    let hir = tcx.hir();
    hir.span_with_body(hir.local_def_id_to_hir_id(def_id.expect_local()))
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

pub fn unwrap_cast_recursively<'a, 'tcx>(e: &'a Expr<'tcx>) -> &'a Expr<'tcx> {
    match &e.kind {
        ExprKind::Cast(e, _) => unwrap_cast_recursively(e),
        _ => e,
    }
}

pub fn unwrap_call<'a, 'tcx>(e: &'a Expr<'tcx>) -> &'a Expr<'tcx> {
    match &e.kind {
        ExprKind::Call(_, args) => unwrap_call(&args[0]),
        _ => e,
    }
}

pub fn span_lines(ctx: &LateContext<'_>, span: Span) -> HashSet<usize> {
    let source_map = ctx.sess().source_map();
    let fname = source_map.span_to_filename(span);
    let file = source_map.get_source_file(&fname).unwrap();
    let lo = file.lookup_file_pos_with_col_display(span.lo());
    let hi = file.lookup_file_pos_with_col_display(span.hi());
    ((lo.0)..=(hi.0)).collect()
}
