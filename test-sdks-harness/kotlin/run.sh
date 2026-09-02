#!/usr/bin/env bash
set -euo pipefail
DIR="$(cd "$(dirname "$0")" && pwd)"
# shellcheck source=../lib.sh
source "$DIR/../lib.sh"
harness_init
require_fixtures
java_home_17
export PATH="$JAVA_HOME/bin:$PATH"

DEST="$REPO_ROOT/kotlin/lib/src/test/com/svix/kotlin/AcItHarness.kt"
RESULTS="$DIR/last-results.txt"
mkdir -p "$(dirname "$DEST")"
cp "$DIR/AcIt.kt" "$DEST"
cleanup() { rm -f "$DEST"; }
trap cleanup EXIT

export AUTOCONFIG_RESULTS="$RESULTS"
rm -f "$RESULTS"

cd "$REPO_ROOT/kotlin"
set +e
./gradlew --quiet test --tests com.svix.kotlin.AcItHarness
rc=$?
set -e

if [[ -f "$RESULTS" ]]; then
  cat "$RESULTS"
  exit 0
fi
echo "LANG: kotlin"
echo "A: FAIL — gradle test did not write results (exit $rc)"
echo "B: FAIL — gradle test did not write results (exit $rc)"
echo "C: FAIL — gradle test did not write results (exit $rc)"
echo "D: FAIL — gradle test did not write results (exit $rc)"
echo "Notes: AcItHarness.kt is copied into the Kotlin test source set for the run"
exit 1
