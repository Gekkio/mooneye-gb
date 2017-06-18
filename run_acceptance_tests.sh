#!/bin/sh

set -e

JOBS="-j `nproc`"
CARGO_FLAGS=""

if [ -n "${CI}" ]; then
    CARGO_FLAGS="${JOBS} --release"
fi

make -C tests clean
make ${JOBS} WLAFLAGS="-DACCEPTANCE_TEST=1" -C tests all
cargo test --features acceptance_tests ${CARGO_FLAGS}
