use std::ffi::OsString;
/// Create our own parser and build the Config from it.
use std::fmt::Debug;
use std::path::PathBuf;
use ui_test::{CommandBuilder, Config, OutputConflictHandling, RustfixMode};

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
/// Decides what is expected of each test's exit status.
pub enum Mode {
    /// The test passes a full execution of the rustc driver
    Pass,
    /// The test produces an executable binary that can get executed on the host
    Run,
    /// The rustc driver panicked
    Panic,
    /// The rustc driver emitted an error
    Fail,
    /// Run the tests, but always pass them as long as all annotations are satisfied and stderr files match.
    Yolo,
    /// The test passes a full execution of `cargo build`
    CargoPass,
}

#[derive(Debug, clap::Parser)]
#[command(version, name = "compiletest")]
pub struct Args {
    /// The path where all tests are
    #[arg(long)]
    src_base: PathBuf,

    /// The mode according to ui_test modes.
    #[arg(long, default_value = "yolo")]
    mode: Mode,

    /// Path for the stable-mir driver.
    #[arg(long)]
    driver_path: PathBuf,

    /// Path for where the output should be stored.
    #[arg(long)]
    output_dir: PathBuf,

    #[arg(long)]
    pub verbose: bool,

    /// Run test-driver on verbose mode to print test outputs.
    #[arg(long)]
    pub no_capture: bool,
}

impl From<Mode> for ui_test::Mode {
    /// Use rustc configuration as default but override arguments to fit our use case.
    fn from(mode: Mode) -> ui_test::Mode {
        match mode {
            Mode::Pass | Mode::CargoPass => ui_test::Mode::Pass,
            Mode::Run => ui_test::Mode::Run { exit_code: 0 },
            Mode::Panic => ui_test::Mode::Panic,
            Mode::Fail => ui_test::Mode::Fail {
                require_patterns: false,
                rustfix: RustfixMode::Disabled,
            },
            Mode::Yolo => ui_test::Mode::Yolo {
                rustfix: RustfixMode::Disabled,
            },
        }
    }
}

impl From<Args> for Config {
    /// Use rustc configuration as default but override arguments to fit our use case.
    fn from(args: Args) -> Config {
        let mut config = if matches!(args.mode, Mode::CargoPass) {
            cargo_config(&args)
        } else {
            driver_config(&args)
        };
        config.filter(r"\[T-DRIVE\].*\n", "");
        config.mode = ui_test::Mode::from(args.mode);
        config.output_conflict_handling = OutputConflictHandling::Error("Should Fail".to_string());
        config.out_dir = args.output_dir;
        //config.run_lib_path = PathBuf::from(env!("RUSTC_LIB_PATH"));
        config
    }
}

fn rustc_flags(args: &Args) -> Vec<OsString> {
    let mut flags = vec!["--check-smir".into()];
    if args.verbose || args.no_capture {
        flags.push("--verbose".into());
    }
    flags
}

/// Configure cargo tests that will run the test-driver instead of rustc.
fn cargo_config(args: &Args) -> Config {
    let mut config = Config::cargo(args.src_base.clone());
    config.program.envs.push((
        "RUST".into(),
        Some(args.driver_path.clone().into_os_string()),
    ));
    config.program.envs.push((
        "CARGO_ENCODED_RUSTFLAGS".into(),
        Some(rustc_flags(args).join(&OsString::from("\x1f")).into()),
    ));
    config
}

/// Configure tests that will invoke the test-driver directly as rustc.
fn driver_config(args: &Args) -> Config {
    let mut config = Config::rustc(args.src_base.clone());
    config.program = CommandBuilder::rustc();
    config.program.program = args.driver_path.clone();
    config.program.args = rustc_flags(args);
    config
}
