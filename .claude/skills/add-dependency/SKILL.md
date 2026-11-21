---
name: add-dependency
description: Add an external dependency the workspace way.
---

Add the version to root `[workspace.dependencies]`, reference it with `{ workspace = true }` in the crate, run `cargo deny check` and `./scripts/prepare.sh`. Prefer small, maintained crates; ask before adding one.
