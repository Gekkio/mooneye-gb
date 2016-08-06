#!/bin/sh

set -e

CARGO_FLAGS=""

if [ -n "${CI}" ]; then
    CARGO_FLAGS="-j 1"
fi

make WLAFLAGS="-DACCEPTANCE_TEST=1" -C tests clean all
cargo test --features acceptance_tests ${CARGO_FLAGS}
