# 5. Tooling

| Task | Command | Tool |
| --- | --- | --- |
| Format | `cargo fmt --all` | rustfmt |
| Lint | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | clippy |
| Tests | `cargo test --workspace` (or `cargo nextest run`) | cargo / nextest |
| Docs | `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` | rustdoc |
| Dependency policy | `cargo deny check` | cargo-deny (`deny.toml`: licences, advisories, bans) |
| Advisories | `cargo audit` | cargo-audit |
| Unused dependencies | `cargo machete` | cargo-machete |
| Task runner | `just` | just (optional) |

Install the optional tools once: `cargo install cargo-nextest cargo-deny cargo-audit cargo-machete just`.

## The gate

`./scripts/prepare.sh` formats, then runs every check. `./scripts/prepare.sh --check` changes nothing and is what CI runs. Optional tools are skipped locally when missing (set `REQUIRE_TOOLS=1` to fail instead) and are required in CI.

The toolchain is `stable` (see `rust-toolchain.toml`); the minimum supported Rust version is set in `rust-version` and `clippy.toml`.
