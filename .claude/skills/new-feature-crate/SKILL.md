---
name: new-feature-crate
description: Add a feature crate under crates/ and wire it into the app.
---

Copy `crates/feature-todo` to `crates/feature-<name>`, rename the package to `essential-feature-<name>`, add it to `[workspace.dependencies]`, depend on it from `crates/app`, add unit tests, run `./scripts/prepare.sh`. It may depend only on `essential-core`.
