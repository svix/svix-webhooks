#!/usr/bin/env bash

set -ex

mypy svix
ty check svix
ruff check svix
ruff format --check svix
