# Rustc Public Project Group (formerly Stable MIR)

<!--
 Status badge advertising the project as being actively worked on. When the
 project has finished be sure to replace the active badge with a badge
 like: https://img.shields.io/badge/status-archived-grey.svg
-->
![project group status: active](https://img.shields.io/badge/status-active-brightgreen.svg)
[![project group documentation](https://img.shields.io/badge/MDBook-View%20Documentation-blue)][gh-pages]
[![Run compiler tests](https://github.com/rust-lang/rustc_public/actions/workflows/nightly.yml/badge.svg)](https://github.com/rust-lang/rustc_public/actions/workflows/nightly.yml)


<!--
 Provide a short introduction about your project group. Make sure to include any
 relevant links to information about your group.
-->

Welcome to the repository for the Rustc Public Project Group! Our goal is to provide a SemVer compliant
API based on the rust compiler mid-level intermediate representation (MIR) that can be used as the foundation
for development of tools that want to perform sophisticated analyses and make stronger guarantees about the
behavior of Rust programs.

To avoid confusion, we have renamed our project and our crates to `rustc_public` to better reflect our goal of providing
a public, SemVer compliant interface rather than a completely stable API.

This is the repository we use to organise and document our work.

If you are wondering how to use rustc_public in your project, please check out the [Getting Started][tutorial] chapter.

[gh-pages]: https://rust-lang.github.io/rustc_public/

[tutorial]: https://rust-lang.github.io/rustc_public/getting-started.html

## How Can I Get Involved?

[You can find a list of the current members available
on `rust-lang/team`.][team-toml]

If you'd like to participate be sure to check out any of our [open issues] on this
repository.

We also participate on [Zulip][chat-link], feel free to introduce
yourself over there and ask us any questions you have.


[open issues]: https://github.com/rust-lang/rustc_public/issues

[chat-link]: https://rust-lang.zulipchat.com/#narrow/stream/320896-project-stable-mir

[team-toml]: https://github.com/rust-lang/team/blob/master/teams/project-stable-mir.toml
