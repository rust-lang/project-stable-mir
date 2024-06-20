# Tricks and tips

The goal of this project is to provide an interface to the Rust compiler that can be used to empower users to build
their own analysis tools.
Most of these tools, however, have similar requirements that goes beyond analyzing with the Rust compiler IR.
For example, most tools want to be able to analyze cargo crates, including their dependencies.

In this section, we document a few tricks that we found useful while developing different Rust analysis tools.

## Storing MIR for dependencies

There is a compiler flag, `-Z always-encode-mir`, that can be used for storing the MIR of all functions in the crate
metadata.

## Handling the Std Library

Either use `Xargo` or `cargo -Z build-std` to build a new version of the std library that includes the MIR body of
all functions.

You can then use the compiler `--sysroot` argument to point to the version you compiled.

## Enabling Rust Analyzer for compiler crates

1. Ensure that any crate that use rustc data structures have the following configuration in their `Cargo.toml`

```toml
[package.metadata.rust-analyzer]
rustc_private = true
```

2. Set the `rust-analyzer.rustc.source` to "discover".
   See [Rust Analyzer manual](https://rust-analyzer.github.io/manual.html) for more advanced options.