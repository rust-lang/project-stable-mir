use std::process::{self, Command};
use std::{env, str};

use rustversion;

const MSRV: &str = "2025-08-09";
const SUPPORTED: bool = rustversion::cfg!(since(2025-08-09));

fn main() {
    if !SUPPORTED && !cfg!(feature = "rustc-build") {
        let current = rustc_version().unwrap_or(String::from("unknown"));
        eprintln!(
            "\nERROR: rustc_public requires rustc nightly-{MSRV} or newer\n\
                current: {current}\n\
                help: run `rustup update nightly`.\n"
        );
        process::exit(1);
    }
    println!("cargo:rerun-if-changed=build.rs");
}

fn rustc_version() -> Option<String> {
    let rustc = env::var_os("RUSTC").unwrap_or_else(|| {
        eprintln!("RUSTC is not set during build script execution.\n");
        process::exit(1);
    });
    let output = Command::new(rustc).arg("--version").output().ok()?;
    let version = str::from_utf8(&output.stdout).ok()?;
    version.parse().ok()
}
