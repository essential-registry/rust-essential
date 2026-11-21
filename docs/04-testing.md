# 4. Testing

```bash
cargo test --workspace
```

- Unit tests sit beside the code in `#[cfg(test)] mod tests`.
- End-to-end tests are in `crates/<crate>/tests/` (see `crates/app/tests/cli.rs`, which runs the real binary).
- Doc-tests run with `cargo test --doc`.
- Tests may use `unwrap()`/`expect()` (allowed by `clippy.toml`).
- Configuration is tested through `Config::from_lookup`, so tests never modify the process environment.
- Coverage (optional): `just coverage` with `cargo-llvm-cov`.
- If `cargo-nextest` is installed the gate uses it automatically (faster, better output).
