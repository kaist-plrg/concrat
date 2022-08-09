use rustc_lint::LateContext;
use rustc_middle::mir::{Body, Operand, Terminator, TerminatorKind};
use rustc_span::def_id::DefId;

use crate::util::{normalize_path, span_to_string};

pub mod intra;
pub mod pass;

pub use pass::run;

fn get_function_call<'tcx>(
    ctx: &LateContext<'tcx>,
    body: &Body<'tcx>,
    tm: &Terminator<'tcx>,
) -> Option<(DefId, Vec<String>)> {
    if let TerminatorKind::Call { func, args, .. } = &tm.kind {
        let func = if let Operand::Constant(func) = func {
            if let rustc_middle::ty::TyKind::FnDef(def_id, _) = func.literal.ty().kind() {
                *def_id
            } else {
                return None;
            }
        } else {
            return None;
        };
        let mut arguments = vec![];
        for arg in args {
            if let Operand::Move(arg) = arg {
                arguments.push(
                    normalize_path(&span_to_string(
                        ctx,
                        body.local_decls[arg.local].source_info.span,
                    ))
                    .replace(".as_mut_ptr.offset", "")
                    .replace(".as.isize", ""),
                );
            }
        }
        Some((func, arguments))
    } else {
        None
    }
}
