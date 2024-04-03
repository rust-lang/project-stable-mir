#!/usr/bin/env bash
# Builds and run the demo driver against itself.
# I.e.: This bootstrap itself.

REPO_DIR=$(git rev-parse --show-toplevel)
DEMO_DIR="${REPO_DIR}/demo"

cd "${DEMO_DIR}"
cargo run -- src/main.rs --crate-name demo --edition 2021 -C panic=abort

