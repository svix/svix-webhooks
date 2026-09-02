# shellcheck shell=bash
# Shared by test-sdks-harness/run.sh and language runners.

harness_init() {
  HARNESS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  REPO_ROOT="$(cd "$HARNESS_DIR/.." && pwd)"
  export AUTOCONFIG_FIXTURES="${AUTOCONFIG_FIXTURES:-$HARNESS_DIR/fixtures.json}"
  export SVIX_SERVER_URL="${SVIX_SERVER_URL:-http://localhost:8071}"
}

require_fixtures() {
  if [[ ! -f "$AUTOCONFIG_FIXTURES" ]]; then
    echo "missing $AUTOCONFIG_FIXTURES — run test-sdks-harness/setup.py first (or ./run.sh)" >&2
    exit 1
  fi
}

java_home_17() {
  if [[ -n "${JAVA_HOME:-}" && -x "$JAVA_HOME/bin/java" ]]; then
    if "$JAVA_HOME/bin/java" -version 2>&1 | grep -q 'version "17'; then
      return
    fi
  fi
  if [[ -x /usr/libexec/java_home ]]; then
    local home
    home="$(/usr/libexec/java_home -v 17 2>/dev/null || true)"
    if [[ -n "$home" ]]; then
      export JAVA_HOME="$home"
      return
    fi
  fi
  if [[ -d /opt/homebrew/Cellar/openjdk@17 ]]; then
    local latest
    latest="$(ls -1d /opt/homebrew/Cellar/openjdk@17/*/libexec/openjdk.jdk/Contents/Home 2>/dev/null | tail -1)"
    if [[ -n "$latest" ]]; then
      export JAVA_HOME="$latest"
      return
    fi
  fi
  echo "need JDK 17 (JAVA_HOME). JDK 21+ / 26 breaks the Kotlin SDK build." >&2
  exit 1
}

dotnet_bin() {
  if [[ -x "${HOME}/.dotnet/dotnet" ]]; then
    echo "${HOME}/.dotnet/dotnet"
    return
  fi
  if command -v dotnet >/dev/null 2>&1; then
    command -v dotnet
    return
  fi
  echo "dotnet not found (expected ~/.dotnet/dotnet or PATH)" >&2
  exit 1
}

ruby_env() {
  if [[ -d "${HOME}/.gem/ruby/4.0.0/bin" ]]; then
    export PATH="${HOME}/.gem/ruby/4.0.0/bin:$PATH"
  fi
  export RUBYLIB="${REPO_ROOT}/ruby/lib${RUBYLIB:+:$RUBYLIB}"
}
