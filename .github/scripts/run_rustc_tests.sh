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

# Set the toolchain to be used in this script
REPO_TOOLCHAIN=$(rustup show active-toolchain | (read toolchain _; echo $toolchain))
TOOLCHAIN="${TOOLCHAIN:-${REPO_TOOLCHAIN}}"

# Build stable_mir tools
function build_smir_tools() {
  pushd "${SMIR_PATH}"
  cargo +${TOOLCHAIN} build -Z unstable-options --out-dir "${TOOLS_BIN}"
  export PATH="${TOOLS_BIN}":"${PATH}"
}

# Set up rustc repository
function setup_rustc_repo() {
  if [[ ! -e "${RUST_REPO}" ]]; then
    mkdir -p "$(dirname ${RUST_REPO})"
    git clone -b master https://github.com/rust-lang/rust.git "${RUST_REPO}"
    pushd "${RUST_REPO}"
    commit="$(rustc +${TOOLCHAIN} -vV | awk '/^commit-hash/ { print $2 }')"
    if [[ "${commit}" != "unknown" ]]; then
       # For custom toolchain, this may return "unknown". Skip this step if that's the case.
       # In that case, we will use the HEAD of the main branch.
      git checkout "${commit}"
    fi
    git submodule init -- "library/stdarch"
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

  SYSROOT=$(rustc +${TOOLCHAIN} --print sysroot)
  PY_PATH=$(type -P python3)
  HOST=$(rustc +${TOOLCHAIN} -vV | awk '/^host/ { print $2 }')
  FILE_CHECK="$(which FileCheck-12 || which FileCheck-13 || which FileCheck-14)"

  echo "#---------- Variables -------------"
  echo "RUST_REPO: ${RUST_REPO}"
  echo "TOOLS_BIN: ${TOOLS_BIN}"
  echo "TOOLCHAIN: ${TOOLCHAIN}"
  echo "SYSROOT: ${SYSROOT}"
  echo "FILE_CHECK: ${FILE_CHECK}"
  echo "-----------------------------------"

  for suite_cfg in "${SUITES[@]}"; do
    # Hack to work on older bash like the ones on MacOS.
    suite_pair=($suite_cfg)
    suite=${suite_pair[0]}
    mode=${suite_pair[1]}

    echo "#### Running suite: ${suite} mode: ${mode}"
    cargo +${TOOLCHAIN} run -p compiletest -- \
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
      --channel=nightly \
      --git-repository="rust-lang/project-stable-mir" \
      --nightly-branch="main" \
      --target-rustcflags="--smir-check" \
      --host-rustcflags="--smir-check"
  done
}

build_smir_tools
setup_rustc_repo
run_tests
