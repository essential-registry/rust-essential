# Task runner (https://github.com/casey/just). `just` lists the recipes.
default:
    @just --list

# Fix formatting, then run every check.
prepare:
    ./scripts/prepare.sh

# The non-mutating gate CI runs.
ci:
    ./scripts/prepare.sh --check

run *args:
    cargo run -p essential-app -- {{args}}

test:
    cargo test --workspace --all-features

lint:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

fmt:
    cargo fmt --all

# Line coverage (needs cargo-llvm-cov).
coverage:
    cargo llvm-cov --workspace --lcov --output-path lcov.info
