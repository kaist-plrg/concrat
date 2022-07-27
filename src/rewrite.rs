use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::File,
    io::Read,
    sync::Mutex,
};

use lazy_static::lazy_static;
use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass, LintContext, LintPass};
use rustc_span::{BytePos, Span, Symbol};
use rustfix::{Replacement, Snippet, Solution, Suggestion};
use spin::once::Once;

use crate::{
    analysis::{AnalysisSummary, FunctionSummary},
    callback::{compile_with, LatePass},
};

lazy_static! {
    static ref REPLACEMENTS: Mutex<Vec<Replacement>> = Mutex::new(vec![]);
    static ref GLOBAL_DEF_MAP: Mutex<HashMap<String, (String, String)>> =
        Mutex::new(HashMap::default());
    static ref ARRAY_DEF_MAP: Mutex<HashMap<String, (String, Vec<String>)>> =
        Mutex::new(HashMap::default());
    static ref STRUCT_DEF_MAP: Mutex<HashMap<String, HashMap<String, String>>> =
        Mutex::new(HashMap::default());
    static ref INIT_MAP: Mutex<HashMap<(String, String, String), String>> =
        Mutex::new(HashMap::default());
    static ref MUTEX_INIT_MAP: Mutex<HashSet<(String, String, String)>> =
        Mutex::new(HashSet::default());
    static ref GUARD_MAP: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::default());
    static ref ID_TYPE_MAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::default());
    static ref DURATION_MAP: Mutex<HashMap<(String, String, String), String>> =
        Mutex::new(HashMap::default());
    static ref TRYLOCK_MAP: Mutex<HashMap<(String, String), (String, String)>> =
        Mutex::new(HashMap::default());
}

static SUMMARY: Once<AnalysisSummary> = Once::new();

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

struct GlobalPass;

impl GlobalPass {
    #[allow(clippy::new_ret_no_self)]
    fn new() -> Box<LatePass> {
        Box::new(Self)
    }
}

impl LintPass for GlobalPass {
    fn name(&self) -> &'static str {
        "GlobalPass"
    }
}

impl<'tcx> LateLintPass<'tcx> for GlobalPass {
    fn check_item(&mut self, ctx: &LateContext<'tcx>, i: &'tcx Item<'tcx>) {
        match &i.kind {
            ItemKind::Struct(VariantData::Struct(fs, _), _) => {
                let s = i.ident.name.to_ident_string();
                let map = fs
                    .iter()
                    .map(|f| {
                        (
                            f.ident.name.to_ident_string(),
                            span_to_string(ctx, f.ty.span),
                        )
                    })
                    .collect();
                STRUCT_DEF_MAP.lock().unwrap().insert(s, map);
            }
            ItemKind::Static(t, _, b) => {
                let name = i.ident.name.to_ident_string();
                match t.kind {
                    // array
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
                        ARRAY_DEF_MAP.lock().unwrap().insert(name, (ty, init));
                    }
                    // global
                    _ => {
                        let ty = span_to_string(ctx, t.span);
                        let init = hid_to_string(ctx, b.hir_id);
                        GLOBAL_DEF_MAP.lock().unwrap().insert(name, (ty, init));
                    }
                }
            }
            _ => (),
        }
    }

    fn check_expr(&mut self, ctx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) {
        let func_name = || current_function(ctx, e.hir_id).unwrap();
        match e.kind {
            ExprKind::Call(func, args) => {
                let f = name(func);
                match f.as_deref() {
                    Some("pthread_mutex_init") => {
                        let m = unwrap_addr(unwrap_cast_recursively(&args[0])).unwrap();
                        match m.kind {
                            ExprKind::Field(s, f) => {
                                let func = func_name();
                                let s = normalize_path(&span_to_string(ctx, s.span));
                                let f = f.name.to_ident_string();
                                MUTEX_INIT_MAP.lock().unwrap().insert((func, s, f));
                            }
                            _ => unreachable!(),
                        }
                    }
                    _ => (),
                }
            }
            ExprKind::Assign(lhs, rhs, _) => {
                match lhs.kind {
                    ExprKind::Field(s, f) => {
                        let func = func_name();
                        let s = normalize_path(&span_to_string(ctx, s.span));
                        let f = f.name.to_ident_string();
                        let i = span_to_string(ctx, rhs.span);
                        INIT_MAP.lock().unwrap().insert((func, s, f), i);
                    }
                    _ => (),
                }
                match rhs.kind {
                    ExprKind::Call(func, args) => {
                        if let Some(f) = name(func) {
                            if f == "pthread_mutex_trylock" {
                                let f = func_name();
                                let l = span_to_string(ctx, lhs.span);
                                let (a, g) = normalize_arg(ctx, &args[0]);
                                TRYLOCK_MAP.lock().unwrap().insert((f, l), (a, g));
                            }
                        }
                    }
                    _ => (),
                }
            }
            ExprKind::AssignOp(op, lhs, rhs) => match op.node {
                BinOpKind::Add => match lhs.kind {
                    ExprKind::Field(s, f) => {
                        if type_as_string(ctx, s) == "timespec" {
                            let func = func_name();
                            let s = span_to_string(ctx, s.span);
                            let f = span_to_string(ctx, f.span);
                            let r = span_to_string(ctx, rhs.span);
                            DURATION_MAP.lock().unwrap().insert((func, s, f), r);
                        }
                    }
                    _ => (),
                },
                _ => (),
            },
            ExprKind::Path(_) => {
                if let (Some(_f), Some(n)) = (current_function(ctx, e.hir_id), name(e)) {
                    let ty = type_as_string(ctx, e);
                    if !ty.contains("fn(") {
                        ID_TYPE_MAP.lock().unwrap().insert(n, ty);
                    }
                }
            }
            _ => (),
        }
    }
}

struct RewritePass;

impl RewritePass {
    #[allow(clippy::new_ret_no_self)]
    fn new() -> Box<LatePass> {
        Box::new(Self)
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
                    if let Some(map) = STRUCT_DEF_MAP.lock().unwrap().get(&s) {
                        if map
                            .iter()
                            .any(|(_, t)| t == "pthread_mutex_t" || t == "pthread_cond_t")
                        {
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
                let struct_def_map = STRUCT_DEF_MAP.lock().unwrap();
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
                    if typ == "pthread_mutex_t" {
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
                        if pfs.is_empty() {
                            add_replacement(ctx, f.ty.span, "Mutex<()>".to_string());
                        } else {
                            let st_name = struct_of2(&s, &name);
                            add_replacement(ctx, f.ty.span, format!("Mutex<{}>", st_name));
                            let st_body = join(pfs, ", ");
                            let st = format!("\npub struct {} {{ {} }}", st_name, st_body);
                            new_structs.push_str(&st);
                        }
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
                if typ == "pthread_mutex_t" {
                    let mut decl = String::new();
                    let mut init = String::new();
                    for (x, m) in global_mutex_map() {
                        if *m == name {
                            let (t, i) = GLOBAL_DEF_MAP.lock().unwrap().get(x).unwrap().clone();
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
                    if span_to_string(ctx, t.span) == "pthread_mutex_t" {
                        let v: Vec<_> = array_mutex_map()
                            .iter()
                            .filter_map(|(x, m)| {
                                if **m == name {
                                    let (t, i) =
                                        ARRAY_DEF_MAP.lock().unwrap().get(x).unwrap().clone();
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
                let name = &id.name.to_ident_string();
                if name == "main" {
                    return;
                }
                let name = correct_function_name(name);

                let empty = vec![];
                let (entry, _, ret) = if let Some(fs) = function_mutex_map().get(&name) {
                    let FunctionSummary {
                        entry_mutex,
                        node_mutex,
                        ret_mutex,
                    } = fs;
                    (entry_mutex, node_mutex, ret_mutex)
                } else {
                    (&empty, &empty, &empty)
                };

                let b = if let ExprKind::Block(b, _) = body.value.kind {
                    b
                } else {
                    unreachable!()
                };
                let stmts = &b.stmts;

                if !entry.is_empty() {
                    let params = entry
                        .iter()
                        .map(|m| {
                            format!(
                                "mut {}: MutexGuard<'static, {}>",
                                guard_of(m),
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

                if !ret.is_empty() {
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

                    assert!(b.expr.is_none());
                    let last = stmts.last().unwrap();
                    if let StmtKind::Semi(e) = last.kind {
                        match e.kind {
                            ExprKind::Ret(_) => (),
                            _ => {
                                let ret_vals = ret.iter().map(guard_of).collect();
                                use_guards(name.clone(), &ret_vals);
                                let ret_val = make_tuple(ret_vals);
                                add_replacement(
                                    ctx,
                                    last.span.shrink_to_hi(),
                                    format!("\n    {}", ret_val),
                                );
                            }
                        }
                    } else {
                        unreachable!();
                    }
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
                let name = &id.name.to_ident_string();
                if name == "main" {
                    return;
                }
                let name = correct_function_name(name);
                let empty = vec![];
                let entry = if let Some(fs) = function_mutex_map().get(&name) {
                    &fs.entry_mutex
                } else {
                    &empty
                };
                let entry: HashSet<_> = entry.iter().map(guard_of).collect();
                let mut guards = GUARD_MAP
                    .lock()
                    .unwrap()
                    .get(&name)
                    .cloned()
                    .unwrap_or_default();
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

    fn check_local(&mut self, ctx: &LateContext<'tcx>, l: &'tcx Local<'tcx>) {
        if let Some(f) = current_function(ctx, l.hir_id) {
            match &l.pat.kind {
                PatKind::Binding(_, _, x, _) => {
                    if TRYLOCK_MAP
                        .lock()
                        .unwrap()
                        .contains_key(&(f, x.name.to_ident_string()))
                    {
                        let span = l.ty.unwrap().span;
                        add_replacement(
                            ctx,
                            span.with_lo(span.lo() - BytePos(2))
                                .with_hi(span.hi() + BytePos(4)),
                            "".to_string(),
                        );
                    }
                }
                _ => (),
            }
        }
    }

    fn check_expr(&mut self, ctx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) {
        let func_name_opt = current_function(ctx, e.hir_id);
        let func_name = || func_name_opt.as_ref().unwrap().clone();
        let empty_summary = FunctionSummary {
            entry_mutex: vec![],
            node_mutex: vec![],
            ret_mutex: vec![],
        };
        let func_summary = || {
            function_mutex_map()
                .get(&func_name())
                .unwrap_or(&empty_summary)
        };
        match &e.kind {
            ExprKind::Call(func, args) => {
                let f = name(func);
                let arg = |idx| normalize_arg(ctx, &args[idx]);
                match f.as_deref() {
                    Some("pthread_mutex_init") => {
                        let m = unwrap_addr(unwrap_cast_recursively(&args[0])).unwrap();
                        match m.kind {
                            ExprKind::Field(s, f) => {
                                let func = func_name();
                                let typ = type_as_string(ctx, s);
                                let s = normalize_path(&span_to_string(ctx, s.span));
                                let f = f.name.to_ident_string();
                                if GLOBAL_DEF_MAP.lock().unwrap().contains_key(&s) {
                                    add_replacement(ctx, e.span, "0".to_string());
                                } else if let Some(map) = struct_mutex_map().get(&typ) {
                                    let zero = "0".to_string();
                                    let init_map = INIT_MAP.lock().unwrap();
                                    let init = join(
                                        map.iter()
                                            .filter_map(|(x, m)| {
                                                if *m == f {
                                                    let i = init_map
                                                        .get(&(func.clone(), s.clone(), x.clone()))
                                                        .unwrap_or(&zero);
                                                    Some(format!("{}: {}", x, i))
                                                } else {
                                                    None
                                                }
                                            })
                                            .collect(),
                                        ", ",
                                    );
                                    let st = format!("{} {{ {} }}", struct_of2(&typ, &f), init);
                                    let new_init = format!(
                                        "{{ {} = Mutex::new({}); 0 }}",
                                        span_to_string(ctx, m.span),
                                        st
                                    );
                                    add_replacement(ctx, e.span, new_init);
                                } else {
                                    let new_init = format!(
                                        "{{ {} = Mutex::new(()); 0 }}",
                                        span_to_string(ctx, m.span)
                                    );
                                    add_replacement(ctx, e.span, new_init);
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    Some("pthread_mutex_destroy") => {
                        add_replacement(ctx, e.span, "".to_string());
                    }
                    Some("pthread_mutex_lock") => {
                        let (arg, guard) = arg(0);
                        add_replacement(
                            ctx,
                            e.span,
                            format!("{} = {}.lock().unwrap()", guard, arg),
                        );
                    }
                    Some("pthread_mutex_trylock") => {
                        add_replacement(ctx, e.span, format!("{}.try_lock()", arg(0).0))
                    }
                    Some("pthread_mutex_unlock") => {
                        let guard = arg(0).1;
                        use_guard(func_name(), guard.clone());
                        add_replacement(ctx, e.span, format!("drop({})", guard));
                    }
                    Some("pthread_cond_init") => {
                        add_replacement(ctx, e.span, format!("{} = Condvar::new()", arg(0).0));
                    }
                    Some("pthread_cond_wait") => {
                        let c = arg(0).0;
                        let g = arg(1).1;
                        use_guard(func_name(), g.clone());
                        add_replacement(ctx, e.span, format!("{1} = {0}.wait({1}).unwrap()", c, g));
                    }
                    Some("pthread_cond_timedwait") => {
                        let c = arg(0).0;
                        let g = arg(1).1;
                        let t = arg(2).0;
                        use_guard(func_name(), g.clone());
                        let duration_map = DURATION_MAP.lock().unwrap();
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
        let tmp = {0}.wait_timeout({1}, Duration::new({2} as u64, {3} as u32)).unwrap();
        {1} = tmp.0;
        if tmp.1.timed_out() {{ libc::ETIMEDOUT }} else {{ 0 }}
    }}",
                                c, g, tv_sec, tv_nsec
                            ),
                        );
                    }
                    Some("pthread_cond_signal") => {
                        add_replacement(ctx, e.span, format!("{}.notify_one()", arg(0).0));
                    }
                    Some("pthread_cond_broadcast") => {
                        add_replacement(ctx, e.span, format!("{}.notify_all()", arg(0).0));
                    }
                    Some(f) => {
                        if let Some(FunctionSummary {
                            entry_mutex: entry,
                            ret_mutex: ret,
                            ..
                        }) = function_mutex_map().get(f)
                        {
                            if !entry.is_empty() {
                                let guards = entry.iter().map(guard_of).collect();
                                use_guards(func_name(), &guards);
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
                                if type_of(ctx, e).is_unit() {
                                    if ret.len() == 1 {
                                        let span = e.span.shrink_to_lo();
                                        let guard = guard_of(&ret[0]);
                                        use_guard(func_name(), guard.clone());
                                        add_replacement(ctx, span, format!("{} = ", guard));
                                    } else {
                                        let span = e.span.shrink_to_lo();
                                        add_replacement(ctx, span, "{ let tmp = ".to_string());
                                        let span = e.span.shrink_to_hi();
                                        for m in ret {
                                            use_guard(func_name(), guard_of(m));
                                        }
                                        let guards: String = ret
                                            .iter()
                                            .enumerate()
                                            .map(|(i, m)| format!("{} = tmp.{}; ", guard_of(m), i))
                                            .collect();
                                        add_replacement(ctx, span, format!("; {} }}", guards));
                                    }
                                } else {
                                    let span = e.span.shrink_to_lo();
                                    add_replacement(ctx, span, "{ let tmp = ".to_string());
                                    let span = e.span.shrink_to_hi();
                                    for m in ret {
                                        use_guard(func_name(), guard_of(m));
                                    }
                                    let guards: String = ret
                                        .iter()
                                        .enumerate()
                                        .map(|(i, m)| format!("{} = tmp.{}; ", guard_of(m), i + 1))
                                        .collect();
                                    add_replacement(ctx, span, format!("; {}tmp.0 }}", guards));
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
            ExprKind::Ret(v_opt) => {
                let ret = &func_summary().ret_mutex;
                if !ret.is_empty() {
                    let ret_vals = ret.iter().map(guard_of).collect();
                    use_guards(func_name(), &ret_vals);
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
                let typ = type_as_string(ctx, e);
                let empty = BTreeMap::new();
                let map = struct_mutex_map().get(&typ).unwrap_or(&empty);
                let struct_def_map = STRUCT_DEF_MAP.lock().unwrap();
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
                let init_map: HashMap<_, _> = fs
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
                    let ftyp = type_as_string(ctx, f.expr);
                    if ftyp.contains("pthread_cond_t") {
                        add_replacement(ctx, f.expr.span, "Condvar::new()".to_string());
                        continue;
                    }
                    if v.iter().any(|(x, _, _)| *x == name) {
                        let span = f.span.with_hi(f.span.hi() + BytePos(1));
                        add_replacement(ctx, span, "".to_string());
                        continue;
                    }
                    if ftyp.contains("pthread_mutex_t") {
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
                        if pfs.is_empty() {
                            add_replacement(ctx, f.expr.span, "Mutex::new(())".to_string());
                        } else {
                            let st_name = struct_of2(&typ, &name);
                            let st_body = join(pfs, ", ");
                            let st = format!("{} {{ {} }}", st_name, st_body);
                            let ini = format!("Mutex::new({})", st);
                            add_replacement(ctx, f.expr.span, ini);
                        }
                    }
                }
            }
            // global variable
            ExprKind::Path(_) => {
                if let Some(x) = name(e) {
                    if let Some(m) = global_mutex_map().get(&x) {
                        let new_e = if func_summary().node_mutex.contains(m) {
                            let guard = guard_of(m);
                            use_guard(func_name(), guard.clone());
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
                        let mutex = format!("{}[{}]", m, ind);
                        let new_e = if func_summary().node_mutex.contains(&mutex) {
                            let guard = guard_of(&mutex);
                            use_guard(func_name(), guard.clone());
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
                let ty = type_as_string(ctx, s).replace("&mut ", "").replace('&', "");
                let f = f.name.to_ident_string();
                if let Some(m) = struct_mutex_map().get(&ty).and_then(|m| m.get(&f)) {
                    let s = span_to_string(ctx, s.span);
                    let hir = ctx.tcx.hir();
                    let parent_is_assignment =
                        if let Node::Expr(e) = hir.get(hir.get_parent_node(e.hir_id)) {
                            matches!(e.kind, ExprKind::Assign(_, _, _))
                        } else {
                            false
                        };
                    if parent_is_assignment
                        && MUTEX_INIT_MAP.lock().unwrap().contains(&(
                            func_name(),
                            normalize_path(&s),
                            m.clone(),
                        ))
                    {
                        return;
                    }
                    let mutex = format!("{}.{}", normalize_path(&s), m);
                    let new_e = if func_summary().node_mutex.contains(&mutex) {
                        let guard = guard_of(&mutex);
                        use_guard(func_name(), guard.clone());
                        format!("(*{}).{}", guard, f)
                    } else {
                        format!("{}.{}.get_mut().unwrap().{}", s, m, f)
                    };
                    add_replacement(ctx, e.span, new_e);
                }
            }
            ExprKind::Assign(lhs, _, _) => match lhs.kind {
                ExprKind::Field(s, f) => {
                    let ty = type_as_string(ctx, s);
                    let s = normalize_path(&span_to_string(ctx, s.span));
                    let f = f.name.to_ident_string();
                    if let Some(m) = struct_mutex_map().get(&ty).and_then(|m| m.get(&f)) {
                        if MUTEX_INIT_MAP
                            .lock()
                            .unwrap()
                            .contains(&(func_name(), s, m.clone()))
                        {
                            add_replacement(ctx, e.span, "()".to_string());
                        }
                    }
                }
                _ => (),
            },
            ExprKind::If(c, t, f) => match c.kind {
                ExprKind::DropTemps(e) => match e.kind {
                    ExprKind::Binary(op, lhs, rhs) => {
                        let lhs = span_to_string(ctx, lhs.span);
                        let rhs = span_to_string(ctx, rhs.span);
                        let trylock_map = TRYLOCK_MAP.lock().unwrap();
                        let (x, g) = if let Some((_, g)) =
                            trylock_map.get(&(func_name(), lhs.clone()))
                        {
                            (lhs, g)
                        } else if let Some((_, g)) = trylock_map.get(&(func_name(), rhs.clone())) {
                            (rhs, g)
                        } else {
                            return;
                        };
                        let eq = match op.node {
                            BinOpKind::Eq => true,
                            BinOpKind::Ne => false,
                            op => panic!("{:?}", op),
                        };
                        let span = c
                            .span
                            .with_lo(c.span.lo() - BytePos(3))
                            .with_hi(c.span.hi() + BytePos(2));
                        let true_branch = if eq {
                            format!("Ok(g) => {{ {} = g;", g)
                        } else {
                            "Err(_) => {".to_string()
                        };
                        add_replacement(ctx, span, format!("match {} {{ {}", x, true_branch));
                        if let Some(f) = f {
                            let span = t
                                .span
                                .with_lo(t.span.hi())
                                .with_hi(f.span.lo() + BytePos(1));
                            let false_branch = if eq {
                                "Err(_) => {".to_string()
                            } else {
                                format!("Ok(g) => {{ {} = g;", g)
                            };
                            add_replacement(ctx, span, false_branch);

                            let span = f.span.with_lo(f.span.hi());
                            add_replacement(ctx, span, "}".to_string());
                        } else {
                            let span = t.span.with_lo(t.span.hi());
                            let false_branch = if eq {
                                "Err(_) => {} }".to_string()
                            } else {
                                format!("Ok(g) => {{ {} = g; }} }}", g)
                            };
                            add_replacement(ctx, span, false_branch);
                        }
                    }
                    _ => (),
                },
                _ => (),
            },
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

fn use_guard(func: String, guard: String) {
    GUARD_MAP
        .lock()
        .unwrap()
        .entry(func)
        .or_insert(vec![])
        .push(guard);
}

fn use_guards(func: String, guards: &Vec<String>) {
    for g in guards {
        use_guard(func.clone(), g.clone());
    }
}

fn hid_to_string(ctx: &LateContext<'_>, hid: HirId) -> String {
    span_to_string(ctx, ctx.tcx.hir().span(hid))
}

pub fn span_to_string(ctx: &LateContext<'_>, span: Span) -> String {
    let source_map = ctx.sess().source_map();
    source_map.span_to_snippet(span).unwrap()
}

fn correct_function_name(name: &str) -> String {
    if name == "main_0" { "main" } else { name }.to_string()
}

fn current_function(ctx: &LateContext<'_>, hid: HirId) -> Option<String> {
    let hir = ctx.tcx.hir();
    if let Node::Item(item) = hir.get(hir.enclosing_body_owner(hid)) {
        if let ItemKind::Fn(_, _, _) = item.kind {
            let func = item.ident.name.to_ident_string();
            Some(correct_function_name(&func))
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

fn type_of<'a, 'b, 'tcx>(
    ctx: &'a LateContext<'b>,
    e: &'tcx Expr<'tcx>,
) -> rustc_middle::ty::Ty<'b> {
    let id = e.hir_id;
    ctx.tcx.typeck(id.owner).node_type(id)
}

fn type_as_string<'a, 'b, 'tcx>(ctx: &'a LateContext<'b>, e: &'tcx Expr<'tcx>) -> String {
    type_of(ctx, e).to_string().replace("main::", "")
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

fn unwrap_addr<'tcx>(e: &'tcx Expr<'tcx>) -> Option<&'tcx Expr<'tcx>> {
    match &e.kind {
        ExprKind::AddrOf(_, _, e) => Some(e),
        _ => None,
    }
}

fn unwrap_cast_recursively<'tcx>(e: &'tcx Expr<'tcx>) -> &'tcx Expr<'tcx> {
    match &e.kind {
        ExprKind::Cast(e, _) => unwrap_cast_recursively(e),
        _ => e,
    }
}

fn normalize_arg<'a, 'b, 'tcx>(ctx: &'a LateContext<'b>, e: &'tcx Expr<'tcx>) -> (String, String) {
    let arg = unwrap_addr(unwrap_cast_recursively(e)).unwrap();
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
            let guard = guard_of(&format!("{}[{}]", arr, ind));
            (arg, guard)
        }
        _ => {
            let arg = span_to_string(ctx, arg.span);
            let guard = guard_of(&arg);
            (arg, guard)
        }
    }
}

pub fn normalize_path(p: &str) -> String {
    p.split(&[' ', '-', '>', '.', '(', ')', '*', '&'])
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(".")
}

fn path_to_id(p: &str) -> String {
    p.split(&[' ', '-', '>', '(', ')', '[', ']', '.', '*', '&'])
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

#[allow(clippy::ptr_arg)]
fn guard_of(m: &String) -> String {
    format!("{}_guard", path_to_id(m))
}

fn struct_of(m: &str) -> String {
    format!("{}Data", path_to_id(m))
}

fn struct_of2(s: &str, m: &str) -> String {
    format!("{}{}Data", path_to_id(s), path_to_id(m))
}

fn struct_of_path(_func: &str, s: &str) -> String {
    if let Some(i) = s.find('.') {
        let (s, f) = s.split_at(i);
        struct_of2(
            &ID_TYPE_MAP
                .lock()
                .unwrap()
                .get(&s.to_string())
                .unwrap()
                .replace("*mut", ""),
            f,
        )
    } else {
        struct_of(s)
    }
}

fn join(mut v: Vec<String>, sep: &str) -> String {
    v.drain(..).intersperse(sep.to_string()).collect()
}

fn make_tuple(mut v: Vec<String>) -> String {
    assert!(!v.is_empty());
    if v.len() == 1 {
        v.pop().unwrap()
    } else {
        format!("({})", join(v, ", "))
    }
}
