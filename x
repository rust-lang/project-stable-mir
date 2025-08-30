#!/usr/bin/env bash
set -eu

# Assume we are inside rustc_public repository
ROOT_DIR=$(git rev-parse --show-toplevel)
# Mix the outputs so that we can do `cargo clean` in one go.
TARGET_DIR="$ROOT_DIR"/target

REPO_TOOLCHAIN=$(rustup show active-toolchain | (read toolchain _; echo $toolchain))
TOOLCHAIN="${TOOLCHAIN:-${REPO_TOOLCHAIN}}"

cargo +${TOOLCHAIN} build --manifest-path "${ROOT_DIR}"/devtool/Cargo.toml \
  --target-dir "${TARGET_DIR}"

"${TARGET_DIR}"/debug/devtool "$@"
