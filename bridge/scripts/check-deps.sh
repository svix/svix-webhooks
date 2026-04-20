#!/bin/bash

BINPATH="$1"

if [[ -z "$BINPATH" ]]; then
    echo >&2 "Usage $0 path/to/binary"
    exit 2
fi

IFS=
MISSING="$(ldd "$BINPATH" | grep 'not found')"

# There'll be 1 linebreak for the empty case. Anything else and it means we
# caught some unexpected output from ldd.
if [ "1" != "$(echo "$MISSING" | wc -l)" ]; then
    echo -e "Binary $BINPATH missing dependencies:\n$MISSING"
    exit 1
fi
