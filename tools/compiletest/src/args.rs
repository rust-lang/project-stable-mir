use compiletest_rs::Config;
use std::fmt::Debug;
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[command(version, name = "compiletest")]
pub struct Args {
    /// The path where all tests are
    #[arg(long)]
    src_base: PathBuf,

    /// The mode according to compiletest modes.
    #[arg(long)]
    mode: String,

    /// Path for the stable-mir driver.
    #[arg(long)]
    driver_path: PathBuf,

    /// Path for where the output should be stored.
    #[arg(long)]
    output_path: PathBuf,

    #[arg(long)]
    verbose: bool,
}

impl From<Args> for Config {
    fn from(args: Args) -> Config {
        let mut config = Config::default();
        config.mode = args.mode.parse().expect("Invalid mode");
        config.src_base = args.src_base;
        config.rustc_path = args.driver_path;
        config.build_base = args.output_path;
        config.verbose = args.verbose;
        config.run_lib_path = PathBuf::from(env!("RUSTC_LIB_PATH"));
        config.link_deps();
        config
    }
}
