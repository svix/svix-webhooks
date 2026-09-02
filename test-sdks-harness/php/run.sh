#!/usr/bin/env bash
set -euo pipefail
DIR="$(cd "$(dirname "$0")" && pwd)"
# shellcheck source=../lib.sh
source "$DIR/../lib.sh"
harness_init
require_fixtures

if command -v php >/dev/null 2>&1; then
  php "$DIR/run.php"
  exit 0
fi

if ! command -v docker >/dev/null 2>&1; then
  echo "php not on PATH and docker not available" >&2
  exit 1
fi

docker run --rm --network host \
  -e AUTOCONFIG_FIXTURES=/fixtures/fixtures.json \
  -v "$AUTOCONFIG_FIXTURES:/fixtures/fixtures.json:ro" \
  -v "$REPO_ROOT:/app" \
  -w /app \
  php:8.3-cli php /app/test-sdks-harness/php/run.php
