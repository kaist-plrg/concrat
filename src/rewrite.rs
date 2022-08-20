use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::Read,
    sync::Mutex,
};

use etrace::some_or;
use lazy_static::lazy_static;
use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass, LintContext, LintPass};
use rustc_span::{BytePos, Span, Symbol};
use rustfix::{Replacement, Snippet, Solution, Suggestion};
use spin::once::Once;

use crate::{
    analysis::{AnalysisSummary, FunctionSummary},
    callback::{compile_with, LatePass},
    graph::transitive_closure,
    util::{
        expr_to_path, function_params, join, span_lines, span_to_string, type_of, type_to_string,
        unwrap_cast_recursively, unwrap_ptr_from_type, ExprPath, ExprPathProj,
    },
};

lazy_static! {
    static ref REPLACEMENTS: Mutex<Vec<Replacement>> = Mutex::new(vec![]);
}

static SUMMARY: Once<AnalysisSummary> = Once::new();
static GLOBAL_DEF_MAP: Once<BTreeMap<String, (String, String)>> = Once::new();
static ARRAY_DEF_MAP: Once<BTreeMap<String, (String, Vec<String>)>> = Once::new();
static STRUCT_DEF_MAP: Once<BTreeMap<String, BTreeMap<String, String>>> = Once::new();
static TRANS_STRUCT_DEF_MAP: Once<BTreeMap<String, BTreeSet<String>>> = Once::new();
static INIT_MAP: Once<BTreeMap<(String, ExprPath, String), String>> = Once::new();
static MUTEX_INIT_MAP: Once<BTreeSet<(String, ExprPath, String)>> = Once::new();
static PATH_TYPE_MAP: Once<BTreeMap<ExprPath, BTreeMap<String, String>>> = Once::new();
static DURATION_MAP: Once<BTreeMap<(String, String, String), String>> = Once::new();
static TRYLOCK_MAP: Once<BTreeMap<(String, String, usize), String>> = Once::new();
static PARAMS_MAP: Once<BTreeMap<String, Vec<String>>> = Once::new();

fn global_mutex_map() -> &'static BTreeMap<String, String> {
    &SUMMARY.get().unwrap().mutex_map
}

fn array_mutex_map() -> &'static BTreeMap<String, String> {
    &SUMMARY.get().unwrap().array_mutex_map
}

fn struct_mutex_map() -> &'static BTreeMap<String, BTreeMap<String, String>> {
    &SUMMARY.get().unwrap().struct_mutex_map
}

fn function_mutex_map() -> &'static BTreeMap<String, FunctionSummary> {
    &SUMMARY.get().unwrap().function_map
}

fn global_def_map() -> &'static BTreeMap<String, (String, String)> {
    GLOBAL_DEF_MAP.get().unwrap()
}

fn array_def_map() -> &'static BTreeMap<String, (String, Vec<String>)> {
    ARRAY_DEF_MAP.get().unwrap()
}

fn struct_def_map() -> &'static BTreeMap<String, BTreeMap<String, String>> {
    STRUCT_DEF_MAP.get().unwrap()
}

fn init_map() -> &'static BTreeMap<(String, ExprPath, String), String> {
    INIT_MAP.get().unwrap()
}

fn mutex_init_map() -> &'static BTreeSet<(String, ExprPath, String)> {
    MUTEX_INIT_MAP.get().unwrap()
}

fn path_type_map() -> &'static BTreeMap<ExprPath, BTreeMap<String, String>> {
    PATH_TYPE_MAP.get().unwrap()
}

fn duration_map() -> &'static BTreeMap<(String, String, String), String> {
    DURATION_MAP.get().unwrap()
}

fn trylock_map() -> &'static BTreeMap<(String, String, usize), String> {
    TRYLOCK_MAP.get().unwrap()
}

fn params_map() -> &'static BTreeMap<String, Vec<String>> {
    PARAMS_MAP.get().unwrap()
}

pub fn collect_replacements(args: Vec<String>, summary: AnalysisSummary) -> Vec<Replacement> {
    SUMMARY.call_once(|| summary);

    let exit_code = compile_with(args.clone(), vec![GlobalPass::new]);
    assert_eq!(exit_code, 0);

    let exit_code = compile_with(args, vec![RewritePass::new]);
    assert_eq!(exit_code, 0);

    let mut replacements = REPLACEMENTS.lock().unwrap();
    replacements.sort_by_key(|r| r.snippet.range.start);
    std::mem::take(&mut replacements)
}

pub fn apply_suggestions(mut replacements: Vec<Replacement>) -> String {
    let file = &replacements.last().unwrap().snippet.file_name;
    let mut file = File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let suggestions: Vec<_> = replacements
        .drain(..)
        .map(|r| Suggestion {
            message: "".to_owned(),
            snippets: vec![r.snippet.clone()],
            solutions: vec![Solution {
                message: "".to_string(),
                replacements: vec![r],
            }],
        })
        .collect();
    rustfix::apply_suggestions(contents.as_str(), &suggestions).unwrap()
}

#[derive(Default)]
struct GlobalPass {
    global_def_map: BTreeMap<String, (String, String)>,
    array_def_map: BTreeMap<String, (String, Vec<String>)>,
    struct_def_map: BTreeMap<String, BTreeMap<String, String>>,
    init_map: BTreeMap<(String, ExprPath, String), String>,
    mutex_init_map: BTreeSet<(String, ExprPath, String)>,
    path_type_map: BTreeMap<ExprPath, BTreeMap<String, String>>,
    duration_map: BTreeMap<(String, String, String), String>,
    trylock_map: BTreeMap<(String, String), Vec<(usize, String)>>,
    if_map: BTreeMap<(String, String), Vec<usize>>,
    params_map: BTreeMap<String, Vec<String>>,
    ty_alias_map: BTreeMap<String, String>,
}

impl GlobalPass {
    #[allow(clippy::new_ret_no_self)]
    fn new() -> Box<LatePass> {
        Box::new(Self::default())
    }
}

impl LintPass for GlobalPass {
    fn name(&self) -> &'static str {
        "GlobalPass"
    }
}

impl<'tcx> LateLintPass<'tcx> for GlobalPass {
    fn check_item(&mut self, ctx: &LateContext<'tcx>, i: &'tcx Item<'tcx>) {
        let name = i.ident.name.to_ident_string();
        match &i.kind {
            ItemKind::Struct(VariantData::Struct(fs, _), _)
            | ItemKind::Union(VariantData::Struct(fs, _), _) => {
                let map = fs
                    .iter()
                    .map(|f| {
                        (
                            f.ident.name.to_ident_string(),
                            span_to_string(ctx, f.ty.span),
                        )
                    })
                    .collect();
                self.struct_def_map.insert(name, map);
            }
            ItemKind::Static(t, _, b) => {
                // global
                let ty = span_to_string(ctx, t.span);
                let init = hid_to_string(ctx, b.hir_id);
                self.global_def_map.insert(name.clone(), (ty, init));

                // array
                match t.kind {
                    TyKind::Array(t, _) => {
                        let ty = span_to_string(ctx, t.span);
                        let init = match &ctx.tcx.hir().body(*b).value.kind {
                            ExprKind::Block(Block { expr: Some(e), .. }, _) => &e.kind,
                            e => e,
                        };
                        let init = match init {
                            ExprKind::Repeat(e, l) => {
                                let e = span_to_string(ctx, e.span);
                                let l = hid_to_string(ctx, l.hir_id()).parse().unwrap();
                                vec![e; l]
                            }
                            ExprKind::Array(es) => {
                                es.iter().map(|e| span_to_string(ctx, e.span)).collect()
                            }
                            _ => unreachable!(),
                        };
                        self.array_def_map.insert(name, (ty, init));
                    }
                    _ => (),
                }
            }
            ItemKind::Fn(_, _, bid) => {
                let mut params = function_params(ctx, *bid);
                let params = params.drain(..).map(|p| p.0).collect();
                self.params_map.insert(name, params);
            }
            ItemKind::TyAlias(t, _) => {
                self.ty_alias_map.insert(name, span_to_string(ctx, t.span));
            }
            _ => (),
        }
    }

    fn check_expr(&mut self, ctx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) {
        if let Some(func) = current_function(ctx, e.hir_id) {
            if let Some(path) = expr_to_path(ctx, e) {
                let ty = type_to_string(unwrap_ptr_from_type(type_of(ctx, e.hir_id)));
                self.path_type_map.entry(path).or_default().insert(func, ty);
            }
        }

        let func_name = || current_function(ctx, e.hir_id).unwrap();
        match e.kind {
            ExprKind::Call(func, args) => {
                let f = name(func);
                match f.as_deref() {
                    Some("pthread_mutex_init" | "pthread_spin_init") => {
                        let mut path = some_or!(expr_to_path(ctx, &args[0]), return);
                        let f = some_or!(path.pop(), return);
                        if let ExprPathProj::Field(f) = f {
                            let func = func_name();
                            self.mutex_init_map.insert((func, path, f));
                        }
                    }
                    _ => (),
                }
            }
            ExprKind::Assign(lhs, rhs, _) => {
                match lhs.kind {
                    ExprKind::Field(s, f) => {
                        let func = func_name();
                        if let Some(func_summary) = function_mutex_map().get(&func) {
                            let is_protected = |mutex: &ExprPath| {
                                if let Some(lines) = func_summary.mutex_line.get(mutex) {
                                    let expr_lines = span_lines(ctx, e.span);
                                    expr_lines.iter().any(|l| lines.contains(l))
                                } else {
                                    false
                                }
                            };
                            let ty = type_to_string(type_of(ctx, s.hir_id));
                            let f = f.name.to_ident_string();
                            let map = some_or!(struct_mutex_map().get(&ty), return);
                            let m = some_or!(map.get(&f), return);
                            let mut mutex = expr_to_path(ctx, s).unwrap();
                            mutex.add_suffix(ExprPathProj::Field(m.clone()));
                            if !is_protected(&mutex) {
                                mutex.pop();
                                let init = span_to_string(ctx, rhs.span);
                                self.init_map.entry((func, mutex, f)).or_insert(init);
                            }
                        }
                    }
                    _ => (),
                }
                match rhs.kind {
                    ExprKind::Call(func, args) => {
                        if let Some(f) = name(func) {
                            if f == "pthread_mutex_lock"
                                || f == "pthread_spin_lock"
                                || f == "pthread_mutex_trylock"
                                || f == "pthread_spin_trylock"
                            {
                                let f = func_name();
                                let l = span_to_string(ctx, lhs.span);
                                let line = span_lines(ctx, args[0].span)
                                    .drain_filter(|_| true)
                                    .max()
                                    .unwrap();
                                let (_, g) = normalize_arg(ctx, &args[0]);
                                self.trylock_map.entry((f, l)).or_default().push((line, g));
                            }
                        }
                    }
                    ExprKind::Path(_) => {
                        let f = func_name();
                        let x = span_to_string(ctx, rhs.span);
                        if let Some(v) = self.trylock_map.get(&(f.clone(), x)) {
                            let v = v.last().unwrap().clone();
                            let l = span_to_string(ctx, lhs.span);
                            self.trylock_map.entry((f, l)).or_default().push(v);
                        }
                    }
                    _ => (),
                }
            }
            ExprKind::AssignOp(op, lhs, rhs) => match op.node {
                BinOpKind::Add => match lhs.kind {
                    ExprKind::Field(s, f) => {
                        if type_to_string(type_of(ctx, s.hir_id)) == "timespec" {
                            let func = func_name();
                            let s = span_to_string(ctx, s.span);
                            let f = span_to_string(ctx, f.span);
                            let r = span_to_string(ctx, rhs.span);
                            self.duration_map.insert((func, s, f), r);
                        }
                    }
                    _ => (),
                },
                _ => (),
            },
            ExprKind::If(c, _, _) => {
                let (expr, _) = some_or!(read_condition(ctx, c), return);
                let f = func_name();
                let line = span_lines(ctx, c.span)
                    .drain_filter(|_| true)
                    .min()
                    .unwrap();
                self.if_map.entry((f, expr)).or_default().push(line);
            }
            _ => (),
        }
    }

    fn check_crate_post(&mut self, _: &LateContext<'tcx>) {
        let mut map: BTreeMap<_, BTreeSet<_>> = self
            .struct_def_map
            .iter()
            .map(|(k, m)| (k.clone(), m.values().cloned().collect()))
            .collect();
        for (lt, rt) in self.ty_alias_map.drain_filter(|_, _| true) {
            map.insert(lt, BTreeSet::from([rt]));
        }
        TRANS_STRUCT_DEF_MAP.call_once(|| transitive_closure(map));

        GLOBAL_DEF_MAP.call_once(|| std::mem::take(&mut self.global_def_map));
        ARRAY_DEF_MAP.call_once(|| std::mem::take(&mut self.array_def_map));
        STRUCT_DEF_MAP.call_once(|| std::mem::take(&mut self.struct_def_map));
        INIT_MAP.call_once(|| std::mem::take(&mut self.init_map));
        MUTEX_INIT_MAP.call_once(|| std::mem::take(&mut self.mutex_init_map));
        PATH_TYPE_MAP.call_once(|| std::mem::take(&mut self.path_type_map));
        DURATION_MAP.call_once(|| std::mem::take(&mut self.duration_map));
        PARAMS_MAP.call_once(|| std::mem::take(&mut self.params_map));

        let mut trylock_map = BTreeMap::new();
        for (k, v) in &mut self.trylock_map {
            let lines = some_or!(self.if_map.get(k), continue);
            for l in lines {
                v.push((*l, String::new()));
            }
            v.sort_by_key(|(l, _)| *l);

            let mut guard = String::new();
            for (l, g) in v {
                if !guard.is_empty() && g.is_empty() {
                    let (f, expr) = k;
                    trylock_map.insert((f.clone(), expr.clone(), *l), guard);
                    guard = String::new();
                } else if guard.is_empty() && !g.is_empty() {
                    guard = g.clone();
                }
            }
        }
        TRYLOCK_MAP.call_once(|| trylock_map);
    }
}

#[derive(Default)]
struct RewritePass {
    guard_map: BTreeMap<String, Vec<String>>,
    replaced: BTreeSet<Span>,
}

impl RewritePass {
    #[allow(clippy::new_ret_no_self)]
    fn new() -> Box<LatePass> {
        Box::new(Self::default())
    }

    fn use_guard(&mut self, func: String, guard: String) {
        self.guard_map.entry(func).or_insert(vec![]).push(guard);
    }

    fn use_guards(&mut self, func: String, guards: &Vec<String>) {
        for g in guards {
            self.use_guard(func.clone(), g.clone());
        }
    }
}

impl LintPass for RewritePass {
    fn name(&self) -> &'static str {
        "RewritePass"
    }
}

impl<'tcx> LateLintPass<'tcx> for RewritePass {
    fn check_mod(&mut self, ctx: &LateContext<'tcx>, m: &'tcx Mod<'tcx>, _: Span, _: HirId) {
        let hir = ctx.tcx.hir();
        if m.item_ids
            .iter()
            .any(|i| matches!(hir.item(*i).kind, ItemKind::Fn(_, _, _)))
        {
            let span = m.spans.inner_span.shrink_to_lo();
            add_replacement(
                ctx,
                span,
                "use std::{sync::{Mutex, MutexGuard, Condvar}, time::Duration};\nu".to_string(),
            );
        }
    }

    fn check_item(&mut self, ctx: &LateContext<'tcx>, i: &'tcx Item<'tcx>) {
        match &i.kind {
            ItemKind::Impl(im) => {
                let t = im.of_trait.as_ref().map(|t| path_to_string(t.path));
                if t == Some("Copy".to_string()) {
                    let s = match &im.self_ty.kind {
                        TyKind::Path(QPath::Resolved(_, p)) => path_to_string(p),
                        _ => unreachable!(),
                    };
                    if let Some(map) = TRANS_STRUCT_DEF_MAP.get().unwrap().get(&s) {
                        if map.iter().any(|t| {
                            t == "pthread_mutex_t"
                                || t == "pthread_spinlock_t"
                                || t == "pthread_cond_t"
                        }) {
                            let span = i.span;
                            add_replacement(
                                ctx,
                                span.with_lo(span.lo() - BytePos(9))
                                    .with_hi(span.hi() + BytePos(9)),
                                "".to_string(),
                            );
                        }
                    }
                }
            }
            ItemKind::Struct(VariantData::Struct(fs, _), _) => {
                let s = i.ident.name.to_ident_string();
                let empty = BTreeMap::new();
                let map = struct_mutex_map().get(&s).unwrap_or(&empty);
                let mut new_structs = String::new();
                let struct_def_map = struct_def_map();
                let v: Vec<_> = map
                    .iter()
                    .map(|(x, m)| {
                        (
                            x.clone(),
                            struct_def_map.get(&s).unwrap().get(x).unwrap().clone(),
                            m.clone(),
                        )
                    })
                    .collect();
                for f in fs.iter() {
                    let name = f.ident.name.to_ident_string();
                    let typ = span_to_string(ctx, f.ty.span);
                    if typ == "pthread_cond_t" {
                        add_replacement(ctx, f.ty.span, "Condvar".to_string());
                        continue;
                    }
                    if v.iter().any(|(x, _, _)| *x == name) {
                        let span = f.span.with_hi(f.span.hi() + BytePos(1));
                        add_replacement(ctx, span, "".to_string());
                        continue;
                    }
                    if typ == "pthread_mutex_t" || typ == "pthread_spinlock_t" {
                        let pfs: Vec<_> = v
                            .iter()
                            .filter_map(|(x, t, m)| {
                                if *m == name {
                                    Some(format!("pub {}: {}", x, t))
                                } else {
                                    None
                                }
                            })
                            .collect();
                        let st_name = struct_of2(&s, &name);
                        add_replacement(ctx, f.ty.span, format!("Mutex<{}>", st_name));
                        let st_body = join(pfs, ", ");
                        let st = format!("\npub struct {} {{ {} }}", st_name, st_body);
                        new_structs.push_str(&st);
                    }
                }
                if !new_structs.is_empty() {
                    add_replacement(ctx, i.span.shrink_to_hi(), new_structs);
                }
            }
            ItemKind::Static(t, _, _) => {
                let name = i.ident.name.to_ident_string();
                let typ = span_to_string(ctx, t.span);

                // condvar
                if typ == "pthread_cond_t" {
                    let new_i = format!("pub static mut {}: Condvar = Condvar::new();", name);
                    add_replacement(ctx, i.span, new_i);
                    remove_attributes(ctx, i);
                    return;
                }

                // global or array
                if global_mutex_map().get(&name).is_some() || array_mutex_map().get(&name).is_some()
                {
                    add_replacement(ctx, i.span, "".to_string());
                    remove_attributes(ctx, i);
                    return;
                }

                // mutex
                if typ == "pthread_mutex_t" || typ == "pthread_spinlock_t" {
                    let mut decl = String::new();
                    let mut init = String::new();
                    for (x, m) in global_mutex_map() {
                        if *m == name {
                            let (t, i) = global_def_map().get(x).unwrap().clone();
                            decl.push_str(format!("pub {}: {}, ", x, t).as_str());
                            init.push_str(format!("{}: {}, ", x, i).as_str());
                        }
                    }
                    let struct_name = struct_of(&name);
                    let code = format!(
                        "
pub struct {0} {{ {1}}}
pub static mut {2}: Mutex<{0}> = Mutex::new(
    {0} {{ {3}}}
);",
                        struct_name, decl, name, init
                    );
                    add_replacement(ctx, i.span, code);
                    remove_attributes(ctx, i);
                    return;
                }

                // mutex array
                if let TyKind::Array(t, _) = t.kind {
                    let ty = span_to_string(ctx, t.span);
                    if ty == "pthread_mutex_t" || ty == "pthread_spinlock_t" {
                        let v: Vec<_> = array_mutex_map()
                            .iter()
                            .filter_map(|(x, m)| {
                                if **m == name {
                                    let (t, i) = array_def_map().get(x).unwrap().clone();
                                    Some((x, t, i))
                                } else {
                                    None
                                }
                            })
                            .collect();
                        let decls = v.iter().map(|(x, t, _)| format!("pub {}: {}", x, t));
                        let decl = join(decls.collect(), ", ");

                        let struct_name = struct_of(&name);
                        let mut lens: Vec<_> = v.iter().map(|(_, _, i)| i.len()).collect();
                        lens.dedup();
                        assert_eq!(lens.len(), 1);
                        let len = lens[0];
                        let init = join(
                            (0..len)
                                .map(|i| {
                                    let inits =
                                        v.iter().map(|(x, _, init)| format!("{}: {}", x, init[i]));
                                    let init = join(inits.collect(), ", ");
                                    format!(
                                        "
    Mutex::new(
        {} {{ {} }}
    )",
                                        struct_name, init
                                    )
                                })
                                .collect(),
                            ",",
                        );
                        let code = format!(
                            "
pub struct {0} {{ {1} }}
pub static mut {2}: [Mutex<{0}>; {3}] = [{4}
];",
                            struct_name, decl, name, len, init
                        );
                        add_replacement(ctx, i.span, code);
                        remove_attributes(ctx, i);
                    }
                }
            }
            _ => (),
        }
    }

    fn check_fn(
        &mut self,
        ctx: &LateContext<'tcx>,
        kind: intravisit::FnKind<'tcx>,
        decl: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        span: Span,
        _: HirId,
    ) {
        match kind {
            intravisit::FnKind::ItemFn(id, _, _) => {
                let name = id.name.to_ident_string();
                let empty = vec![];
                let (entry, ret) = if let Some(fs) = function_mutex_map().get(&name) {
                    let FunctionSummary {
                        entry_mutex,
                        ret_mutex,
                        ..
                    } = fs;
                    (entry_mutex, ret_mutex)
                } else {
                    (&empty, &empty)
                };

                if !entry.is_empty() {
                    let params = entry
                        .iter()
                        .map(|m| {
                            format!(
                                "mut {}: MutexGuard<'static, {}>",
                                m.guard(),
                                struct_of_path(&name, m)
                            )
                        })
                        .collect();
                    let params = join(params, ", ");
                    let (span, sugg) = if body.params.is_empty() {
                        (
                            ctx.sess()
                                .source_map()
                                .span_through_char(span, '(')
                                .shrink_to_hi(),
                            params,
                        )
                    } else {
                        (
                            decl.inputs.last().unwrap().span.shrink_to_hi(),
                            format!(", {}", params),
                        )
                    };
                    add_replacement(ctx, span, sugg);
                }

                if !ret.is_empty() && name != "main_0" {
                    let mut ret_types = vec![];
                    if let FnRetTy::Return(t) = decl.output {
                        ret_types.push(span_to_string(ctx, t.span));
                    }
                    for m in ret {
                        ret_types
                            .push(format!("MutexGuard<'static, {}>", struct_of_path(&name, m)));
                    }
                    let ret_type = make_tuple(ret_types);
                    let sugg = if let FnRetTy::Return(_) = decl.output {
                        ret_type
                    } else {
                        format!("-> {} ", ret_type)
                    };
                    add_replacement(ctx, decl.output.span(), sugg);

                    if matches!(decl.output, FnRetTy::Return(_)) {
                        return;
                    }

                    let ret_vals: Vec<_> = ret.iter().map(|m| m.guard()).collect();
                    self.use_guards(name.clone(), &ret_vals);
                    let ret_val = make_tuple(ret_vals);
                    let span = span.with_hi(span.hi() - BytePos(1)).shrink_to_hi();
                    add_replacement(ctx, span, format!("    {}\n", ret_val));
                }
            }
            _ => (),
        }
    }

    fn check_fn_post(
        &mut self,
        ctx: &LateContext<'tcx>,
        kind: intravisit::FnKind<'tcx>,
        _: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        _: Span,
        _: HirId,
    ) {
        match kind {
            intravisit::FnKind::ItemFn(id, _, _) => {
                let name = id.name.to_ident_string();
                let empty = vec![];
                let entry = if let Some(fs) = function_mutex_map().get(&name) {
                    &fs.entry_mutex
                } else {
                    &empty
                };
                let entry: BTreeSet<_> = entry.iter().map(|m| m.guard()).collect();
                let mut guards = self.guard_map.get(&name).cloned().unwrap_or_default();
                guards.sort();
                guards.dedup();
                let local_vars: String = guards
                    .iter()
                    .filter(|m| !entry.contains(*m))
                    .map(|m| format!("\n    let mut {};", m))
                    .collect();
                if !local_vars.is_empty() {
                    let span = body.value.span;
                    let span = span
                        .with_lo(span.lo() + BytePos(1))
                        .with_hi(span.lo() + BytePos(1));
                    add_replacement(ctx, span, format!("{}\n", local_vars));
                }
            }
            _ => (),
        }
    }

    fn check_expr(&mut self, ctx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) {
        if self.replaced.iter().any(|s| s.overlaps(e.span)) {
            return;
        }
        let func_name_opt = current_function(ctx, e.hir_id);
        let func_name = || func_name_opt.as_ref().unwrap().clone();
        let empty_summary = FunctionSummary::default();
        let func_summary = || {
            function_mutex_map()
                .get(&func_name())
                .unwrap_or(&empty_summary)
        };
        let is_protected = |mutex: &ExprPath| {
            if func_name_opt.is_none() {
                false
            } else if let Some(lines) = func_summary().mutex_line.get(mutex) {
                let expr_lines = span_lines(ctx, e.span);
                expr_lines.iter().any(|l| lines.contains(l))
            } else {
                false
            }
        };
        match &e.kind {
            ExprKind::Call(func, args) => {
                let f = name(func);
                let arg = |idx| normalize_arg(ctx, &args[idx]);
                match f.as_deref() {
                    Some("pthread_mutex_init" | "pthread_spin_init") => {
                        let mut path = expr_to_path(ctx, &args[0]).unwrap();
                        if global_def_map().contains_key(&path.base) {
                            add_replacement(ctx, e.span, "0".to_string());
                        } else {
                            let func = func_name();
                            let f = loop {
                                if let ExprPathProj::Field(f) = path.pop().unwrap() {
                                    break f;
                                }
                            };
                            let typ = get_type(&path, &func_name());
                            let empty = BTreeMap::new();
                            let map = struct_mutex_map().get(typ).unwrap_or(&empty);
                            let init_map = init_map();
                            let struct_map = struct_def_map();
                            let init = join(
                                map.iter()
                                    .filter_map(|(x, m)| {
                                        let default = default_value(
                                            struct_map.get(typ).unwrap().get(x).unwrap(),
                                        );
                                        if *m == f {
                                            let i = init_map
                                                .get(&(func.clone(), path.clone(), x.clone()))
                                                .unwrap_or(&default);
                                            Some(format!("{}: {}", x, i))
                                        } else {
                                            None
                                        }
                                    })
                                    .collect(),
                                ", ",
                            );
                            let st = format!("{} {{ {} }}", struct_of2(typ, &f), init);
                            let m = unwrap_addr(unwrap_cast_recursively(&args[0]));
                            let m = span_to_string(ctx, m.span);
                            let new_init = if in_assignment(ctx, e, false) {
                                format!("{{ {} = Mutex::new({}); 0 }}", m, st)
                            } else {
                                format!("{} = Mutex::new({})", m, st)
                            };
                            add_replacement(ctx, e.span, new_init);
                        }
                    }
                    Some("pthread_mutex_destroy" | "pthread_spin_destroy") => {
                        add_replacement(ctx, e.span, "0".to_string());
                    }
                    Some("pthread_mutex_lock" | "pthread_spin_lock") => {
                        let (arg, guard) = arg(0);
                        if in_assignment(ctx, e, false) {
                            let new_e = format!(
                                "{{
        {0}_opt = {1}.lock().ok();
        if {0}_opt.is_some() {{ 0 }} else {{ libc::ENOTRECOVERABLE }}
    }}",
                                guard, arg
                            );
                            add_replacement(ctx, e.span, new_e);
                            self.use_guard(func_name(), format!("{}_opt", guard));
                        } else {
                            add_replacement(
                                ctx,
                                e.span,
                                format!("{} = {}.lock().unwrap()", guard, arg),
                            );
                            self.use_guard(func_name(), guard);
                        }
                    }
                    Some("pthread_mutex_trylock" | "pthread_spin_trylock") => {
                        let (arg, guard) = arg(0);
                        let new_e = format!(
                            "{{
        {0}_opt = {1}.try_lock().ok();
        if {0}_opt.is_some() {{ 0 }} else {{ libc::EBUSY }}
    }}",
                            guard, arg
                        );
                        add_replacement(ctx, e.span, new_e);
                        self.use_guard(func_name(), format!("{}_opt", guard));
                    }
                    Some("pthread_mutex_unlock" | "pthread_spin_unlock") => {
                        let guard = arg(0).1;
                        self.use_guard(func_name(), guard.clone());
                        let new_e = if in_assignment(ctx, e, false) {
                            format!("{{ drop({}); 0 }}", guard)
                        } else {
                            format!("drop({})", guard)
                        };
                        add_replacement(ctx, e.span, new_e);
                    }
                    Some("pthread_cond_init") => {
                        let new_e = if in_assignment(ctx, e, false) {
                            format!("{{ {} = Condvar::new(); 0 }}", arg(0).0)
                        } else {
                            format!("{} = Condvar::new()", arg(0).0)
                        };
                        add_replacement(ctx, e.span, new_e);
                    }
                    Some("pthread_cond_destroy") => {
                        add_replacement(ctx, e.span, "0".to_string());
                    }
                    Some("pthread_cond_wait") => {
                        let c = arg(0).0;
                        let g = arg(1).1;
                        self.use_guard(func_name(), g.clone());
                        add_replacement(
                            ctx,
                            e.span,
                            format!("{{ {1} = {0}.wait({1}).unwrap(); 0 }}", c, g),
                        );
                    }
                    Some("pthread_cond_timedwait") => {
                        let c = arg(0).0;
                        let g = arg(1).1;
                        let t = arg(2).0;
                        self.use_guard(func_name(), g.clone());
                        let duration_map = duration_map();
                        let f = func_name();
                        let zero = "0".to_string();
                        let tv_sec = duration_map
                            .get(&(f.clone(), t.clone(), "tv_sec".to_string()))
                            .unwrap_or(&zero);
                        let tv_nsec = duration_map
                            .get(&(f, t, "tv_nsec".to_string()))
                            .unwrap_or(&zero);
                        add_replacement(
                            ctx,
                            e.span,
                            format!(
                                "{{
        let {1}_tmp = {0}.wait_timeout({1}, Duration::new({2} as u64, {3} as u32)).unwrap();
        {1} = {1}_tmp.0;
        if {1}_tmp.1.timed_out() {{ libc::ETIMEDOUT }} else {{ 0 }}
    }}",
                                c, g, tv_sec, tv_nsec
                            ),
                        );
                    }
                    Some("pthread_cond_signal") => {
                        let new_e = if in_assignment(ctx, e, false) {
                            format!("{{ {}.notify_one(); 0 }}", arg(0).0)
                        } else {
                            format!("{}.notify_one()", arg(0).0)
                        };
                        add_replacement(ctx, e.span, new_e);
                    }
                    Some("pthread_cond_broadcast") => {
                        let new_e = if in_assignment(ctx, e, false) {
                            format!("{{ {}.notify_all(); 0 }}", arg(0).0)
                        } else {
                            format!("{}.notify_all()", arg(0).0)
                        };
                        add_replacement(ctx, e.span, new_e);
                    }
                    Some(f) => {
                        if f == "main_0" {
                            return;
                        }
                        if let Some(FunctionSummary {
                            entry_mutex: entry,
                            ret_mutex: ret,
                            ..
                        }) = function_mutex_map().get(f)
                        {
                            let params = params_map().get(f).unwrap();
                            // param-to-arg aliasing
                            let alias_mutex = |m: &ExprPath| {
                                let mut m = m.clone();
                                if m.is_variable() {
                                    return m;
                                }
                                let (i, _) = some_or!(
                                    params.iter().enumerate().find(|(_, p)| &m.base == *p),
                                    return m
                                );
                                let arg = some_or!(expr_to_path(ctx, &args[i]), return m);
                                m.set_base(&arg);
                                m
                            };
                            if !entry.is_empty() {
                                let guards = entry.iter().map(|m| alias_mutex(m).guard()).collect();
                                self.use_guards(func_name(), &guards);
                                let guards = join(guards, ", ");
                                if let Some(arg) = args.last() {
                                    let span = arg.span.shrink_to_hi();
                                    add_replacement(ctx, span, format!(", {}", guards));
                                } else {
                                    let span = ctx
                                        .sess()
                                        .source_map()
                                        .span_through_char(e.span, '(')
                                        .shrink_to_hi();
                                    add_replacement(ctx, span, guards);
                                }
                            }
                            if !ret.is_empty() {
                                let guards: Vec<_> =
                                    ret.iter().map(|m| alias_mutex(m).guard()).collect();
                                self.use_guards(func_name(), &guards);
                                if type_of(ctx, e.hir_id).is_unit() {
                                    if ret.len() == 1 {
                                        let span = e.span.shrink_to_lo();
                                        let guard = &guards[0];
                                        add_replacement(ctx, span, format!("{} = ", guard));
                                    } else {
                                        let span = e.span.shrink_to_lo();
                                        add_replacement(
                                            ctx,
                                            span,
                                            format!("{{ let {}_res_tmp = ", f),
                                        );
                                        let span = e.span.shrink_to_hi();
                                        let guards: String = guards
                                            .iter()
                                            .enumerate()
                                            .map(|(i, g)| format!("{} = {}_res_tmp.{}; ", g, f, i))
                                            .collect();
                                        add_replacement(ctx, span, format!("; {} }}", guards));
                                    }
                                } else {
                                    let span = e.span.shrink_to_lo();
                                    add_replacement(ctx, span, format!("{{ let {}_res_tmp = ", f));
                                    let span = e.span.shrink_to_hi();
                                    let guards: String = guards
                                        .iter()
                                        .enumerate()
                                        .map(|(i, g)| format!("{} = {}_res_tmp.{}; ", g, f, i + 1))
                                        .collect();
                                    add_replacement(
                                        ctx,
                                        span,
                                        format!("; {}{}_res_tmp.0 }}", guards, f),
                                    );
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
            ExprKind::Ret(v_opt) => {
                let f = func_name();
                if f == "main_0" {
                    return;
                }
                let ret = &func_summary().ret_mutex;
                if !ret.is_empty() {
                    let ret_vals = ret.iter().map(|m| m.guard()).collect();
                    self.use_guards(f, &ret_vals);
                    match v_opt {
                        Some(v) => {
                            let ret_val = join(ret_vals, ", ");
                            add_replacement(ctx, v.span.shrink_to_lo(), "(".to_string());
                            add_replacement(ctx, v.span.shrink_to_hi(), format!(", {})", ret_val));
                        }
                        None => {
                            let ret_val = make_tuple(ret_vals);
                            add_replacement(ctx, e.span.shrink_to_hi(), format!(" {}", ret_val));
                        }
                    }
                }
            }
            ExprKind::Struct(_, fs, _) => {
                let typ = type_to_string(type_of(ctx, e.hir_id));
                let empty = BTreeMap::new();
                let map = struct_mutex_map().get(&typ).unwrap_or(&empty);
                let struct_def_map = struct_def_map();
                let v: Vec<_> = map
                    .iter()
                    .map(|(x, m)| {
                        (
                            x.clone(),
                            struct_def_map.get(&typ).unwrap().get(x).unwrap().clone(),
                            m.clone(),
                        )
                    })
                    .collect();
                let init_map: BTreeMap<_, _> = fs
                    .iter()
                    .map(|f| {
                        (
                            f.ident.name.to_ident_string(),
                            span_to_string(ctx, f.expr.span),
                        )
                    })
                    .collect();
                for f in fs.iter() {
                    let name = f.ident.name.to_ident_string();
                    let ftyp = type_to_string(type_of(ctx, f.expr.hir_id));
                    if ftyp.contains("pthread_cond_t") {
                        add_replacement(ctx, f.expr.span, "Condvar::new()".to_string());
                        continue;
                    }
                    if v.iter().any(|(x, _, _)| *x == name) {
                        let span = f.span.with_hi(f.span.hi() + BytePos(1));
                        add_replacement(ctx, span, "".to_string());
                        continue;
                    }
                    if ftyp.contains("pthread_mutex_t") || ftyp.contains("pthread_spinlock_t") {
                        let pfs: Vec<_> = v
                            .iter()
                            .filter_map(|(x, _, m)| {
                                if *m == name {
                                    let i = init_map.get(x).unwrap();
                                    Some(format!("{}: {}", x, i))
                                } else {
                                    None
                                }
                            })
                            .collect();
                        let st_name = struct_of2(&typ, &name);
                        let st_body = join(pfs, ", ");
                        let st = format!("{} {{ {} }}", st_name, st_body);
                        let ini = format!("Mutex::new({})", st);
                        add_replacement(ctx, f.expr.span, ini);
                    }
                }
            }
            // global variable
            ExprKind::Path(_) => {
                if let Some(x) = name(e) {
                    if let Some(m) = global_mutex_map().get(&x) {
                        let mutex = ExprPath::new(m.clone(), vec![]);
                        let new_e = if is_protected(&mutex) {
                            let guard = mutex.guard();
                            self.use_guard(func_name(), guard.clone());
                            format!("(*{}).{}", guard, x)
                        } else {
                            format!("{}.get_mut().unwrap().{}", m, x)
                        };
                        add_replacement(ctx, e.span, new_e);
                    }
                }
            }
            // array
            ExprKind::Index(a, i) => {
                if let Some(a) = name(a) {
                    let ind = unwrap_cast_recursively(i);
                    let ind = span_to_string(ctx, ind.span);
                    if let Some(m) = array_mutex_map().get(&a) {
                        let mutex = ExprPath::new(m.clone(), vec![ExprPathProj::Index(ind)]);
                        let new_e = if is_protected(&mutex) {
                            let guard = mutex.guard();
                            self.use_guard(func_name(), guard.clone());
                            format!("(*{}).{}", guard, a)
                        } else {
                            let i = span_to_string(ctx, i.span);
                            format!("{}[{}].get_mut().unwrap().{}", m, i, a)
                        };
                        add_replacement(ctx, e.span, new_e);
                    }
                }
            }
            // struct
            ExprKind::Field(s, f) => {
                let ty = type_to_string(unwrap_ptr_from_type(type_of(ctx, s.hir_id)));
                let f = f.name.to_ident_string();
                let map = some_or!(struct_mutex_map().get(&ty), return);
                let m = some_or!(map.get(&f), return);
                let s_path_opt = expr_to_path(ctx, s);
                let mut mutex = s_path_opt.unwrap();
                mutex.add_suffix(ExprPathProj::Field(m.clone()));
                if is_protected(&mutex) {
                    let guard = mutex.guard();
                    self.use_guard(func_name(), guard.clone());
                    let new_e = format!("(*{}).{}", guard, f);
                    add_replacement(ctx, e.span, new_e);
                } else {
                    let assigned = in_assignment(ctx, e, true);
                    let path = expr_to_path(ctx, s);
                    let initialized = if let Some(path) = path {
                        mutex_init_map().contains(&(func_name(), path, m.clone()))
                    } else {
                        false
                    };
                    if assigned && initialized {
                        return;
                    }
                    let s = span_to_string(ctx, s.span);
                    let new_e = format!("{}.{}.get_mut().unwrap().{}", s, m, f);
                    add_replacement(ctx, e.span, new_e);
                }
            }
            ExprKind::Assign(lhs, _, _) => {
                if let Some(mut path) = expr_to_path(ctx, lhs) {
                    while path.pop().is_some() {
                        let ty = get_type(&path, &func_name());
                        if ty.contains("pthread_cond_t") {
                            add_replacement(ctx, e.span, "()".to_string());
                        }
                    }
                }
                match lhs.kind {
                    ExprKind::Field(s, f) => {
                        let ty = type_to_string(type_of(ctx, s.hir_id));
                        let f = f.name.to_ident_string();
                        let map = some_or!(struct_mutex_map().get(&ty), return);
                        let m = some_or!(map.get(&f), return);
                        let s_path_opt = expr_to_path(ctx, s);
                        let mut mutex = s_path_opt.unwrap();
                        mutex.add_suffix(ExprPathProj::Field(m.clone()));
                        if is_protected(&mutex) {
                            return;
                        }
                        let s = some_or!(expr_to_path(ctx, s), return);
                        if mutex_init_map().contains(&(func_name(), s, m.clone())) {
                            add_replacement(ctx, e.span, "()".to_string());
                        }
                    }
                    _ => (),
                }
            }
            ExprKind::If(c, t, f) => {
                let (expr, eq) = some_or!(read_condition(ctx, c), return);
                let line = span_lines(ctx, c.span)
                    .drain_filter(|_| true)
                    .min()
                    .unwrap();
                let g = some_or!(trylock_map().get(&(func_name(), expr, line)), return);
                let span = c
                    .span
                    .with_lo(c.span.lo() - BytePos(3))
                    .with_hi(c.span.hi() + BytePos(2));
                let true_branch = if eq {
                    format!("Some({0}_tmp) => {{ {0} = {0}_tmp;", g)
                } else {
                    "None => {".to_string()
                };
                add_replacement(ctx, span, format!("match {}_opt {{ {}", g, true_branch));
                if let Some(f) = f {
                    let span = t
                        .span
                        .with_lo(t.span.hi())
                        .with_hi(f.span.lo() + BytePos(1));
                    let false_branch = if eq {
                        "None => {".to_string()
                    } else {
                        format!("Some({0}_tmp) => {{ {0} = {0}_tmp;", g)
                    };
                    add_replacement(ctx, span, false_branch);

                    let span = f.span.with_lo(f.span.hi());
                    add_replacement(ctx, span, "}".to_string());
                } else {
                    let span = t.span.with_lo(t.span.hi());
                    let false_branch = if eq {
                        "None => {} }".to_string()
                    } else {
                        format!("Some({0}_tmp) => {{ {0} = {0}_tmp; }} }}", g)
                    };
                    add_replacement(ctx, span, false_branch);
                }
            }
            ExprKind::MethodCall(method, args, _) => {
                // ((*a).b[(*a).c as usize]).as_mut_ptr()
                // ==>
                // {
                //     let c_index_tmp = (*guard).c;
                //     (*guard).b[c_index_tmp as usize].as_mut_ptr()
                // }
                if method.ident.to_string() != "as_mut_ptr" {
                    return;
                }
                let (arr, ind) = if let ExprKind::Index(arr, ind) = &args[0].kind {
                    (arr, ind)
                } else {
                    return;
                };
                let (s1, f1) = if let ExprKind::Field(s, f) = &arr.kind {
                    (s, f)
                } else {
                    return;
                };
                let ind = unwrap_cast_recursively(ind);
                let (s2, f2) = if let ExprKind::Field(s, f) = &ind.kind {
                    (s, f)
                } else {
                    return;
                };
                if span_to_string(ctx, s1.span) != span_to_string(ctx, s2.span) {
                    return;
                }
                let ty = type_to_string(unwrap_ptr_from_type(type_of(ctx, s1.hir_id)));
                let map = some_or!(struct_mutex_map().get(&ty), return);
                let m1 = some_or!(map.get(&f1.to_string()), return);
                let m2 = some_or!(map.get(&f2.to_string()), return);
                if m1 != m2 {
                    return;
                }
                let mut mutex = expr_to_path(ctx, s1).unwrap();
                mutex.add_suffix(ExprPathProj::Field(m1.clone()));
                if !is_protected(&mutex) {
                    return;
                }
                let guard = mutex.guard();
                self.use_guard(func_name(), guard.clone());
                let new_e = format!(
                    "{{
        let {0}_index_tmp = (*{1}).{0};
        (*{1}).{2}[{0}_index_tmp as usize].as_mut_ptr()
    }}",
                    f2, guard, f1
                );
                add_replacement(ctx, e.span, new_e);
                self.replaced.insert(e.span);
            }
            _ => (),
        }
    }
}

// suggestions for left-hand side of expressions
fn add_replacement(ctx: &LateContext<'_>, span: Span, replacement: String) {
    use rustfix::{LinePosition, LineRange};

    let source_map = ctx.sess().source_map();
    let fname = source_map.span_to_filename(span);

    let file = source_map.get_source_file(&fname).unwrap();
    // get the line and the column numbers
    let lo = file.lookup_file_pos_with_col_display(span.lo());
    let hi = file.lookup_file_pos_with_col_display(span.hi());
    let line_range = LineRange {
        start: LinePosition {
            line: lo.0,
            column: lo.2,
        },
        end: LinePosition {
            line: hi.0,
            column: hi.2,
        },
    };
    // get the char offsets within the file
    let lo_offset = file.original_relative_byte_pos(span.lo()).0;
    let hi_offset = file.original_relative_byte_pos(span.hi()).0;
    let snippet = Snippet {
        file_name: fname.prefer_remapped().to_string(),
        line_range,
        range: (lo_offset as usize)..(hi_offset as usize),
        text: (
            "".into(),
            source_map.span_to_snippet(span).unwrap(),
            "".into(),
        ),
    };

    REPLACEMENTS.lock().unwrap().push(Replacement {
        snippet,
        replacement,
    });
}

fn hid_to_string(ctx: &LateContext<'_>, hid: HirId) -> String {
    span_to_string(ctx, ctx.tcx.hir().span(hid))
}

fn current_function(ctx: &LateContext<'_>, hid: HirId) -> Option<String> {
    let hir = ctx.tcx.hir();
    if let Node::Item(item) = hir.get(hir.enclosing_body_owner(hid)) {
        if let ItemKind::Fn(_, _, _) = item.kind {
            Some(item.ident.name.to_ident_string())
        } else {
            None
        }
    } else {
        None
    }
}

fn remove_attributes(ctx: &LateContext<'_>, i: &Item<'_>) {
    let hir = ctx.tcx.hir();
    let attrs = hir.attrs(hir.local_def_id_to_hir_id(i.def_id));
    for a in attrs {
        add_replacement(ctx, a.span, "".to_string());
    }
}

fn path_to_symbol<'tcx>(p: &'tcx Path<'tcx>) -> Symbol {
    p.segments.last().unwrap().ident.name
}

fn path_to_string<'tcx>(p: &'tcx Path<'tcx>) -> String {
    path_to_symbol(p).to_ident_string()
}

fn name_symbol<'tcx>(e: &'tcx Expr<'tcx>) -> Option<Symbol> {
    match &e.kind {
        ExprKind::Path(QPath::Resolved(_, p)) => Some(path_to_symbol(p)),
        _ => None,
    }
}

fn name<'tcx>(e: &'tcx Expr<'tcx>) -> Option<String> {
    name_symbol(e).map(|s| s.to_ident_string())
}

fn unwrap_addr<'a, 'tcx>(e: &'a Expr<'tcx>) -> &'a Expr<'tcx> {
    match &e.kind {
        ExprKind::AddrOf(_, _, e) => e,
        _ => e,
    }
}

fn normalize_arg<'a, 'b, 'tcx>(ctx: &'a LateContext<'b>, e: &'tcx Expr<'tcx>) -> (String, String) {
    let path = expr_to_path(ctx, e).unwrap();
    let guard = path.guard();
    let arg = unwrap_addr(unwrap_cast_recursively(e));
    match &arg.kind {
        ExprKind::Unary(
            UnOp::Deref,
            Expr {
                kind: ExprKind::MethodCall(method, args, _),
                ..
            },
        ) => {
            assert_eq!(method.ident.name.to_ident_string(), "offset");
            assert_eq!(args.len(), 2);
            let arr = match &args[0].kind {
                ExprKind::MethodCall(method, args, _) => {
                    assert_eq!(method.ident.name.to_ident_string(), "as_mut_ptr");
                    assert_eq!(args.len(), 1);
                    name(&args[0]).unwrap()
                }
                _ => unreachable!(),
            };
            let ind = unwrap_cast_recursively(&args[1]);
            let ind = span_to_string(ctx, ind.span);
            let arg = format!("{}[{} as usize]", arr, ind);
            (arg, guard)
        }
        _ => {
            let arg = span_to_string(ctx, arg.span);
            (arg, guard)
        }
    }
}

fn in_assignment<'tcx>(ctx: &LateContext<'tcx>, e: &Expr<'tcx>, lhs: bool) -> bool {
    let hir = ctx.tcx.hir();
    if let Node::Expr(parent) = hir.get(hir.get_parent_node(e.hir_id)) {
        if let ExprKind::Assign(l, r, _) = &parent.kind {
            if lhs {
                l.hir_id == e.hir_id
            } else {
                r.hir_id == e.hir_id
            }
        } else {
            false
        }
    } else {
        false
    }
}

fn read_condition<'a, 'tcx>(ctx: &LateContext<'tcx>, e: &'a Expr<'tcx>) -> Option<(String, bool)> {
    let e = match e.kind {
        ExprKind::DropTemps(e) => e,
        _ => e,
    };
    let (e, not) = match e.kind {
        ExprKind::Unary(op, e) => {
            if op == UnOp::Not {
                (e, true)
            } else {
                return None;
            }
        }
        _ => (e, false),
    };
    match e.kind {
        ExprKind::Binary(op, lhs, rhs) => {
            let eq = match op.node {
                BinOpKind::Eq => true,
                BinOpKind::Ne => false,
                _ => return None,
            };
            let lhs = unwrap_cast_recursively(lhs);
            let rhs = unwrap_cast_recursively(rhs);
            let e = if span_to_string(ctx, lhs.span) == "0" {
                rhs
            } else if span_to_string(ctx, rhs.span) == "0" {
                lhs
            } else {
                return None;
            };
            Some((span_to_string(ctx, e.span), not ^ eq))
        }
        _ => None,
    }
}

fn path_to_id(p: &str) -> String {
    p.split(&[' ', '-', '>', '(', ')', '[', ']', '.', '*', '&'])
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

fn struct_of(m: &str) -> String {
    format!("{}Data", path_to_id(m))
}

fn struct_of2(s: &str, m: &str) -> String {
    format!("{}{}Data", path_to_id(s), path_to_id(m))
}

fn struct_of_path(func: &String, s: &ExprPath) -> String {
    if s.is_variable() {
        struct_of(&s.base)
    } else {
        let mut s = s.clone();
        let f = s.pop().unwrap();
        let ty = get_type(&s, func);
        struct_of2(ty, f.inner())
    }
}

fn get_type(s: &ExprPath, f: &String) -> &'static String {
    let map = path_type_map().get(s).unwrap();
    map.get(f).unwrap_or_else(|| map.iter().next().unwrap().1)
}

fn default_value(typ: &str) -> String {
    if typ.contains("*mut") {
        "0 as *mut _".to_string()
    } else if typ.contains("int") || typ.contains("size") {
        "0".to_string()
    } else {
        format!("std::mem::transmute([0u8; std::mem::size_of::<{}>()])", typ)
    }
}

fn make_tuple(mut v: Vec<String>) -> String {
    assert!(!v.is_empty());
    if v.len() == 1 {
        v.pop().unwrap()
    } else {
        format!("({})", join(v, ", "))
    }
}
