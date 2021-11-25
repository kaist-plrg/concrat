use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::Read,
    mem,
    path::PathBuf,
    sync::Mutex,
};

use lazy_static::lazy_static;
use rustc_driver::{Callbacks, RunCompiler};
use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass, LintContext, LintPass};
use rustc_span::{FileName, Span};
use rustfix::{Replacement, Snippet, Solution, Suggestion};

lazy_static! {
    static ref RUSTFIX_SUGGESTIONS: Mutex<HashMap<PathBuf, BTreeMap<i32, Vec<Suggestion>>>> =
        Mutex::new(HashMap::default());
}

pub fn collect_suggestions(args: Vec<String>) -> HashMap<PathBuf, BTreeMap<i32, Vec<Suggestion>>> {
    let mut callbacks = DriverCallbacks;

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

struct DriverCallbacks;

impl Callbacks for DriverCallbacks {
    fn config(&mut self, cfg: &mut rustc_interface::Config) {
        let prev_lints = std::mem::replace(&mut cfg.register_lints, None);
        cfg.register_lints = Some(Box::new(move |sess, lint_store| {
            lint_store.register_late_pass(|| Box::new(RewritePass { depth: 0 }));
            if let Some(lints) = &prev_lints {
                lints(sess, lint_store);
            }
        }));
    }
}

struct RewritePass {
    depth: i32,
}

impl LintPass for RewritePass {
    fn name(&self) -> &'static str {
        "Pass"
    }
}

impl<'tcx> LateLintPass<'tcx> for RewritePass {
    fn check_item(&mut self, _: &LateContext<'tcx>, i: &'tcx Item<'tcx>) {
        match i.kind {
            ItemKind::Static(t, _, b) => println!("{:?} {:?} {:?}", i.ident, t, b),
            _ => (),
        }
    }

    fn check_fn(
        &mut self,
        _: &LateContext<'tcx>,
        kind: intravisit::FnKind<'tcx>,
        decl: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        _: Span,
        _: HirId,
    ) {
        match kind {
            intravisit::FnKind::ItemFn(id, _, _, _) => {
                // println!("{:?}: {:?} -> {:?}", id.name, decl.inputs, decl.output);
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
                        self.make_suggestion(
                            ctx,
                            e.span,
                            "".to_string(),
                            format!("{0}_guard = {0}.lock()", arg()),
                        );
                    }
                    Some("pthread_mutex_unlock") => {
                        self.make_suggestion(
                            ctx,
                            e.span,
                            "".to_string(),
                            format!("drop({}_guard)", arg()),
                        );
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}

impl RewritePass {
    // suggestions for left-hand side of expressions
    fn make_suggestion(
        &self,
        ctx: &LateContext<'_>,
        span: Span,
        message: String,
        replacement: String,
    ) {
        self.make_suggestion_impl(ctx, span, message, replacement, self.depth)
    }

    // suggestions for right-hand side of expressions
    fn make_suggestion_after(
        &self,
        ctx: &LateContext<'_>,
        span: Span,
        message: String,
        replacement: String,
    ) {
        self.make_suggestion_impl(ctx, span, message, replacement, -self.depth)
    }

    fn make_suggestion_impl(
        &self,
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
}

fn name<'tcx>(e: &'tcx Expr<'tcx>) -> Option<String> {
    match &e.kind {
        ExprKind::Path(QPath::Resolved(_, p)) => {
            Some(p.segments.last().unwrap().ident.name.to_ident_string())
        }
        _ => None,
    }
}

fn addr_of_name<'tcx>(e: &'tcx Expr<'tcx>) -> Option<String> {
    match &e.kind {
        ExprKind::AddrOf(_, _, e) => name(e),
        _ => None,
    }
}
