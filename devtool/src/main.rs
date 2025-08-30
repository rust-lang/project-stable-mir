mod utils;

use std::env;
use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use xshell::{Cmd, Shell, cmd};

use crate::utils::active_toolchain;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Build rustc_public itself.
    Build {
        /// Flags that are passed through to `cargo build`.
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        flags: Vec<String>,
    },
    /// Run rustc_public test suites.
    Test {
        /// Overwrite *.stderr/stdout files.
        #[arg(long)]
        bless: bool,
        /// Run test-drive on verbose mode to print test outputs.
        #[arg(long)]
        verbose: bool,
        /// Flags that are passed through to the test harness.
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        flags: Vec<String>,
    },
    /// Clean out build directories.
    Clean {
        /// Flags that are passed through to `cargo clean`.
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        flags: Vec<String>,
    },
    /// Run rustfmt.
    Fmt {
        /// Run rustfmt check.
        #[arg(long)]
        check: bool,
        /// Flags that are passed through to `rustfmt`.
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        flags: Vec<String>,
    },
    /// Bump the Minimum Supported Rust Version.
    MSRV {
        /// The nightly version you want to bump to. Note that it should be a date.
        #[arg(long)]
        date: String,
    },
    /// Install the git hook to perform format check just before pushing.
    Githook {
        #[command(subcommand)]
        command: GithookCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum GithookCommand {
    /// Install our `pre-push.sh` to `.git/hooks/pre-push`.
    Install,
    /// Uninstall our `pre-push.sh` from `.git/hooks/pre-push`.
    Uninstall,
}

#[derive(Parser)]
#[command(name = "devtool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

pub struct DevCx {
    /// The root path of the rustc_public checkout.
    root_dir: PathBuf,
    /// The toolchain that we use to compile our crates.
    toolchain: String,
    /// We use this shell to execute commands.
    sh: Shell,
    /// The cargo we use.
    cargo_bin: String,
}

impl DevCx {
    pub fn new() -> Result<Self> {
        let root_dir = utils::rustc_public_dir();
        let toolchain = active_toolchain()?;
        let sh: Shell = Shell::new()?;
        let cargo_bin = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
        Ok(Self { root_dir, toolchain, sh, cargo_bin })
    }

    pub fn cargo(&self, cmd: &str, crate_dir: &str) -> Cmd<'_> {
        let Self { root_dir, toolchain, sh, cargo_bin } = self;
        let mainfest: PathBuf =
            [root_dir.to_str().unwrap(), crate_dir, "Cargo.toml"].iter().collect();
        cmd!(sh, "{cargo_bin} +{toolchain} {cmd} --manifest-path {mainfest}")
    }

    pub fn git(&self, cmd: &str) -> Cmd<'_> {
        cmd!(self.sh, "git {cmd}")
    }

    pub fn build(&self, crate_dir: &str, flags: &Vec<String>) -> Result<()> {
        let cmd = self.cargo("build", crate_dir).args(&["--all-targets"]).args(flags);
        cmd.run()?;
        Ok(())
    }

    pub fn test(&self, bless: bool, verbose: bool, flags: &Vec<String>) -> Result<()> {
        let test_drive_path =
            utils::rustc_public_dir().join("target").join("debug").join("test-drive");
        // compiletest needs this var to run test suites
        self.sh.set_var("RP_TEST_DRIVER_PATH", test_drive_path);
        if verbose {
            self.sh.set_var("RP_TEST_DRIVER_VERBOSE", "Ciallo");
        }
        if bless {
            self.sh.set_var("RP_TEST_DRIVER_BLESS", "Ciallo");
        }
        let cmd = self.cargo("test", "rustc_public").args(flags);
        cmd.run()?;
        Ok(())
    }

    pub fn clean(&self, flags: &Vec<String>) -> Result<()> {
        let cmd = self.cargo("clean", ".").args(flags);
        cmd.run()?;
        Ok(())
    }

    pub fn fmt(&self, check: bool, crate_dir: &str, flags: &Vec<String>) -> Result<()> {
        let mut cmd = self.cargo("fmt", crate_dir).args(flags);
        if check {
            cmd = cmd.args(&["--check"]);
        }
        cmd.run()?;
        Ok(())
    }

    pub fn bump_msrv(&self, date: String) -> Result<()> {
        let _ = date;
        todo!()
    }

    pub fn install_git_book(&self) -> Result<()> {
        let cmd = self.git("rev-parse").arg("--git-common-dir");
        let git_dir = cmd.read()?;
        let git_dir = PathBuf::from(git_dir.trim());
        let hooks_dir = git_dir.join("hooks");
        let dst = hooks_dir.join("pre-push");
        if dst.exists() {
            // The git hook has already been set up.
            println!("`.git/hooks/pre-push` has already been installed.");
            return Ok(());
        }
        if !hooks_dir.exists() {
            let _ = std::fs::create_dir(hooks_dir);
        }
        let pre_push = self.root_dir.join("scripts").join("pre-push.sh");
        match std::fs::hard_link(pre_push, &dst) {
            Err(e) => {
                eprintln!(
                    "ERROR: could not create hook {}: do you already have the git hook installed?\n{}",
                    dst.display(),
                    e
                );
                return Err(e.into());
            }
            Ok(_) => println!("Linked `scripts/pre-push.sh` to `.git/hooks/pre-push`"),
        };
        Ok(())
    }

    pub fn uninstall_git_book(&self) -> Result<()> {
        let cmd = self.git("rev-parse").arg("--git-common-dir");
        let git_dir = cmd.read()?;
        let git_dir = PathBuf::from(git_dir.trim());
        let hooks_dir = git_dir.join("hooks");
        let dst = hooks_dir.join("pre-push");
        if !dst.exists() {
            // The git hook has not been set up yet.
            println!("`.git/hooks/pre-push` has not been set up yet.");
            return Ok(());
        }
        match std::fs::remove_file(&dst) {
            Ok(_) => println!("`.git/hooks/pre-push` is removed"),
            Err(e) => {
                eprintln!("ERROR: could not remove hook {}\n{}", dst.display(), e);
                return Err(e.into());
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Build { flags } => {
            let cx = DevCx::new()?;
            cx.build("rustc_public", &flags)?;
            Ok(())
        }
        Command::Test { bless, verbose, flags } => {
            let cx = DevCx::new()?;
            cx.build("rustc_public", &flags)?;
            cx.build("test-drive", &flags)?;
            cx.test(bless, verbose, &flags)?;
            Ok(())
        }
        Command::Clean { flags } => {
            let cx = DevCx::new()?;
            cx.clean(&flags)?;
            Ok(())
        }
        Command::MSRV { date } => {
            let cx = DevCx::new()?;
            cx.bump_msrv(date)?;
            Ok(())
        }
        Command::Fmt { check, flags } => {
            let cx = DevCx::new()?;
            cx.fmt(check, "rustc_public", &flags)?;
            cx.fmt(check, "devtool", &flags)?;
            cx.fmt(check, "test-drive", &flags)?;
            Ok(())
        }
        Command::Githook { command } => {
            let cx = DevCx::new()?;
            match command {
                GithookCommand::Install => cx.install_git_book()?,
                GithookCommand::Uninstall => cx.uninstall_git_book()?,
            }
            Ok(())
        }
    }
}
