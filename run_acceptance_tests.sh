#!/bin/sh

set -e

make WLAFLAGS="-DACCEPTANCE_TEST=1" -C tests clean all
cargo test --features acceptance_tests
