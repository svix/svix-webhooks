#!/bin/sh -e

# cleanly run the following commands in their own session
bash -c "
# tell Rust to run with coverage instrumentation
RUSTFLAGS=\"-Cinstrument-coverage\"
# give grcov a profile name template for output files
LLVM_PROFILE_FILE=\"svix-webhooks-%p-%m.profraw\"
# put the compiler in nightly mode
RUSTC_BOOTSTRAP=1

# run tests
./run-tests.sh
" || true

# generate and open report output
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/

LOCATION="coverage HTML file at $(pwd)/target/debug/coverage/index.html"
echo "$LOCATION"
