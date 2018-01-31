#!/bin/sh

set -e

CARGO_FLAGS="--release"

make -C tests clean all
cargo test --features acceptance_tests ${CARGO_FLAGS}
