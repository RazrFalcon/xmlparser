#!/bin/bash

# exit on error, verbose
set -ev

# we can't test this crate with Rust < 1.18
if [ "$TRAVIS_RUST_VERSION" == "1.16.0" ]; then
    cargo build --verbose
else
    cargo test --verbose
fi
