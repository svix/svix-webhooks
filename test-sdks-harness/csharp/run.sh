#!/usr/bin/env bash
set -euo pipefail
DIR="$(cd "$(dirname "$0")" && pwd)"
# shellcheck source=../lib.sh
source "$DIR/../lib.sh"
harness_init
require_fixtures
DOTNET="$(dotnet_bin)"
cd "$DIR"
"$DOTNET" run --project Harness.csproj -f net8.0 --no-restore 2>/dev/null || \
  "$DOTNET" run --project Harness.csproj -f net8.0
