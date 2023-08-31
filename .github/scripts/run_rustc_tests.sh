#!/usr/bin/env bash

set -e
set -u

# Location of a rust repository. Clone one if path doesn't exist.
RUST_REPO="${RUST_REPO:?Missing path to rust repository. Set RUST_REPO}"
# Where we will store the SMIR tools (Optional).
TOOLS_BIN="${TOOLS_BIN:-"/tmp/smir/bin"}"
# Assume we are inside SMIR repository
SMIR_PATH=$(git rev-parse --show-toplevel)
export RUST_BACKTRACE=1

pushd "${SMIR_PATH}"
cargo +smir build -Z unstable-options --out-dir "${TOOLS_BIN}"
export PATH="${TOOLS_BIN}":"${PATH}"

if [[ ! -e "${RUST_REPO}" ]]; then
  mkdir -p "$(dirname ${RUST_REPO})"
  git clone --depth 1 https://github.com/rust-lang/rust.git "${RUST_REPO}"
fi

pushd "${RUST_REPO}"
SUITES=(
  # Match https://github.com/rust-lang/rust/blob/master/src/bootstrap/test.rs for now
  "tests/ui/cfg ui"
)
for suite_cfg in "${SUITES[@]}"; do
  # Hack to work on older bash like the ones on MacOS.
  suite_pair=($suite_cfg)
  suite=${suite_pair[0]}
  mode=${suite_pair[1]}
  echo "${suite_cfg} pair: $suite_pair mode: $mode"
  compiletest --driver-path="${TOOLS_BIN}/test-drive" --mode=${mode} --src-base="${suite}" --output-path "${RUST_REPO}/build"
done
