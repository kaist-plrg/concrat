use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::Read,
    mem,
    path::PathBuf,
    sync::Mutex,
};

use lazy_static::lazy_static;
use rustc_data_structures::sync;
use rustc_driver::{Callbacks, RunCompiler};
use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass, LintContext, LintPass};
use rustc_span::{FileName, Span, Symbol};
use rustfix::{Replacement, Snippet, Solution, Suggestion};
use spin::once::Once;

lazy_static! {
    static ref RUSTFIX_SUGGESTIONS: Mutex<HashMap<PathBuf, BTreeMap<i32, Vec<Suggestion>>>> =
        Mutex::new(HashMap::default());
    static ref MUTEX_MAP: Mutex<HashMap<String, Vec<(String, String, String)>>> =
        Mutex::new(HashMap::default());
    static ref GLOBAL_MAP: Mutex<HashMap<Symbol, String>> = Mutex::new(HashMap::default());
}

static PROTECT_MAP: Once<HashMap<String, Option<String>>> = Once::new();
static FUNCTION_MAP: Once<
    HashMap<String, HashMap<String, (Vec<String>, Vec<String>, Vec<String>)>>,
> = Once::new();

fn protect_map() -> &'static HashMap<String, Option<String>> {
    PROTECT_MAP.get().unwrap()
}

fn function_map(
) -> &'static HashMap<String, HashMap<String, (Vec<String>, Vec<String>, Vec<String>)>> {
    FUNCTION_MAP.get().unwrap()
}

pub fn collect_suggestions(
    args: Vec<String>,
    mutex_map: HashMap<String, Option<String>>,
    function_map: HashMap<String, HashMap<String, (Vec<String>, Vec<String>, Vec<String>)>>,
) -> HashMap<PathBuf, BTreeMap<i32, Vec<Suggestion>>> {
    PROTECT_MAP.call_once(|| mutex_map);
    FUNCTION_MAP.call_once(|| function_map);

    let mut callbacks = DriverCallbacks {
        passes: vec![GlobalPass::new, RewritePass::new],
    };

    let exit_code = rustc_driver::catch_with_exit_code(|| {
        let compiler = RunCompiler::new(&args, &mut callbacks);
        compiler.run()
    });

    assert_eq!(exit_code, 0);

    let mut suggestions = RUSTFIX_SUGGESTIONS.lock().unwrap();
    mem::replace(&mut suggestions, HashMap::default())
}

pub fn apply_suggestions(path: PathBuf, suggestions: BTreeMap<i32, Vec<Suggestion>>) -> String {
    let ordered_suggestions = suggestions
        .into_values()
        .flatten()
        .collect::<Vec<Suggestion>>();

    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    rustfix::apply_suggestions(contents.as_str(), &ordered_suggestions).unwrap()
}

type LatePass = dyn for<'tcx> LateLintPass<'tcx> + sync::Send + sync::Sync + 'static;
type LateCallback = fn() -> Box<LatePass>;

struct DriverCallbacks {
    passes: Vec<LateCallback>,
}

impl Callbacks for DriverCallbacks {
    fn config(&mut self, cfg: &mut rustc_interface::Config) {
        let prev_lints = std::mem::replace(&mut cfg.register_lints, None);
        let passes = self.passes.clone();
        cfg.register_lints = Some(Box::new(move |sess, lint_store| {
            for p in passes.clone() {
                lint_store.register_late_pass(p);
            }
            if let Some(lints) = &prev_lints {
                lints(sess, lint_store);
            }
        }));
    }
}

struct GlobalPass {
    depth: i32,
}

impl GlobalPass {
    fn new() -> Box<LatePass> {
        Box::new(Self { depth: 0 })
    }
}

impl LintPass for GlobalPass {
    fn name(&self) -> &'static str {
        "GlobalPass"
    }
}

impl<'tcx> LateLintPass<'tcx> for GlobalPass {
    fn check_item(&mut self, ctx: &LateContext<'tcx>, i: &'tcx Item<'tcx>) {
        match i.kind {
            ItemKind::Static(t, _, b) => {
                let name = i.ident.name.to_ident_string();
                if let Some(m) = protect_map().get(&name).unwrap() {
                    let ty = span_to_string(ctx, t.span);
                    let init = hid_to_string(ctx, b.hir_id);
                    MUTEX_MAP
                        .lock()
                        .unwrap()
                        .entry(m.clone())
                        .or_default()
                        .push((name, ty, init));
                    GLOBAL_MAP
                        .lock()
                        .unwrap()
                        .entry(i.ident.name)
                        .or_insert(m.clone());

                    make_suggestion(ctx, i.span, "".to_string(), "".to_string(), self.depth);
                    remove_attributes(ctx, i, self.depth);
                }
            }
            _ => (),
        }
    }
}

struct RewritePass {
    depth: i32,
}

impl RewritePass {
    fn new() -> Box<LatePass> {
        Box::new(Self { depth: 0 })
    }
}

impl LintPass for RewritePass {
    fn name(&self) -> &'static str {
        "RewritePass"
    }
}

impl<'tcx> LateLintPass<'tcx> for RewritePass {
    fn check_item(&mut self, ctx: &LateContext<'tcx>, i: &'tcx Item<'tcx>) {
        match i.kind {
            ItemKind::Static(_, _, _) => {
                let name = i.ident.name.to_ident_string();
                if let Some(v) = MUTEX_MAP.lock().unwrap().get(&name) {
                    let mut decl = String::new();
                    let mut init = String::new();
                    for (x, t, i) in v {
                        decl.push_str(format!("{}: {}, ", x, t).as_str());
                        init.push_str(format!("{}: {}, ", x, i).as_str());
                    }
                    let struct_name = struct_of(&name);
                    let code = format!(
                        "
pub struct {0} {{ {1}}}
pub static {2}: Mutex<{0}> = lock_api::Mutex::const_new(
    RawMutex::INIT,
    {0} {{ {3}}}
);",
                        struct_name, decl, name, init
                    );
                    make_suggestion(ctx, i.span, "".to_string(), code, self.depth);
                    remove_attributes(ctx, i, self.depth);
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
            intravisit::FnKind::ItemFn(id, _, _, _) => {
                let curr = get_current_file_name(ctx, span);
                let name = id.name.to_ident_string();
                let (entry, node, ret) = function_map().get(&curr).unwrap().get(&name).unwrap();

                let stmts = if let ExprKind::Block(b, _) = body.value.kind {
                    &b.stmts
                } else {
                    panic!()
                };

                if entry.len() > 0 {
                    let params: String = entry
                        .iter()
                        .map(|m| format!("{}: MutexGuard<'static, {}>", guard_of(m), struct_of(m)))
                        .intersperse(", ".to_string())
                        .collect();
                    let (span, sugg) = if body.params.len() == 0 {
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
                    make_suggestion(ctx, span, "".to_string(), sugg, self.depth);
                }

                let local_vars: String = node
                    .iter()
                    .filter(|m| !entry.contains(m))
                    .map(|m| format!("let mut {};\n    ", guard_of(m)))
                    .collect();
                if local_vars.len() > 0 {
                    make_suggestion(
                        ctx,
                        stmts[0].span.shrink_to_lo(),
                        "".to_string(),
                        local_vars,
                        self.depth,
                    );
                }

                if ret.len() > 0 {
                    let mut ret_types = vec![];
                    if let FnRetTy::Return(t) = decl.output {
                        ret_types.push(span_to_string(ctx, t.span));
                    }
                    for m in ret {
                        ret_types.push(format!("MutexGuard<'static, {}>", struct_of(m)));
                    }
                    let ret_type = if ret_types.len() == 1 {
                        ret_types.pop().unwrap()
                    } else {
                        let s: String = ret_types.drain(..).intersperse(", ".to_string()).collect();
                        format!("({})", s)
                    };
                    let sugg = if let FnRetTy::Return(_) = decl.output {
                        ret_type
                    } else {
                        format!("-> {} ", ret_type)
                    };
                    make_suggestion(ctx, decl.output.span(), "".to_string(), sugg, self.depth);
                    // let ret_vals =
                    //     if ret.len() == 1 {
                    //         ret[0].clone()
                    //     } else {
                    //         let s: String = ret.iter().map(guard_of).intersperse(", ".to_string()).collect();
                    //         format!("({})", s)
                    //     };
                    // make_suggestion(ctx, stmts.last().unwrap().span.shrink_to_hi(), "".to_string(), format!("\n    {}", ret_vals), self.depth);
                }
            }
            _ => (),
        }
    }

    fn check_expr(&mut self, ctx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) {
        match &e.kind {
            ExprKind::Call(func, args) => {
                let f = name(func);
                let arg = || {
                    assert_eq!(args.len(), 1);
                    addr_of_name(args.last().unwrap()).unwrap()
                };
                match f.as_ref().map(|s| s.as_str()) {
                    Some("pthread_mutex_lock") => {
                        let arg = arg();
                        make_suggestion(
                            ctx,
                            e.span,
                            "".to_string(),
                            format!("{} = {}.lock()", guard_of(&arg), arg),
                            self.depth,
                        );
                    }
                    Some("pthread_mutex_unlock") => {
                        make_suggestion(
                            ctx,
                            e.span,
                            "".to_string(),
                            format!("drop({})", guard_of(&arg())),
                            self.depth,
                        );
                    }
                    _ => (),
                }
            }
            ExprKind::Path(_) => {
                if let Some(x) = name_symbol(e) {
                    if let Some(m) = GLOBAL_MAP.lock().unwrap().get(&x).clone() {
                        make_suggestion(
                            ctx,
                            e.span,
                            "".to_string(),
                            format!("(*{}).{}", guard_of(m), x),
                            self.depth,
                        );
                    }
                }
            }
            _ => (),
        }
    }
}

// suggestions for left-hand side of expressions
fn make_suggestion(
    ctx: &LateContext<'_>,
    span: Span,
    message: String,
    replacement: String,
    depth: i32,
) {
    make_suggestion_impl(ctx, span, message, replacement, depth)
}

// suggestions for right-hand side of expressions
fn make_suggestion_after(
    ctx: &LateContext<'_>,
    span: Span,
    message: String,
    replacement: String,
    depth: i32,
) {
    make_suggestion_impl(ctx, span, message, replacement, -depth)
}

fn make_suggestion_impl(
    ctx: &LateContext<'_>,
    span: Span,
    message: String,
    replacement: String,
    depth: i32,
) {
    use rustfix::{LinePosition, LineRange};

    let source_map = ctx.sess().source_map();
    let fname = source_map.span_to_filename(span);

    let fname_real = match fname {
        FileName::Real(ref n) => n,
        _ => panic!(),
    };

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

    let mut suggestions = RUSTFIX_SUGGESTIONS.lock().unwrap();

    suggestions
        .entry(fname_real.clone().into_local_path().unwrap())
        .or_default()
        .entry(depth)
        .or_insert(vec![])
        .push(Suggestion {
            message: "".to_owned(),
            snippets: vec![snippet.clone()],
            solutions: vec![Solution {
                message,
                replacements: vec![Replacement {
                    snippet,
                    replacement,
                }],
            }],
        });
}

fn hid_to_string(ctx: &LateContext<'_>, hid: HirId) -> String {
    span_to_string(ctx, ctx.tcx.hir().span(hid))
}

fn span_to_string(ctx: &LateContext<'_>, span: Span) -> String {
    let source_map = ctx.sess().source_map();
    source_map.span_to_snippet(span).unwrap()
}

fn remove_attributes(ctx: &LateContext<'_>, i: &Item, depth: i32) {
    let hir = ctx.tcx.hir();
    let attrs = hir.attrs(hir.local_def_id_to_hir_id(i.def_id));
    for a in attrs {
        make_suggestion(ctx, a.span, "".to_string(), "".to_string(), depth);
    }
}

fn get_current_file_name(ctx: &LateContext<'_>, span: Span) -> String {
    let mut root = ctx.sess().local_crate_source_file.as_ref().unwrap().clone();
    root.pop();
    let root = root.as_path();
    let curr = ctx
        .sess()
        .source_map()
        .span_to_filename(span)
        .prefer_local()
        .to_string();
    let curr = std::path::Path::new(&curr);
    curr.strip_prefix(root)
        .unwrap()
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

fn name_symbol<'tcx>(e: &'tcx Expr<'tcx>) -> Option<Symbol> {
    match &e.kind {
        ExprKind::Path(QPath::Resolved(_, p)) => Some(p.segments.last().unwrap().ident.name),
        _ => None,
    }
}

fn name<'tcx>(e: &'tcx Expr<'tcx>) -> Option<String> {
    name_symbol(e).map(|s| s.to_ident_string())
}

fn addr_of_name<'tcx>(e: &'tcx Expr<'tcx>) -> Option<String> {
    match &e.kind {
        ExprKind::AddrOf(_, _, e) => name(e),
        _ => None,
    }
}

fn guard_of(m: &String) -> String {
    format!("{}_guard", m)
}

fn struct_of(m: &String) -> String {
    format!("{}ProtectedData", m)
}
