use rustc_data_structures::sync;
use rustc_driver::{Callbacks, RunCompiler};
use rustc_lint::LateLintPass;

pub type LatePass = dyn for<'tcx> LateLintPass<'tcx> + sync::Send + sync::Sync + 'static;
pub type LateCallback = fn() -> Box<LatePass>;

#[allow(missing_debug_implementations)]
pub struct DriverCallbacks {
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

pub fn compile_with(args: Vec<String>, passes: Vec<LateCallback>) -> i32 {
    let mut callbacks = DriverCallbacks { passes };
    rustc_driver::catch_with_exit_code(|| {
        let compiler = RunCompiler::new(&args, &mut callbacks);
        compiler.run()
    })
}
