# 2. Naming and style

- Crates: package `essential-feature-<name>`, directory `crates/feature-<name>`. Library crate names are snake_case (`essential_feature_todo`).
- Types `UpperCamelCase`, functions and modules `snake_case`, constants `SCREAMING_SNAKE_CASE`.
- Every public item has a doc comment (`missing_docs` is on). Examples in docs are doc-tests, so they stay correct.
- Format with `cargo fmt` (`rustfmt.toml`: 100 columns). Do not hand-format.
- Lints: `[workspace.lints]` denies `unwrap`, `expect`, `panic`, `todo`, `dbg` and `unsafe`, warns on clippy pedantic. The gate runs clippy with `-D warnings`, so warnings fail. An `#[allow(...)]` needs a comment explaining why.
