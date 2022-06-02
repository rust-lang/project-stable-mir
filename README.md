This crate is regularly synced with its mirror in the rustc repo at `compiler/rustc_smir`.

We use `git subtree` for this to preserve commits and allow the rustc repo to
edit these crates without having to touch this repo. This keeps the crates compiling
while allowing us to independently work on them here. The effort of keeping them in
sync is pushed entirely onto us, without affecting rustc workflows negatively.
This may change in the future, but changes to policy should only be done via a
compiler team MCP.

## Instructions for working on this crate locally

Since the crate is the same in the rustc repo and here, the dependencies on rustc_* crates
will only either work here or there, but never in both places at the same time. Thus we use
optional dependencies on the rustc_* crates, requiring local development to use

```
cargo build --no-default-features -Zavoid-dev-deps
```

in order to compile successfully.

## Instructions for syncing

### Updating this repository

In the rustc repo, execute

```
git subtree push --prefix=compiler/rustc_smir url_to_your_fork_of_project_stable_mir some_feature_branch
```

and then open a PR of your `some_feature_branch` against https://github.com/rust-lang/project-stable-mir

### Updating the rustc library

First we need to bump our stack limit, as the rustc repo otherwise quickly hits that:

```
ulimit -s 60000
```

#### Maximum function recursion depth (1000) reached

Then we need to disable `dash` as the default shell for sh scripts, as otherwise we run into a
hard limit of a recursion depth of 1000:

```
sudo dpkg-reconfigure dash
```

and then select `No` to disable dash.


#### Patching your `git worktree`

The regular git worktree does not scale to repos of the size of the rustc repo.
So download the `git-subtree.sh` from https://github.com/gitgitgadget/git/pull/493/files and run

```
sudo cp --backup /path/to/patched/git-subtree.sh /usr/lib/git-core/git-subtree
sudo chmod --reference=/usr/lib/git-core/git-subtree~ /usr/lib/git-core/git-subtree
sudo chown --reference=/usr/lib/git-core/git-subtree~ /usr/lib/git-core/git-subtree
```

#### Actually doing a sync

In the rustc repo, execute

```
git subtree pull --prefix=compiler/rustc_smir https://github.com/rust-lang/project-stable-mir smir
```

Note: only ever sync to rustc from the project-stable-mir's `smir` branch. Do not sync with your own forks.

Then open a PR against rustc just like a regular PR.
