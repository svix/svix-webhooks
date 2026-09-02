#!/usr/bin/env bash
# Mint fixtures (unless --skip-setup) and run A–E in every SDK.
#
#   SVIX_TOKEN=testsk_... ./run.sh
#   ./run.sh --skip-setup csharp ruby
#   LANGS=javascript,python ./run.sh
set -euo pipefail

HARNESS_DIR="$(cd "$(dirname "$0")" && pwd)"
# shellcheck source=lib.sh
source "$HARNESS_DIR/lib.sh"
harness_init

SKIP_SETUP=0
SETUP_ONLY=0
ARGS=()
for arg in "$@"; do
  case "$arg" in
    --skip-setup) SKIP_SETUP=1 ;;
    --setup-only) SETUP_ONLY=1 ;;
    -h|--help)
      sed -n '2,12p' "$0"
      exit 0
      ;;
    *) ARGS+=("$arg") ;;
  esac
done

ALL=(javascript python go rust java kotlin csharp ruby php)
if [[ ${#ARGS[@]} -gt 0 ]]; then
  SELECTED=("${ARGS[@]}")
elif [[ -n "${LANGS:-}" ]]; then
  IFS=',' read -r -a SELECTED <<< "$LANGS"
else
  SELECTED=("${ALL[@]}")
fi

if [[ "$SKIP_SETUP" -eq 0 ]]; then
  if [[ -z "${SVIX_TOKEN:-}" ]]; then
    echo "SVIX_TOKEN is required for setup (org token). Pass --skip-setup if fixtures.json already exists." >&2
    exit 1
  fi
  python3 "$HARNESS_DIR/setup.py"
fi

require_fixtures

if [[ "$SETUP_ONLY" -eq 1 ]]; then
  exit 0
fi

failed=0
for lang in "${SELECTED[@]}"; do
  runner="$HARNESS_DIR/$lang/run.sh"
  if [[ ! -x "$runner" ]]; then
    echo "======== $lang ======== missing $runner" >&2
    failed=1
    continue
  fi
  echo "======== $lang ========"
  set +e
  out="$("$runner" 2>&1)"
  rc=$?
  set -e
  printf '%s\n' "$out"
  if [[ $rc -ne 0 ]]; then
    failed=1
    continue
  fi
  if printf '%s\n' "$out" | grep -E -q '^[ABCDE]: (FAIL|BLOCKED)'; then
    failed=1
  fi
done

if [[ $failed -ne 0 ]]; then
  echo "one or more languages failed (see FAILURES.md)" >&2
  exit 1
fi
