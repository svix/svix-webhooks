#!/usr/bin/env bash

set -ex

cd svix

# hack: CI will run this file and fail, this is because openapi-python-client was removed as a dependency


# if [ ! -d "./internal" ]; then
#     COMMAND="generate"
# else
#     COMMAND="update"
# fi

# openapi-python-client $COMMAND --path ../../lib-openapi.json --custom-template-path=../templates --config ../openapi-generator-config.json

# cd internal

# touch __init__.py
