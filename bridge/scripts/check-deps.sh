#!/bin/sh

BINPATH=$1

IFS=
MISSING="$(ldd $BINPATH | grep 'not found')"

# There'll be 1 linebreak for the empty case. Anything else and it means we
# caught some unexpected output from ldd.
if [ "1" != "$(echo $MISSING | wc -l)" ]; then
    echo "Binary $BINPATH missing dependencies:\n$MISSING"
    exit 1
fi
