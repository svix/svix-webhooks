#!/usr/bin/env bash

set -ex

mypy svix
ruff check svix
ruff format --check svix
