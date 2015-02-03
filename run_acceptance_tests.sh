#!/bin/sh

make -C tests
cargo test --features acceptance_tests
