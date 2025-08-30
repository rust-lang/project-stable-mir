use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::OnceLock;

use regex::bytes::Regex;
use ui_test::color_eyre::eyre::Result;
use ui_test::spanned::Spanned;
use ui_test::status_emitter::StatusEmitter;
use ui_test::{
    Args, CommandBuilder, Config, Match, bless_output_files, error_on_output_conflict,
    run_tests_generic,
};

#[derive(Copy, Clone, Debug)]
/// Decides what is expected of each test's exit status.
enum Mode {
    /// The test passes a full execution of the rustc driver.
    Pass,
    /// The rustc driver should emit an error.
    Fail,
    /// The test is currently failing but is expected to succeed.
    /// This is used to add test cases that reproduce an existing bug. This help us identify issues
    /// that may be "accidentally" fixed.
    FixMe,
    /// The test passes a full execution of `cargo build`
    CargoPass,
}

#[derive(Debug)]
struct TestCx {
    /// Path for the rustc_public driver.
    driver_path: PathBuf,
    /// Arguments from the `cargo test` command.
    args: Args,
}

impl TestCx {
    fn new() -> Result<Self> {
        let driver_path: PathBuf = PathBuf::from(
            env::var("RP_TEST_DRIVER_PATH")
                .expect("RP_TEST_DRIVER_PATH must be set to run rustc_public test suites"),
        );
        let args = Args::test()?;
        Ok(Self { driver_path, args })
    }

    fn run_test(&mut self, mode: Mode, test_dir: &str) -> Result<()> {
        let config = config(mode, test_dir, self);
        eprintln!("Compiler: {}", config.program.display());
        run_tests_generic(
            vec![config],
            ui_test::default_file_filter,
            ui_test::default_per_file_config,
            Box::<dyn StatusEmitter>::from(self.args.format),
        )?;

        Ok(())
    }
}

fn config(mode: Mode, test_dir: &str, cx: &mut TestCx) -> Config {
    let config = if matches!(mode, Mode::CargoPass) {
        cargo_config(mode, test_dir, cx)
    } else {
        driver_config(mode, test_dir, cx)
    };
    cx.args.bless |= env::var("RP_TEST_DRIVER_BLESS").is_ok_and(|v| v != "0");

    common_settings(config, mode, cx.args.bless)
}

/// Configure cargo tests that will run the test-driver instead of rustc.
fn cargo_config(mode: Mode, test_dir: &str, cx: &TestCx) -> Config {
    let mut config = Config::cargo(test_dir);
    config
        .program
        .envs
        .push(("RUST".into(), Some(cx.driver_path.clone().into_os_string())));
    config.program.envs.push((
        "CARGO_ENCODED_RUSTFLAGS".into(),
        Some(rustc_flags(mode).join(&OsString::from("\x1f")).into()),
    ));
    config
}

/// Configure tests that will invoke the test-driver directly as rustc.
fn driver_config(mode: Mode, test_dir: &str, cx: &TestCx) -> Config {
    let mut config = Config::rustc(test_dir);
    config.program = CommandBuilder::rustc();
    config.program.program = cx.driver_path.clone();
    config.program.args = rustc_flags(mode);
    config
}

fn common_settings(mut config: Config, mode: Mode, bless: bool) -> Config {
    // Recommend that users should use this command to bless failing tests.
    config.bless_command = Some("./x test --bless".into());
    config.output_conflict_handling = if bless {
        bless_output_files
    } else {
        error_on_output_conflict
    };
    config.comment_defaults.base().exit_status = match mode {
        Mode::Pass | Mode::CargoPass => Some(0),
        Mode::Fail | Mode::FixMe => Some(1),
    }
    .map(Spanned::dummy)
    .into();
    config.comment_defaults.base().require_annotations =
        Spanned::dummy(matches!(mode, Mode::Fail)).into();
    config.comment_defaults.base().normalize_stderr = stderr_filters()
        .iter()
        .map(|(m, p)| (m.clone(), p.to_vec()))
        .collect();
    config.comment_defaults.base().normalize_stdout = stdout_filters()
        .iter()
        .map(|(m, p)| (m.clone(), p.to_vec()))
        .collect();
    config.out_dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("rp_tests");
    config
}

fn rustc_flags(mode: Mode) -> Vec<OsString> {
    let mut flags = vec!["--smir-check".into()];
    let verbose = env::var("RP_TEST_DRIVER_VERBOSE").is_ok_and(|v| v != "0");
    if verbose {
        flags.push("--smir-verbose".into());
    }
    if matches!(mode, Mode::FixMe) {
        // Enable checks that should pass but may trigger an existing issue.
        flags.push("--smir-fixme".into());
    }
    flags
}

fn run_all(mut cx: TestCx) -> Result<()> {
    cx.run_test(Mode::Pass, "tests/sanity-checks")?;
    cx.run_test(Mode::Pass, "tests/print")?;
    cx.run_test(Mode::FixMe, "tests/fixme")?;
    cx.run_test(Mode::Fail, "tests/fail")?;
    Ok(())
}

fn main() -> Result<()> {
    let cx = TestCx::new()?;
    run_all(cx)?;
    Ok(())
}

macro_rules! regexes {
    ($name:ident: $($regex:expr => $replacement:expr,)*) => {
        fn $name() -> &'static [(Match, &'static [u8])] {
            static S: OnceLock<Vec<(Match, &'static [u8])>> = OnceLock::new();
            S.get_or_init(|| vec![
                $((Regex::new($regex).unwrap().into(), $replacement.as_bytes()),)*
            ])
        }
    };
}

regexes! {
    stdout_filters:
    // Windows file paths
    r"\\"                           => "/",
    // erase borrow tags
    "<[0-9]+>"                      => "<TAG>",
    "<[0-9]+="                      => "<TAG=",
    // erase test paths
    "tests/print"                   => "$$DIR",
}

regexes! {
    stderr_filters:
    // erase line and column info
    r"\.rs:[0-9]+:[0-9]+(: [0-9]+:[0-9]+)?" => ".rs:LL:CC",
    // erase alloc ids
    "alloc[0-9]+"                    => "ALLOC",
    // erase thread ids
    r"unnamed-[0-9]+"               => "unnamed-ID",
    r"thread '(?P<name>.*?)' \(\d+\) panicked" => "thread '$name' ($$TID) panicked",
    // erase borrow tags
    "<[0-9]+>"                       => "<TAG>",
    "<[0-9]+="                       => "<TAG=",
    // normalize width of Tree Borrows diagnostic borders (which otherwise leak borrow tag info)
    "(─{50})─+"                      => "$1",
    // erase whitespace that differs between platforms
    r" +at (.*\.rs)"                 => " at $1",
    // erase generics in backtraces
    "([0-9]+: .*)::<.*>"             => "$1",
    // erase long hexadecimals
    r"0x[0-9a-fA-F]+[0-9a-fA-F]{2,2}" => "$$HEX",
    // erase specific alignments
    "alignment [0-9]+"               => "alignment ALIGN",
    "[0-9]+ byte alignment but found [0-9]+" => "ALIGN byte alignment but found ALIGN",
    // erase thread caller ids
    r"call [0-9]+"                  => "call ID",
    // erase platform module paths
    r"\bsys::([a-z_]+)::[a-z]+::"   => "sys::$1::PLATFORM::",
    // Windows file paths
    r"\\"                           => "/",
    // test-drive tags
    r"\[T-DRIVE\].*\n"              => "",
    // erase Rust stdlib path
    "[^ \n`]*/(rust[^/]*|checkout)/library/" => "RUSTLIB/",
    // erase rustc path
    "[^ \n`]*/(rust[^/]*|checkout)/compiler/" => "RUSTC/",
    // erase platform file paths
    r"\bsys/([a-z_]+)/[a-z]+\b"     => "sys/$1/PLATFORM",
    // erase paths into the crate registry
    r"[^ ]*/\.?cargo/registry/.*/(.*\.rs)"  => "CARGO_REGISTRY/.../$1",
}
