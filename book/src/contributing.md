# Developer documentation

This folder contains some documentation useful for the development of Stable MIR

## Tools

We currently have two different tools that is used to help with testing StableMIR

### TestDrive

This is a small driver that we build on the top of the rust compiler.
By default, this driver behaves exactly like `rustc`, and it can be used to build a crate
or multiple packages using cargo.

This driver also contains multiple checks that will inspect the stable MIR of the crate that was compiled.
In order to run those tests, you need to pass an extra argument `--smir-check`.

Let's say we have a crate `input.rs`, in order to run all checks, one should invoke:

```shell
cargo run -p test-drive -- --smir-check test.rs
# or run the test-drive directly where ${BIN_PATH} is the binary location
${BIN_PATH}/test-drive --smir-check test.rs
```

In order to run this as part of a cargo build, you can run from a workspace:

```shell
# Only check SMIR for the target crate
RUSTC=${BIN_PATH} cargo rustc -p ${TARGET_PACKAGE} -- --smir-check

# Check SMIR for all crates being compiled, including dependencies
RUSTC=${BIN_PATH} RUSTFLAGS=--smir-check cargo build
```

This driver accepts a few other options that are all preceded with `--smir` prefix[^outdated]. For example
 - `--smir-fixme`: Will run checks that currently trigger a known bug in StableMIR.
 - `--smir-verbose`: Print status and failure messages.

[^outdated]: Since these are test tools, this documentation may be outdated.

### Compiletest

This is a little utility built on the top of `ui_test` crate.
It scans our test folders and run tests according to the selected mode.
For more details on how to run this utility, check its help option:
```shell
cargo run -p compiletest -- --help
```

## Test Suites

We have a few different test suites for Stable MIR:
  - **Rust compiler [`ui-fulldeps/stable-mir`](https://github. com/rust-lang/rust/tree/master/tests/ui-fulldeps/stable-mir):**
    These are very simple sanity checks that live inside the main rust repository.
    These tests should cover very basic functionality related to the translation of internal structures to stable ones.
  - **Rust compiler suites:** We are enabling the execution of rust compiler test suites.
  These suites are run with the rust respository compiletest. 
  To run them, I recommend using our script `scripts/run_rustc_tests.sh` since there are a lot of arguments.
  - **Local suites:** These are suites hosted in this repository inside `tests/`.
  These are run using our local `compilest` and
  `scripts/run_own_tests.sh` is a script to run all local suites
    - `fixme`: Single file crates that are currently failing.
      These tests are run using `--smir-fixme`.
    - `sanity-checks`: Small examples that exercise some specific construct.
      These tests succeed if compilation and all smir checks succeed.

## Toolchain versions

Our CI currently run a nightly job that build and run our test suites against the latest nightly  toolchain.
This repository also contains a .toolchain file that specifies to use `nightly` channel.

However, if you already have the nightly toolchain installed, this will not update the toolchain to
the latest nightly.
You need to explicitly do that.

If you see some errors while compiling our tools, please ensure you have the latest nightly.
You might also want to check [our nightly runs](https://github. com/rust-lang/project-stable-mir/actions/workflows/nightly.yml?query=branch%2Amain) to ensure they are not currently broken.
If so, you can check what was the last successful nightly run, and use its nightly version.

### Custom toolchain

In order to run the tools and test suites using a local copy of `rustc`, do the following:
  1. Build stage 2 of the compiler.
     See [`rustc` build guide](https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html) for more details. E.g.:
```shell
git clone https://github.com/rust-lang/rust.git
cd rust
git checkout ${COMMIT_ID:?"Missing rustc commit id"}
./configure --enable-extended --tools=src,rustfmt,cargo --enable-debug --set=llvm.download-ci-llvm=true
./x.py build -i --stage 2
```

  2. Create a custom toolchain:
```shell
# This will create a toolchain named "local"
# Set the TARGET variable, e.g.: x86_64-unknown-linux-gnu
rustup toolchain link local build/${TARGET}/stage2
cp build/${TARGET}/stage2-tools-bin/* build/${TARGET}/stage2/bin/
```
  3. Override the current toolchain in your `project-stable-mir` repository.
```shell
cd ${SMIR_FOLDER}
rustup override set local
cargo clean
cargo build
```

By default, the build scripts will use the active toolchain for the project folder.
If you run step 3, the scripts should already pick up the local toolchain.
Additionally, you can also set the rust toolchain by setting the TOOLCHAIN environment variable.
E.g.:
```shell
# Clean old build
cargo clean

# Run test with local toolchain
TOOLCHAIN=local ./.github/scripts/run_own_tests.sh
```

Note: To remove the override created on step 3, run `rustup override unset` in the same folder.
