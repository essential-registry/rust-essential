# Rust Essential

[![CI](https://github.com/essential-registry/rust-essential/actions/workflows/ci.yml/badge.svg)](https://github.com/essential-registry/rust-essential/actions/workflows/ci.yml) ![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)

A Rust (edition 2024) workspace starter: a `core` crate, feature-based crates and a thin binary, with a strict lint policy, one quality gate, CI and an agent guide.

Part of [Essential](https://github.com/essential-registry): starter templates with the tooling, structure, docs and AI-agent guide already in place.

## What you get

- **Cargo workspace:** `essential-core` (errors, config), `essential-feature-*` crates, and `essential-app` (the binary)
- **One-way dependencies:** `app -> feature-* -> core`; features never depend on each other
- **Strict lints** (`[workspace.lints]`): `unsafe` forbidden; `unwrap`, `expect`, `panic`, `todo`, `dbg` denied; clippy pedantic as warnings, all denied by the gate
- **Typed errors** with `thiserror` in libraries, `anyhow` only in the binary; configuration from the environment, testable without touching it
- **Tests** at three levels: unit, doc-tests and an end-to-end test that runs the real binary
- **Quality gate** script (fmt, clippy, tests, rustdoc, cargo-deny, cargo-audit, cargo-machete, release build), **justfile**, **GitHub Actions** on Linux and macOS
- **Dockerfile** (distroless, non-root), **docs and AGENTS.md**

## Requirements

- [Rust](https://rustup.rs) stable (1.90 or newer; the toolchain is pinned to `stable` in `rust-toolchain.toml`)
- Optional: `cargo install cargo-nextest cargo-deny cargo-audit cargo-machete just`

## Quick start

Click **Use this template** on GitHub (or run `gh repo create my-project --template essential-registry/rust-essential`), then:

```bash
cargo run -p essential-app -- greet Ada
cargo run -p essential-app -- todo
cargo test --workspace
```

Configuration is read from environment variables (see `.env.example`).

## Commands

| Command | What it does |
| --- | --- |
| `cargo run -p essential-app -- <command>` | Run the binary (`greet <name>`, `todo`) |
| `cargo test --workspace` | Unit, doc and end-to-end tests |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | Lint |
| `cargo fmt --all` | Format |
| `./scripts/prepare.sh` | Fix mode: format, then every check |
| `./scripts/prepare.sh --check` | The non-mutating gate CI runs |
| `just prepare` / `just ci` | The same through `just` (optional) |

## Project structure

```text
crates/
  core/               essential-core: Error, Result, Config (no IO)
  feature-greeting/   example feature
  feature-todo/       example feature (in-memory list)
  app/                essential-app: the `essential` binary, wires the features
docs/                 architecture, style, errors, testing, tooling, release
scripts/prepare.sh    the quality gate
Cargo.toml            workspace: shared metadata, dependency versions, lints
deny.toml  clippy.toml  rustfmt.toml  rust-toolchain.toml
```

## Architecture

`core` holds only shared types and has no IO or knowledge of features. Each `feature-*` crate depends on `core` alone, and `app` is the only crate that knows them all, so a feature can be added, removed or reused without touching the others. All external versions are declared once in `[workspace.dependencies]`.

Full details: [`docs/01-architecture.md`](docs/01-architecture.md).

## Documentation

- [`docs/01-architecture.md`](docs/01-architecture.md): layers, folders and how the pieces fit
- [`docs/02-naming-and-style.md`](docs/02-naming-and-style.md): naming and code style
- [`docs/03-errors.md`](docs/03-errors.md): error handling
- [`docs/04-testing.md`](docs/04-testing.md): how to test and what the gate checks
- [`docs/05-tooling.md`](docs/05-tooling.md): tools, presets and the quality gate
- [`docs/06-release.md`](docs/06-release.md): versioning and releasing

## Quality gate and CI

`./scripts/prepare.sh --check` runs `cargo fmt --check`, clippy with warnings denied, tests (nextest when installed), rustdoc with warnings denied, cargo-deny, cargo-audit, cargo-machete and a release build. Optional tools are skipped locally when missing (`REQUIRE_TOOLS=1` makes that a failure) and required in CI, which runs on Ubuntu and macOS.

## Working with AI agents

[`AGENTS.md`](AGENTS.md) is the guide coding agents follow: commands, layout, conventions and a "never" list (`CLAUDE.md` points to it). `.claude/` adds safe permissions, a formatter hook, a read-only reviewer agent and task skills. Commits always use your own git identity; the guide tells agents not to add attribution lines and not to commit unless asked.

## Make it yours

1. Rename the crates and the binary (`essential-*` and `essential`).
2. Replace the example features; copy `crates/feature-todo` for each new one.
3. Update `repository` and `license` in the workspace manifest.
4. Set `publish = true` only on crates you intend to release.

## Author

Built and maintained by [Vaibhav Gosavi](https://vgseven.com). Part of [Essential](https://github.com/essential-registry).

## License

MIT. See [LICENSE](LICENSE). Copyright (c) 2025-2026 Vaibhav Gosavi (https://vgseven.com).
