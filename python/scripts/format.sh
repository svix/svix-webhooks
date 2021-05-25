#!/bin/sh -e
set -x

autoflake --remove-all-unused-imports --recursive --remove-unused-variables --in-place svix --exclude=__init__.py
isort svix
black svix
