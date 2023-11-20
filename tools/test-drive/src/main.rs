//! Test that users are able to inspec the MIR body of functions and types

#![feature(rustc_private)]
#![feature(assert_matches)]
#![feature(result_option_inspect)]

mod sanity_checks;

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_smir;
extern crate stable_mir;

use rustc_middle::ty::TyCtxt;
use rustc_smir::{run, rustc_internal};
use stable_mir::CompilerError;
use std::ops::ControlFlow;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, Ordering};

// ---- Arguments that should be parsed by the test-driver (w/ "smir" prefix)
const CHECK_ARG: &str = "--smir-check";
/// Enable verbose mode.
const VERBOSE_ARG: &str = "--smir-verbose";
/// Argument used to enable checks that may be failing due to an existing issue.
const FIXME_ARG: &str = "--smir-fixme";

// Use a static variable for simplicity.
static VERBOSE: AtomicBool = AtomicBool::new(false);
static FIXME_CHECKS: AtomicBool = AtomicBool::new(false);

type TestResult = Result<(), String>;

/// This is a wrapper that can be used to replace rustc.
///
/// Besides all supported rustc arguments, use `--check-smir` to run all the stable-mir checks.
/// This allows us to use this tool in cargo projects to analyze the target crate only by running
/// `cargo rustc --check-smir`.
fn main() -> ExitCode {
    let args = std::env::args();
    let (smir_args, rustc_args): (Vec<String>, _) = args.partition(|arg| arg.starts_with("--smir"));
    let result = if smir_args.contains(&CHECK_ARG.to_string()) {
        VERBOSE.store(
            smir_args.contains(&VERBOSE_ARG.to_string()),
            Ordering::Relaxed,
        );
        FIXME_CHECKS.store(
            smir_args.contains(&FIXME_ARG.to_string()),
            Ordering::Relaxed,
        );
        run!(rustc_args, tcx, test_stable_mir(tcx))
    } else {
        run!(rustc_args, ControlFlow::<()>::Continue(()))
    };
    if result.is_ok() || matches!(result, Err(CompilerError::Skipped)) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

macro_rules! run_tests {
    ($( $test:path ),+ $(,)?) => {
        [$({
            run_test(stringify!($test), || { $test() })
        },)+]
    };
}

fn info(msg: String) {
    if VERBOSE.load(Ordering::Relaxed) {
        // We filter output based on [T-DRIVE] prefix.
        eprintln!("[T-DRIVE] {}", msg);
    }
}

/// This function invoke other tests and process their results.
/// Tests should avoid panic,
fn test_stable_mir(_tcx: TyCtxt<'_>) -> ControlFlow<()> {
    let mut results = Vec::from(run_tests![
        sanity_checks::test_entry_fn,
        sanity_checks::test_all_fns,
        sanity_checks::test_crates,
        sanity_checks::test_instances,
    ]);
    if FIXME_CHECKS.load(Ordering::Relaxed) {
        results.extend_from_slice(&run_tests!(sanity_checks::test_traits))
    }
    let (success, failure): (Vec<_>, Vec<_>) = results.iter().partition(|r| r.is_ok());
    info(format!(
        "Ran {} tests. {} succeeded. {} failed",
        results.len(),
        success.len(),
        failure.len()
    ));
    if failure.is_empty() {
        ControlFlow::<()>::Continue(())
    } else {
        ControlFlow::<()>::Break(())
    }
}

fn run_test<F: FnOnce() -> TestResult>(name: &str, f: F) -> TestResult {
    let result = match catch_unwind(AssertUnwindSafe(f)) {
        Err(_) => Err("Panic!".to_string()),
        Ok(result) => result,
    };
    if let Err(ref msg) = result {
        eprintln!("Test {}: Failed:\n    - {}", name, msg);
    } else {
        info(format!("Test {}: Ok", name,));
    }
    result
}
