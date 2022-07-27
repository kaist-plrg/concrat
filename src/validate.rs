use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;
use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass, LintPass};

use crate::{
    callback::{compile_with, LatePass},
    rewrite::span_to_string,
};

lazy_static! {
    static ref STRUCT_MAP: Mutex<HashMap<String, HashMap<String, String>>> =
        Mutex::new(HashMap::default());
}

pub fn collect_definitions(args: Vec<String>) -> HashMap<String, HashMap<String, String>> {
    let exit_code = compile_with(args, vec![GlobalPass::new]);
    assert_eq!(exit_code, 0);
    std::mem::take(&mut STRUCT_MAP.lock().unwrap())
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
            ItemKind::Struct(vd, _) | ItemKind::Union(vd, _) => {
                let fds = match vd {
                    VariantData::Struct(fds, _) => fds,
                    _ => return,
                };
                let name = i.ident.name.to_ident_string();
                let fds = fds
                    .iter()
                    .map(|fd| {
                        (
                            fd.ident.name.to_ident_string(),
                            span_to_string(ctx, fd.ty.span),
                        )
                    })
                    .collect();
                STRUCT_MAP.lock().unwrap().insert(name, fds);
            }
            _ => (),
        }
    }
}
