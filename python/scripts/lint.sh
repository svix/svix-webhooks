#!/usr/bin/env bash

set -ex

mypy svix
ty check svix/
ty check tests/
ruff check svix/
ruff check tests/
ruff format --check svix/
ruff format --check tests/
