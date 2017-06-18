#!/bin/sh

set -e

CARGO_FLAGS=""

if [ -n "${CI}" ]; then
    CARGO_FLAGS="--release"
fi

make -C tests clean
make WLAFLAGS="-DACCEPTANCE_TEST=1" -C tests all
cargo test --features acceptance_tests ${CARGO_FLAGS}
