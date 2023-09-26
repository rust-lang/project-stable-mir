#!/usr/bin/env bash

# Run rustc test suites using our test driver using nightly.
# This script leverages the rustc's repo compiletest crate.
#
# The suites configuration should match:
# https://github.com/rust-lang/rust/blob/master/src/bootstrap/test.rs

set -e
set -u
export RUST_BACKTRACE=1

# Location of a rust repository. Clone one if path doesn't exist.
RUST_REPO="${RUST_REPO:-"/tmp/rustc"}"

# Where we will store the SMIR tools (Optional).
TOOLS_BIN="${TOOLS_BIN:-"/tmp/smir/bin"}"

# Assume we are inside SMIR repository
SMIR_PATH=$(git rev-parse --show-toplevel)

# Build stable_mir tools
function build_smir_tools() {
  pushd "${SMIR_PATH}"
  cargo +nightly build -Z unstable-options --out-dir "${TOOLS_BIN}"
  export PATH="${TOOLS_BIN}":"${PATH}"
}

# Set up rustc repository
function setup_rustc_repo() {
  if [[ ! -e "${RUST_REPO}" ]]; then
    mkdir -p "$(dirname ${RUST_REPO})"
    git clone -b master https://github.com/rust-lang/rust.git "${RUST_REPO}"
    pushd "${RUST_REPO}"
    commit="$(rustc +nightly -vV | awk '/^commit-hash/ { print $2 }')"
    git checkout ${commit}
    git submodule init -- "${RUST_REPO}/library/stdarch"
    git submodule update
  else
    pushd "${RUST_REPO}"
  fi
}

function run_tests() {
  # Run the following suite configuration for now (test suite + mode)
  SUITES=(
    "codegen codegen"
    "codegen-units codegen-units"
    # -- The suites below are failing because of fully qualified paths for standard library
    # E.g.:
    # -    _10 = _eprint(move _11) -> [return: bb6, unwind unreachable];
    # +    _10 = std::io::_eprint(move _11) -> [return: bb6, unwind unreachable];
    #
    #"ui ui"
    #"mir-opt mir-opt"
    #"pretty pretty" -- 2 failing tests
  )

  SYSROOT=$(rustc +nightly --print sysroot)
  PY_PATH=$(type -P python3)
  HOST=$(rustc +nightly -vV | awk '/^host/ { print $2 }')
  FILE_CHECK="$(which FileCheck-12 || which FileCheck-13 || which FileCheck-14)"

  for suite_cfg in "${SUITES[@]}"; do
    # Hack to work on older bash like the ones on MacOS.
    suite_pair=($suite_cfg)
    suite=${suite_pair[0]}
    mode=${suite_pair[1]}

    echo "#### Running suite: ${suite} mode: ${mode}"
    cargo +nightly run -p compiletest -- \
      --compile-lib-path="${SYSROOT}/lib" \
      --run-lib-path="${SYSROOT}/lib"\
      --python="${PY_PATH}" \
      --rustc-path="${TOOLS_BIN}/test-drive" \
      --mode=${mode} \
      --suite="${suite}" \
      --src-base="tests/${suite}" \
      --build-base="$(pwd)/build/${HOST}/stage1/tests/${suite}" \
      --sysroot-base="$SYSROOT" \
      --stage-id=stage1-${HOST} \
      --cc= \
      --cxx= \
      --cflags= \
      --cxxflags= \
      --llvm-components= \
      --android-cross-path= \
      --target=${HOST} \
      --llvm-filecheck="${FILE_CHECK}" \
      --channel=nightly
  done
}

build_smir_tools
setup_rustc_repo
run_tests
