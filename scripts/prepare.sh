#!/usr/bin/env sh
# The one quality gate. CI and developers run the same script.
#
#   ./scripts/prepare.sh          fix mode (default locally): cargo fmt, then all checks
#   ./scripts/prepare.sh --check  check mode (default when CI=true): changes nothing
#
# Optional tools (cargo-nextest, cargo-deny, cargo-audit, cargo-machete) run when installed.
# In CI they are installed, so a missing one fails there: set REQUIRE_TOOLS=1 to do the same locally.
set -eu
cd -P "$(dirname "$0")/.."

mode=fix
[ "${CI:-}" = "true" ] && mode=check
for arg in "$@"; do
    case "$arg" in
    --check) mode=check ;;
    --fix) mode=fix ;;
    *) printf '%s\n' "Usage: scripts/prepare.sh [--check|--fix]" >&2; exit 2 ;;
    esac
done
require="${REQUIRE_TOOLS:-}"
[ "${CI:-}" = "true" ] && require=1

step() {
    label=$1; shift
    printf '[gate] START %s\n' "$label"
    if "$@"; then printf '[gate] OK    %s\n' "$label"; else
        status=$?; printf '[gate] FAIL  %s (exit %s)\n' "$label" "$status" >&2; return "$status"
    fi
}

optional() {
    tool=$1; label=$2; shift 2
    if cargo "$tool" --version >/dev/null 2>&1; then step "$label" "$@"
    elif [ -n "$require" ]; then printf '[gate] FAIL  %s: cargo-%s is not installed\n' "$label" "$tool" >&2; return 1
    else printf '[gate] SKIP  %s (install cargo-%s to enable)\n' "$label" "$tool"; fi
}

if [ "$mode" = fix ]; then
    step "format (cargo fmt)" cargo fmt --all
else
    step "format check (cargo fmt)" cargo fmt --all --check
fi
step "lint (clippy, warnings denied)" cargo clippy --workspace --all-targets --all-features -- -D warnings
if cargo nextest --version >/dev/null 2>&1; then
    step "tests (nextest)" cargo nextest run --workspace --all-features
    step "doc tests" cargo test --workspace --doc
else
    step "tests (cargo test)" cargo test --workspace --all-features
fi
RUSTDOCFLAGS="-D warnings" step "docs" cargo doc --workspace --no-deps
optional deny "dependency policy (cargo-deny)" cargo deny check
optional audit "advisories (cargo-audit)" cargo audit
optional machete "unused dependencies (cargo-machete)" cargo machete
step "release build" cargo build --workspace --release
printf '[gate] all checks passed (%s mode)\n' "$mode"
