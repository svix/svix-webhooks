#!/usr/bin/env bash
set -euo pipefail
DIR="$(cd "$(dirname "$0")" && pwd)"
# shellcheck source=../lib.sh
source "$DIR/../lib.sh"
harness_init
require_fixtures
cd "$REPO_ROOT/python"
if command -v uv >/dev/null 2>&1; then
  uv run python "$DIR/run.py"
else
  PYTHONPATH="$REPO_ROOT/python${PYTHONPATH:+:$PYTHONPATH}" python3 "$DIR/run.py"
fi
