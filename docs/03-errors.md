# 3. Errors

- Library crates return `essential_core::Result<T>`. `essential_core::Error` is a `thiserror` enum with stable variants (`NotFound`, `Invalid`, `Config`). Add a variant when callers need to react differently; otherwise reuse one.
- Only the binary uses `anyhow`, to add context and print a readable message. Library errors convert with `?`.
- Never `unwrap()` or `expect()` outside tests. Return the error or handle it.
- Error messages are lowercase, no trailing punctuation, and say what went wrong, not what to do.
