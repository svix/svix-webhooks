#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "semver",
# ]
# ///
"""
Version management for the svix-webhooks monorepo.

Usage:
  uv run tools/bump_version.py check         # verify all tracked files match .version
  uv run tools/bump_version.py bump <ver>    # propagate new version into all tracked files
"""

import argparse
import re
import subprocess
import sys
from pathlib import Path
from dataclasses import dataclass

import semver

REPO_ROOT = Path(__file__).parent.parent


@dataclass
class VersionFile:
    path: str
    patterns: list[str]
    pattern_flags: int = 0


POST_BUMP_COMMANDS = [
    # Rust
    "cargo update --workspace --manifest-path=server/Cargo.toml",
    "cargo update --workspace --manifest-path=bridge/Cargo.toml",
    "cargo update --workspace --manifest-path=svix-cli/Cargo.toml",
    # JavaScript
    "cd javascript && npm i --package-lock-only --ignore-scripts",
]

VERSION_FILES = [
    # CLI
    VersionFile(
        "svix-cli/Cargo.toml",
        [r'(\[package\][\s\S]*?\nversion\s*=\s*")([^"]*)(")', ],
        re.DOTALL,
    ),
    VersionFile(
        "svix-cli/README.md",
        [r'(download/v)(\d+\.\d+\.\d+)(/svix-cli-installer)'],
    ),
    # Server
    VersionFile(
        "server/Cargo.toml",
        [r'(\[workspace\.package\][\s\S]*?\nversion\s*=\s*")([^"]*)(")', ],
        re.DOTALL,
    ),
    # Bridge
    VersionFile(
        "bridge/Cargo.toml",
        [r'(\[workspace\.package\][\s\S]*?\nversion\s*=\s*")([^"]*)(")', ],
        re.DOTALL,
    ),
    # Rust client
    VersionFile(
        "rust/Cargo.toml",
        [r'(\[package\][\s\S]*?\nversion\s*=\s*")([^"]*)(")', ],
        re.DOTALL,
    ),
    # C#
    VersionFile(
        "csharp/Svix/Svix.csproj",
        [r'(<Version>)([^<]*)(</Version>)'],
    ),
    VersionFile(
        "csharp/Svix/Version.cs",
        [r'(public const string version\s*=\s*")([^"]*)(")', ],
    ),
    # Go
    VersionFile(
        "go/internal/version.go",
        [r'(const Version\s*=\s*")([^"]*)(")', ],
    ),
    # Java
    VersionFile(
        "java/gradle.properties",
        [r'(VERSION_NAME=)([^\n]+)'],
    ),
    VersionFile(
        "java/README.md",
        [
            r'(<version>)([^<]*)(</version>)',
            r'(com\.svix:svix:)([^\s"]+)',
        ],
    ),
    VersionFile(
        "java/lib/src/main/java/com/svix/Version.java",
        [r'(public static final String VERSION\s*=\s*")([^"]*)(")', ],
    ),
    # JavaScript
    VersionFile(
        "javascript/package.json",
        [r'("version"\s*:\s*")([^"]*)(")'],
    ),
    VersionFile(
        "javascript/src/request.ts",
        [r'(export const LIB_VERSION\s*=\s*")([^"]*)(")', ],
    ),
    # Kotlin
    VersionFile(
        "kotlin/gradle.properties",
        [r'(VERSION_NAME=)([^\n]+)'],
    ),
    VersionFile(
        "kotlin/README.md",
        [
            r'(<version>)([^<]*)(</version>)',
            r'(com\.svix\.kotlin:svix-kotlin:)([^\s"]+)',
        ],
    ),
    VersionFile(
        "kotlin/lib/src/main/kotlin/Version.kt",
        [r'(const val Version\s*=\s*")([^"]*)(")', ],
    ),
    # Python
    VersionFile(
        "python/svix/__init__.py",
        [r'(__version__\s*=\s*")([^"]*)(")', ],
    ),
    # Ruby
    VersionFile(
        "ruby/Gemfile.lock",
        [r'(\bsvix \()([^)]+)(\))'],
    ),
    VersionFile(
        "ruby/lib/svix/version.rb",
        [r'(VERSION\s*=\s*")([^"]*)(")', ],
    ),
    # PHP
    VersionFile(
        "php/src/Version.php",
        [r"(const VERSION\s*=\s*')([^']*)(')", ],
    ),
    # OpenAPI spec (anchor to info.title to avoid matching schema version fields)
    VersionFile(
        "codegen/lib-openapi.json",
        [r'("title":\s*"Svix API",\s*\n\s*"version":\s*")([^"]*)(")', ],
        re.DOTALL,
    ),
]


def read_file(path: Path) -> tuple[str, bool]:
    raw = path.read_bytes()
    had_crlf = b"\r\n" in raw
    return raw.decode("utf-8").replace("\r\n", "\n"), had_crlf


def write_file(path: Path, content: str, had_crlf: bool) -> None:
    if had_crlf:
        content = content.replace("\n", "\r\n")
    path.write_bytes(content.encode("utf-8"))


def read_canonical_version() -> str:
    return (REPO_ROOT / ".version").read_text().strip()


def cmd_check() -> int:
    expected = read_canonical_version()
    errors = 0
    for vf in VERSION_FILES:
        content, _ = read_file(REPO_ROOT / vf.path)
        for pattern in vf.patterns:
            m = re.search(pattern, content, vf.pattern_flags)
            if not m:
                print(f"NO MATCH pattern '{pattern}' in {vf.path}", file=sys.stderr)
                errors += 1
                continue
            actual = m.group(2)
            if actual != expected:
                print(
                    f"MISMATCH {vf.path}: got '{actual}', expected '{expected}'",
                    file=sys.stderr,
                )
                errors += 1

    if errors:
        print(
            f"\nERROR: {errors} version mismatch(es) found. All versions must match .version ({expected}).",
            file=sys.stderr,
        )
        return 1

    print(f"All versions match: {expected}")
    return 0


def update_changelog(new_version: str) -> None:
    changelog_path = REPO_ROOT / "ChangeLog.md"
    content = changelog_path.read_text()
    updated = content.replace(
        "# Changelog\n\n## Next",
        f"# Changelog\n\n## Next\n* \n\n## Version {new_version}",
    )
    if updated != content:
        changelog_path.write_text(updated)


def cmd_bump(new_version: str) -> int:
    if not semver.Version.is_valid(new_version):
        print(f"ERROR: '{new_version}' is not a valid semver", file=sys.stderr)
        return 1

    old_version = read_canonical_version()

    if semver.Version.is_valid(old_version) and semver.Version.parse(
        new_version
    ) <= semver.Version.parse(old_version):
        print(
            f"ERROR: new version '{new_version}' is not greater than current '{old_version}'",
            file=sys.stderr,
        )
        return 1

    (REPO_ROOT / ".version").write_text(new_version + "\n")
    version = new_version

    for vf in VERSION_FILES:
        abs_path = REPO_ROOT / vf.path
        content, had_crlf = read_file(abs_path)
        for pattern in vf.patterns:
            m = re.search(pattern, content, vf.pattern_flags)
            if not m:
                print(
                    f"ERROR: no match for pattern '{pattern}' in {vf.path}",
                    file=sys.stderr,
                )
                return 1

            content = re.sub(
                pattern,
                lambda mo, v=version: mo.group(1) + v + (mo.group(3) if mo.lastindex and mo.lastindex >= 3 else ""),
                content,
                flags=vf.pattern_flags,
            )
        write_file(abs_path, content, had_crlf)

    update_changelog(new_version)
    print(f"Updated all versions to: {version}")

    if rc := cmd_check():
        return rc

    for cmd in POST_BUMP_COMMANDS:
        print(f"\n$ {cmd}")
        result = subprocess.run(cmd, shell=True, cwd=REPO_ROOT)
        if result.returncode != 0:
            print(f"ERROR: command failed: {cmd}", file=sys.stderr)
            return result.returncode

    return 0


def main() -> None:
    parser = argparse.ArgumentParser(description="svix-webhooks version management")
    sub = parser.add_subparsers(dest="command", required=True)
    sub.add_parser("check", help="verify all tracked files match .version")
    bump_parser = sub.add_parser(
        "bump", help="set new version in .version and propagate to all tracked files"
    )
    bump_parser.add_argument("version", help="new semver version to set (e.g. 1.2.3)")

    args = parser.parse_args()
    sys.exit(cmd_check() if args.command == "check" else cmd_bump(args.version))


if __name__ == "__main__":
    main()
