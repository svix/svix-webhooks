#!/usr/bin/env bash
set -euo pipefail
DIR="$(cd "$(dirname "$0")" && pwd)"
# shellcheck source=../lib.sh
source "$DIR/../lib.sh"
harness_init
require_fixtures
cd "$REPO_ROOT/javascript"
npx --yes tsx "$DIR/run.ts"
