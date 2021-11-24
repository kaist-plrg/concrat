#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_span;

use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use std::sync::Mutex;

use lazy_static::lazy_static;
use rustc_lint::{LateContext, LateLintPass, LintContext, LintPass};
use rustc_span::{FileName, Span};
use rustfix::{Replacement, Snippet, Solution, Suggestion};

lazy_static! {
    static ref RUSTFIX_SUGGESTIONS: Mutex<HashMap<PathBuf, BTreeMap<i32, Vec<Suggestion>>>> =
        Mutex::new(HashMap::default());
}

fn main() {
    let args: Vec<_> = vec![
        "create-initial-program",
        "/home/medowhill/simple/c2rust-lib.rs",
        "--sysroot",
        "/home/medowhill/.rustup/toolchains/nightly-2021-11-24-x86_64-unknown-linux-gnu",
        "--crate-type",
        "lib",
        "-A",
        "warnings",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let mut callbacks = DriverCallbacks;

    let exit_code = rustc_driver::catch_with_exit_code(|| {
        let compiler = rustc_driver::RunCompiler::new(&args, &mut callbacks);
        compiler.run()
    });

    println!("{}", exit_code);

    for (file, suggestions) in RUSTFIX_SUGGESTIONS.lock().unwrap().iter() {
        println!("For file {}:", file.to_str().unwrap());

        for suggestion in suggestions.values().flatten() {
            let solution = &suggestion.solutions[0];
            println!("{}", solution.message);
            for replacement in &solution.replacements {
                println!(" - replace {:?}", replacement.snippet.text);
                println!("   with   `{}`", replacement.replacement);
                println!(
                    "   at {} {}:{}-{}:{}",
                    replacement.snippet.file_name,
                    replacement.snippet.line_range.start.line,
                    replacement.snippet.line_range.start.column,
                    replacement.snippet.line_range.end.line,
                    replacement.snippet.line_range.end.column,
                );
            }
        }
    }

    for (file, suggestions) in RUSTFIX_SUGGESTIONS.lock().unwrap().drain() {
        let ordered_suggestions = suggestions
            .into_values()
            .flatten()
            .collect::<Vec<Suggestion>>();

        use std::fs::File;
        use std::io::{Read, Write};
        let mut file = File::open(file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let fixed_source_code =
            rustfix::apply_suggestions(contents.as_str(), &ordered_suggestions).unwrap();

        file = File::create("a.rs").unwrap();
        file.write_all(fixed_source_code.as_bytes()).unwrap();
    }
}

struct DriverCallbacks;

impl rustc_driver::Callbacks for DriverCallbacks {
    fn config(&mut self, cfg: &mut rustc_interface::Config) {
        let prev_lints = std::mem::replace(&mut cfg.register_lints, None);
        cfg.register_lints = Some(Box::new(move |sess, lint_store| {
            lint_store.register_late_pass(|| Box::new(Pass { depth: 0 }));
            if let Some(lints) = &prev_lints {
                lints(sess, lint_store);
            }
        }));
    }
}

struct Pass {
    depth: i32,
}

impl LintPass for Pass {
    fn name(&self) -> &'static str {
        "Pass"
    }
}

impl<'tcx> LateLintPass<'tcx> for Pass {
    fn check_fn(
        &mut self,
        _: &LateContext<'tcx>,
        kind: rustc_hir::intravisit::FnKind<'tcx>,
        decl: &'tcx rustc_hir::FnDecl<'tcx>,
        body: &'tcx rustc_hir::Body<'tcx>,
        _: Span,
        _: rustc_hir::HirId,
    ) {
        match kind {
            rustc_hir::intravisit::FnKind::ItemFn(id, _, _, _) => {
                // println!("{:?}: {:?} -> {:?}", id.name, decl.inputs, decl.output);
            }
            _ => (),
        }
    }

    fn check_expr(&mut self, ctx: &LateContext<'tcx>, e: &'tcx rustc_hir::Expr<'tcx>) {
        match &e.kind {
            rustc_hir::ExprKind::Call(func, args) => {
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

impl Pass {
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
        // get the char offsets within the file
        let lo_offset = file.original_relative_byte_pos(span.lo()).0;
        let hi_offset = file.original_relative_byte_pos(span.hi()).0;
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

        // EDIT_OFFSETS.lock().unwrap().add_edit(
        // Name::from(fname_real.local_path().to_str().unwrap()),
        // lo_offset,
        // hi_offset - lo_offset,
        // replacement.len() as u32,
        // );

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

fn name<'tcx>(e: &'tcx rustc_hir::Expr<'tcx>) -> Option<String> {
    match &e.kind {
        rustc_hir::ExprKind::Path(rustc_hir::QPath::Resolved(_, p)) => {
            Some(p.segments.last().unwrap().ident.name.to_ident_string())
        }
        _ => None,
    }
}

fn addr_of_name<'tcx>(e: &'tcx rustc_hir::Expr<'tcx>) -> Option<String> {
    match &e.kind {
        rustc_hir::ExprKind::AddrOf(_, _, e) => name(e),
        _ => None,
    }
}
