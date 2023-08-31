//! Run compiletest on a given folder.

mod args;
use clap::Parser;
use compiletest_rs::Config;

fn main() {
    let args = args::Args::parse();
    println!("args: ${args:?}");
    let cfg = Config::from(args);
    compiletest_rs::run_tests(&cfg);
}
