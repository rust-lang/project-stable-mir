# Getting Started

The StableMIR APIs are currently exposed as a crate in the compiler named `stable_mir`[^release].
This crate includes the definition of structures and methods to be stabilized,
which are expected to become the stable APIs in the compiler.

These APIs were designed to provide information about a Rust crate, including the body of functions, as well as type
and layout information.

This chapter has two sections directed at different use cases.

1. If you already have a crate that uses some of the Rust compiler libraries,
   and you are interested in migrating them to the StableMIR APIs,
   you can find more details about your use case at the [Migrating to StableMIR](./migrating.md) section.
2. If you are starting your integration with the Rust compiler via StableMIR, we recommend reading through the
   [Initial Integration](./initial.md) chapter.

We also include a [Tips and Tricks](./tricks.md) section that is related to a few common obstacles tool writers
encounter,
that is not directly related to the `stable_mir` crate and APIs.

Our repository also includes a little [demo crate](https://github.com/rust-lang/project-stable-mir/tree/main/demo) that
demonstrate how `stable_mir` crate can be used to analyze the main function of a crate.

The latest crate documentation can be found in the
[nightly documentation here](https://doc.rust-lang.org/nightly/nightly-rustc/stable_mir/index.html)

[^release]: We are planning to release the `stable_mir` crate into crates.io in the near future.
See issue [#0030](https://github.com/rust-lang/project-stable-mir/issues/30) for the current release status.
