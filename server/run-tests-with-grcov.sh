#!/bin/sh -e

# install deps
cargo install grcov
rustup component add llvm-tools-preview

# set env vars
RUSTFLAGS="-Cinstrument-coverage"
LLVM_PROFILE_FILE="svix-webhooks-%p-%m.profraw"
RUSTC_BOOTSTRAP=1

# run tests
./run-tests.sh

# back to stable
RUSTC_BOOTSTRAP=0

# generate and open report output
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
open ./target/debug/coverage/index.html
