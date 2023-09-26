#!/usr/bin/env bash
# Execute our own set of tests using a local `compiletest` tool based on `ui_test`.
set -e
set -u

# Where we will store the SMIR tools (Optional).
TOOLS_BIN="${TOOLS_BIN:-"/tmp/smir/bin"}"
# Assume we are inside SMIR repository
SMIR_PATH=$(git rev-parse --show-toplevel)
export RUST_BACKTRACE=1

# Build stable_mir tools
function build_smir_tools() {
  pushd "${SMIR_PATH}"
  cargo +nightly build -Z unstable-options --out-dir "${TOOLS_BIN}"
  export PATH="${TOOLS_BIN}":"${PATH}"
}

# Run tests
function run_tests() {
  SUITES=(
    "sanity-checks pass"
    "fixme fail"
  )
  for suite_cfg in "${SUITES[@]}"; do
    # Hack to work on older bash like the ones on MacOS.
    suite_pair=($suite_cfg)
    suite=${suite_pair[0]}
    mode=${suite_pair[1]}
    echo "#### Running suite: ${suite} mode: ${mode}"
    compiletest \
        --driver-path="${TOOLS_BIN}/test-drive" \
        --mode=${mode} \
        --src-base="tests/${suite}" \
        --output-dir="target/tests/" \
        --no-capture
  done
}

build_smir_tools
run_tests
