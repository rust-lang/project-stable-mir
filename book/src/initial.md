# Initial Integration

In order to use `stable_mir` in your crate, you will need to do the following:

1. Use a nightly toolchain that includes the `stable_mir` crate.
2. Install at least the following rustup components: "rustc-dev" and "llvm-tools"
3. Declare `stable_mir` as an external crate at the top of your crate:

```rust
extern crate stable_mir;
```

For 1 and 2, we highly recommend adding a "rust-toolchain.toml" file to your project.
We also recommend to pin down a specific nightly version, to ensure all users and tests are using the same compiler
version.
Therefore, the same `stable_mir` crate version. E.g.:

```toml
# Example of a rust-toolchain.toml
[toolchain]
# Update the date to change the toolchain version.
channel = "nightly-2024-06-17"
components = ["llvm-tools", "rustc-dev", "rust-src"]
```

## Initializing StableMIR

There's currently no stable way to initialize the Rust compiler and StableMIR.
See [#0069](https://github.com/rust-lang/project-stable-mir/issues/69) for more details.

Instead, StableMIR includes two unstable workarounds to give you a quick start.
The `run` and `run_with_tcx` macros, both from present in the `rustc_smir` crate.

In order to use the `run` macro, you first need to declare the following external crates:

```rust
extern crate rustc_driver;
extern crate rustc_interface;
#[macro_use]
extern crate rustc_smir;
// This one you should know already. =)
extern crate stable_mir;
```

Then start the driver using the `run!()` macro:

```rust
 let result = run!(rustc_args, callback_fn);
```

This macro takes two arguments:

1. A vector with the command arguments to the compiler.
2. A callback function, which can either be a closure expression or a function ident.
    - This callback function shouldn't take any argument, and it should return a `ControlFlow<B,C>`.

It will trigger the compilation in the current process, with the provided arguments, and invoke the callback after the
compiler ran all its analyses, but before code generation.

The macro will return a `Result<C, CompilerError<B>>` representing the compilation result and the callback return value.

The second option is the `run_with_tcx!()` macro, which is very similar to the `run` macro.
The main difference is that this macro passes a copy of the Rust compiler context (`TyCtxt`) to the callback,
which allows the user to also make calls to internal compiler APIs.

Note that this option also requires the declaration of the `rustc_middle` external crate, i.e., you should now have the
following declarations:

```rust
extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle; // This one is new!
#[macro_use]
extern crate rustc_smir;
extern crate stable_mir;
```

## Scope of StableMIR objects

StableMIR objects should not be used outside the scope of the callback function.
Any usage outside this scope can panic or return an incorrect value.

For example, the following code is valid, since the logic we are storing is only used while the callback function
is running:

```rust
fn print_items(rustc_args: Vec<String>) {
    let _result = run!(rustc_args, || {
       for item in stable_mir::all_local_items() {
           // Using items inside the callback!
           println!(" - {}", item.name())
       }
    });
}
```

However, the following usage isn't valid, and `stable_mir` will panic when we invoke the `name()` function.

```rust
fn broken_print_items(rustc_args: Vec<String>) {
    let result = run!(rustc_args, || { ControlFlow::Continue(stable_mir::all_local_items())});
    if let ControlFlow::Continue(items) = result {
        for item in items {
            // Using item outside the callback function is wrong!
            println!(" - {}", item.name())
        }
    }
}
```

StableMIR objects should also not be shared across different threads.

## Analyzing crate definitions

TODO

## Analyzing monomorphic instances

TODO