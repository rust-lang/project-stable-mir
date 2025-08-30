use std::path::PathBuf;

use anyhow::{Context, Result};
use xshell::{Shell, cmd};

pub fn rustc_public_dir() -> PathBuf {
    let dev_tool_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dev_tool_dir.parent().unwrap().to_path_buf()
}

pub fn active_toolchain() -> Result<String> {
    let sh = Shell::new()?;
    sh.change_dir(rustc_public_dir());
    let stdout = cmd!(sh, "rustup show active-toolchain").read()?;
    Ok(stdout.split_whitespace().next().context("Could not obtain active Rust toolchain")?.into())
}
