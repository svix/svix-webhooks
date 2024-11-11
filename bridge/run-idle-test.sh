#!/bin/sh

cargo build

# We've seen some segfauls from deno that require the interpreter running
# for a period. The test here is to watch the bridge process and make sure
# it is still alive after some time has passed.
RUST_LOG="notset" target/debug/svix-bridge --cfg '{}' &
SVIX_BRIDGE_PID=$!
echo "Monitoring PID=$SVIX_BRIDGE_PID"
sleep 15
ps $SVIX_BRIDGE_PID > /dev/null
if [ "$?" -ne 0 ]; then
    echo "fail: process terminated prematurely"
    exit 1
fi
echo "success: process stayed up"
kill $SVIX_BRIDGE_PID
