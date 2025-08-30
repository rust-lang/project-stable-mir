#!/usr/bin/env bash
# Builds and run the demo driver against an example.

REPO_DIR=$(git rev-parse --show-toplevel)
DEMO_DIR="${REPO_DIR}/demo"

cd "${DEMO_DIR}"
cargo run -- example/methods.rs --crate-name exp --edition 2021 -C panic=abort
