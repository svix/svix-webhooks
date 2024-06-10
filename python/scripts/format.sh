#!/bin/sh -e
set -x

ruff check --fix svix
ruff format svix
