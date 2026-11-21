# 6. Release

- Bump `[workspace.package] version` and add a `CHANGELOG.md` entry.
- Run `./scripts/prepare.sh --check`.
- Tag `vX.Y.Z`. Build the container with `docker build -t essential .` (distroless, non-root).
- To publish a library crate to crates.io, set `publish = true` in its manifest, fill in `description`, `repository` and `license`, and run `cargo publish -p <crate>` (ask the maintainer first).
