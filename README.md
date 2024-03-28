# Stable MIR Librarification Project Group

<!--
 Status badge advertising the project as being actively worked on. When the
 project has finished be sure to replace the active badge with a badge
 like: https://img.shields.io/badge/status-archived-grey.svg
-->
![project group status: active](https://img.shields.io/badge/status-active-brightgreen.svg)
[![project group documentation](https://img.shields.io/badge/MDBook-View%20Documentation-blue)][gh-pages]
[![Run compiler tests](https://github.com/rust-lang/project-stable-mir/actions/workflows/nightly.yml/badge.svg)](https://github.com/rust-lang/project-stable-mir/actions/workflows/nightly.yml)


<!--
 Provide a short introduction about your project group. Make sure to include any
 relevant links to information about your group.
-->

Welcome to the repository for the Stable MIR Librarification Project Group! Our goal is to provide a stable
API based on the rust compiler mid-level intermediate representation (MIR) that can be used as the foundation
for development of tools that want to perform sophisticated analyses and make stronger guarantees about the
behavior of Rust programs.


This is the repository we use to organise our work. Please refer to our [charter] as well
as our [github pages website][gh-pages] for more information on our goals and
current scope.

If you are wondering how to use Stable MIR in your project, check the `stable_mir` crate [source code][stable_mir]
and [documentation][stable_mir_doc].

[charter]: ./CHARTER.md
[gh-pages]: https://rust-lang.github.io/project-stable-mir
[stable_mir]: https://github.com/rust-lang/rust/tree/master/compiler/stable_mir
[stable_mir_doc]: https://doc.rust-lang.org/beta/nightly-rustc/stable_mir/index.html


## How Can I Get Involved?


[You can find a list of the current members available
on `rust-lang/team`.][team-toml]

If you'd like to participate be sure to check out any of our [open issues] on this
repository.

We also participate on [Zulip][chat-link], feel free to introduce
yourself over there and ask us any questions you have.


[open issues]: https://github.com/rust-lang/project-stable-mir/issues
[chat-link]: https://rust-lang.zulipchat.com/#narrow/stream/320896-project-stable-mir
[team-toml]: https://github.com/rust-lang/team/blob/master/teams/project-stable-mir.toml
