#!/bin/bash

agave-install init 2.1.0
rm -rf target
cargo build
./scripts/build-test-programs.sh
cargo +nightly fmt --all -- --check
cargo +nightly clippy --all --all-features -- -D warnings
cargo test --all-features