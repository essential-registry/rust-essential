# 1. Architecture

```text
crates/app  ->  crates/feature-*  ->  crates/core
```

| Crate | Role | May depend on |
| --- | --- | --- |
| `essential-core` | Shared `Error`, `Result`, `Config`. No IO. | external crates only |
| `essential-feature-<name>` | One feature: its types, rules and tests. | `essential-core` |
| `essential-app` | Binary `essential`: CLI, logging, wiring. | everything |

Rules:

- Features never depend on each other. If two features need the same thing, move it to `core`.
- `core` never depends on a feature.
- Logic lives in libraries; the binary stays thin so it is easy to replace (CLI today, HTTP server tomorrow).

## Workspace

The root `Cargo.toml` holds shared package metadata (`[workspace.package]`), every external dependency version (`[workspace.dependencies]`) and the lint policy (`[workspace.lints]`). Crates opt in with `workspace = true` and `[lints] workspace = true`.

## Adding a feature

1. Copy `crates/feature-todo` to `crates/feature-<name>` and rename the package.
2. Add it to `[workspace.dependencies]` in the root manifest.
3. Depend on it from `crates/app` and wire it in `main.rs`.
