#!/usr/bin/env bash
set -euo pipefail
DIR="$(cd "$(dirname "$0")" && pwd)"
# shellcheck source=../lib.sh
source "$DIR/../lib.sh"
harness_init
require_fixtures
java_home_17
export PATH="$JAVA_HOME/bin:$PATH"

cd "$REPO_ROOT/java"
./gradlew --quiet jar --init-script "$HARNESS_DIR/printcp.gradle"
JAR="$(ls -1 build/libs/svix-*.jar | sort -V | tail -1)"
CP="$(./gradlew --quiet --init-script "$HARNESS_DIR/printcp.gradle" printRuntimeClasspath)"

OUT="$DIR/out"
rm -rf "$OUT"
mkdir -p "$OUT"
javac --release 17 -cp "$JAR:$CP" -d "$OUT" "$DIR/AcIt.java"
java -cp "$OUT:$JAR:$CP" AcIt
