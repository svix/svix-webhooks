#!/usr/bin/env bash

set -ex

if [[ "${PYTHON_VERSION:-x}" = pypy* ]]; then
    echo >&2 "mypy doesn't run under pypy. :-("
else
    mypy svix
fi

ty check svix/
ty check tests/
ruff check svix/
ruff check tests/
ruff format --check svix/
ruff format --check tests/
