#!/usr/bin/env bash

set -ex

mypy svix
isort --check-only svix
black svix --check
flake8 svix
