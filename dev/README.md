# Development documentation

This folder contains some documentation useful for the development of Stable MIR

## Toolchain versions

Our CI currently run a nightly job that build and run our test suites against the latest nightly  toolchain.
This repository also contains a .toolchain file that specifies to use `nightly` channel.

However, if you already have the nightly toolchain installed, this will not update the toolchain to
the latest nightly.
You need to explicitly do that.

If you see some errors while compiling our tools, please ensure you have the latest nightly.
You might also want to check [our nightly runs](https://github.com/rust-lang/project-stable-mir/actions/workflows/nightly.yml?query=branch%3Amain) to ensure they are not currently broken.
If so, you can check what was the last successful nightly run, and use its nightly version.

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
