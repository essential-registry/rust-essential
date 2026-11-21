# Rust Essential: guide for AI agents

Rust (edition 2024) Cargo workspace: a `core` crate, feature-based crates, and a thin `app` binary. rustfmt, clippy (warnings denied, unwrap/expect/panic denied), cargo-deny, tests, docs, CI.

## Commands

```bash
cargo run -p essential-app -- greet Ada   # run the binary
cargo test --workspace                     # tests
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all                            # format
./scripts/prepare.sh                       # fix mode: fmt, then every check
./scripts/prepare.sh --check               # the CI gate (non-mutating)
just prepare | just ci                     # same, via just (optional)
```

The gate is `./scripts/prepare.sh --check`. CI runs the same thing, so a change that passes locally passes in CI.

## Layout

```text
Cargo.toml                 workspace: shared package metadata, dependency versions, lints
crates/
  core/                    essential-core: Error, Result, Config. No IO, depends on no feature
  feature-<name>/          one crate per feature; depends only on essential-core
    feature-greeting/ feature-todo/   examples: copy one to start a feature
  app/                     essential-app (binary `essential`): wires the features. The only crate that knows all of them
scripts/prepare.sh  justfile  rust-toolchain.toml  rustfmt.toml  clippy.toml  deny.toml
docs/
```

## Conventions

- Dependency direction: `app` -> `feature-*` -> `core`. Features never depend on each other; `core` never depends on a feature. If two features need to share something, move it to `core`.
- External dependency versions are declared once in the root `[workspace.dependencies]`; crates use `workspace = true`.
- Libraries return `essential_core::Result<T>` (typed `thiserror` errors). Only the binary uses `anyhow`.
- `unwrap()`, `expect()`, `panic!`, `todo!` and `dbg!` are denied outside tests. `unsafe` is forbidden workspace-wide.
- Every public item has a doc comment (`missing_docs` warns; the gate denies warnings).
- Configuration comes from `essential_core::Config::from_env`; document each variable in `.env.example`.
- Unit tests live in the file (`#[cfg(test)]`), end-to-end tests in `crates/<crate>/tests/`.
- New crate: add `crates/feature-<name>` with `[lints] workspace = true`, register it in `[workspace.dependencies]`, and wire it in `crates/app`.

## Rules (all Essential templates)

- **Git:** commits and pushes use the user's own git identity. Never set a Claude identity and never add `Co-Authored-By` or "Generated with" lines. Do not commit unless asked.
- Never read or print `.env` files or secrets. Every setting has an entry in `.env.example`.
- Conventional commits (`feat(scope): ...`). Feature branch, then pull request.
- Business numbers (limits, durations, thresholds) are configuration, never constants in code.
- If a rule is missing, stop and ask. Do not invent one.
- Run the gate before saying "done".

## Never

- Add `unsafe` code or allow the `unsafe_code` lint.
- Use `unwrap()`/`expect()` in non-test code.
- Add a dependency with its own version in a crate manifest.
- Make a feature crate depend on another feature crate.
- Silence clippy with `#[allow]` without a comment saying why.

## Read next

- `docs/01-architecture.md`, `docs/02-naming-and-style.md`, `docs/03-errors.md`, `docs/04-testing.md`, `docs/05-tooling.md`, `docs/06-release.md`
