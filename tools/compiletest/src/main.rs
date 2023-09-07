//! Run compiletest on a given folder.

mod args;

use std::process::ExitCode;
use clap::Parser;
use ui_test::{status_emitter, Config};

fn main() -> ExitCode {
    let args = args::Args::parse();
    let verbose = args.verbose;
    if verbose {
        println!("args: ${args:?}");
    }
    let config = Config::from(args);
    if verbose {
        println!("Compiler: {}", config.program.display());
    }

    let name = config.root_dir.display().to_string();

    let text = if verbose {
        status_emitter::Text::verbose()
    } else {
        status_emitter::Text::quiet()
    };

    let result = ui_test::run_tests_generic(
        vec![config],
        ui_test::default_file_filter,
        ui_test::default_per_file_config,
        (text, status_emitter::Gha::<true> { name }),
    );
    if result.is_ok() { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}
