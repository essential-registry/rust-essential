use thiserror::Error;

/// The error type shared by every crate in the workspace.
///
/// Libraries return this typed error; only the binary turns it into `anyhow::Error`.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    /// A requested item does not exist.
    #[error("{0} not found")]
    NotFound(String),
    /// The caller supplied invalid input.
    #[error("invalid input: {0}")]
    Invalid(String),
    /// Configuration is missing or malformed.
    #[error("configuration error: {0}")]
    Config(String),
}

/// Result alias using the workspace [`enum@Error`].
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn messages_are_readable() {
        assert_eq!(
            Error::NotFound("todo 3".into()).to_string(),
            "todo 3 not found"
        );
        assert_eq!(
            Error::Invalid("empty name".into()).to_string(),
            "invalid input: empty name"
        );
    }
}
