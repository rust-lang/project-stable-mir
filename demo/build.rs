use std::env;
use std::path::PathBuf;

pub fn main() {
    // Add rustup to the rpath in order to properly link with the correct rustc version.
    let rustup_home = env::var("RUSTUP_HOME").unwrap();
    let toolchain = env::var("RUSTUP_TOOLCHAIN").unwrap();
    let rustc_lib: PathBuf = [&rustup_home, "toolchains", &toolchain, "lib"]
        .iter()
        .collect();
    println!(
        "cargo:rustc-link-arg-bin=smir-demo=-Wl,-rpath,{}",
        rustc_lib.display()
    );
}
