#!/bin/sh
set -e

make -C external/mooneye-test-suite clean all
cargo test --release -p mooneye-gb-core --test mooneye_suite
