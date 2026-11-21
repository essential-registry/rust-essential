# Rust Essential

A Rust (edition 2024) workspace starter: a `core` crate, feature-based crates and a thin binary, with the quality gate, lint policy, docs, CI and agent guide already set up.

## Layout

```text
crates/
  core/             shared Error, Result, Config (no IO)
  feature-greeting/ example feature
  feature-todo/     example feature
  app/              binary `essential` that wires the features
docs/               architecture, style, errors, testing, tooling, release
scripts/prepare.sh  the quality gate
```

Dependencies point one way: `app -> feature-* -> core`. Features never depend on each other.

## Run

```bash
cargo run -p essential-app -- greet Ada
cargo run -p essential-app -- todo
```

## Quality gate

```bash
./scripts/prepare.sh            # fix: cargo fmt, then every check
./scripts/prepare.sh --check    # what CI runs
```

It runs rustfmt, clippy (warnings denied), tests, rustdoc, cargo-deny, cargo-audit, cargo-machete and a release build. `unwrap`, `expect`, `panic` and `unsafe` are denied by workspace lints.

## Working with AI agents

`AGENTS.md` is the guide agents follow (`CLAUDE.md` points to it). `.claude/` has permissions, a formatter hook, a reviewer agent and skills.

## License

MIT.
