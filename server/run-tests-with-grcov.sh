rustup component add llvm-tools-preview

RUSTFLAGS="-Cinstrument-coverage"
LLVM_PROFILE_FILE="svix-webhooks-%p-%m.profraw"

./run-tests.sh

grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
