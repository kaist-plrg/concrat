use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use etrace::some_or;
use lazy_static::lazy_static;
use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass, LintPass};

use crate::{
    callback::{compile_with, LatePass},
    util::{current_function, def_id_to_item_name, span_to_string},
};

lazy_static! {
    static ref SUMMARY: Mutex<CodeSummary> = Mutex::new(Default::default());
}

pub fn collect_definitions(args: Vec<String>) -> CodeSummary {
    let exit_code = compile_with(args, vec![GlobalPass::new]);
    assert_eq!(exit_code, 0);
    std::mem::take(&mut SUMMARY.lock().unwrap())
}

#[derive(Default, Debug)]
pub struct CodeSummary {
    pub global_set: HashSet<String>,
    pub param_map: HashMap<String, HashSet<String>>,
    pub local_map: HashMap<String, HashSet<String>>,
    pub struct_map: HashMap<String, HashMap<String, String>>,
}

#[derive(Default)]
struct GlobalPass {
    summary: CodeSummary,
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
            ItemKind::Struct(vd, _) | ItemKind::Union(vd, _) => {
                let fds = match vd {
                    VariantData::Struct(fds, _) => fds,
                    _ => return,
                };
                let fds = fds
                    .iter()
                    .map(|fd| {
                        (
                            fd.ident.name.to_ident_string(),
                            span_to_string(ctx, fd.ty.span),
                        )
                    })
                    .collect();
                self.summary.struct_map.insert(name, fds);
            }
            ItemKind::Fn(_, _, bid) => {
                let params = ctx
                    .tcx
                    .hir()
                    .body(*bid)
                    .params
                    .iter()
                    .map(|p| span_to_string(ctx, p.pat.span).replace("mut ", ""))
                    .collect();
                self.summary.param_map.insert(name, params);
            }
            ItemKind::Static(_, _, _) => {
                self.summary.global_set.insert(name);
            }
            _ => (),
        }
    }

    fn check_stmt(&mut self, ctx: &LateContext<'tcx>, s: &'tcx Stmt<'tcx>) {
        match &s.kind {
            StmtKind::Local(l) => {
                let pat = span_to_string(ctx, l.pat.span);
                let x = some_or!(pat.strip_prefix("mut "), return);
                let func = some_or!(current_function(ctx), return);
                if !ctx.tcx.type_of(func).is_fn() {
                    return;
                }
                let func = def_id_to_item_name(ctx.tcx, func);
                self.summary
                    .local_map
                    .entry(func)
                    .or_default()
                    .insert(x.to_string());
            }
            _ => (),
        }
    }

    fn check_crate_post(&mut self, _: &LateContext<'tcx>) {
        *SUMMARY.lock().unwrap() = std::mem::take(&mut self.summary);
    }
}
