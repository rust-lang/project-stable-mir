# Initial Integration

For an example of how to use `rustc_public`, see the [`demo/`] directory of the rustc_public repo.
For a tutorial, read on!

In order to use `rustc_public` in your crate, you will need to do the following:

1. Use a nightly toolchain that includes the `rustc_public` crate.
2. Install at least the following rustup components: "rustc-dev" and "llvm-tools"
3. Declare `rustc_public` as an external crate at the top of your crate:

```rust
#![feature(rustc_private)]
extern crate rustc_public;
```

For 1 and 2, we highly recommend adding a "rust-toolchain.toml" file to your project.
We also recommend to pin down a specific nightly version, to ensure all users and tests are using the same compiler
version.
Therefore, the same `rustc_public` crate version. E.g.:

```toml
# Example of a rust-toolchain.toml
[toolchain]
# Update the date to change the toolchain version.
channel = "nightly-2025-10-27"
components = ["llvm-tools", "rustc-dev", "rust-src"]
```

## Initializing `rustc_public`

There's currently no stable way to initialize the Rust compiler and `rustc_public`.
See [#0069](https://github.com/rust-lang/rustc_public/issues/69) for more details.

Instead, `rustc_public` includes two unstable workarounds to give you a quick start.
The `run` and `run_with_tcx` macros, both from present in the `rustc_public` crate.

In order to use the `run` macro, you first need to declare the following external crates:

```rust
#![feature(rustc_private)]
extern crate rustc_public;

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
```

Then start the driver using the `run!()` macro:

```rust
let result = rustc_public::run!(rustc_args, callback_fn);
```

This macro takes two arguments:

1. A `&[String]` with the command line arguments to the compiler.
2. A callback function, which can either be a closure expression or a function ident.
    - This callback function shouldn't take any argument, and it should return a `ControlFlow<B,C>`.

It will trigger the compilation in the current process, with the provided arguments, and invoke the callback after the
compiler ran all its analyses, but before code generation.

The macro will return a `Result<C, CompilerError<B>>` representing the compilation result and the callback return value.

The second option is the `run_with_tcx!()` macro, which is very similar to the `run` macro.
The main difference is that this macro passes a copy of the Rust compiler context (`TyCtxt`) to the callback,
which allows the user to also make calls to internal compiler APIs.

## Scope of `rustc_public` objects

`rustc_public` objects should not be used outside the scope of the callback function.
Any usage outside this scope can panic or return an incorrect value.

For example, the following code is valid, since the logic we are storing is only used while the callback function
is running:

```rust
fn print_items(rustc_args: Vec<String>) {
    let _result = run!(rustc_args, || {
       for item in rustc_public::all_local_items() {
           // Using items inside the callback!
           println!(" - {}", item.name())
       }
    });
}
```

However, the following usage isn't valid, and `rustc_public` will panic when we invoke the `name()` function.

```rust
fn broken_print_items(rustc_args: Vec<String>) {
    let result = run!(rustc_args, || { ControlFlow::Continue(rustc_public::all_local_items())});
    if let ControlFlow::Continue(items) = result {
        for item in items {
            // Using item outside the callback function is wrong!
            println!(" - {}", item.name())
        }
    }
}
```

`rustc_public` objects should also not be shared across different threads.

## Analyzing crate definitions

TODO

## Analyzing monomorphic instances

TODO

[`demo/`](https://github.com/rust-lang/rustc_public/tree/main/demo)
