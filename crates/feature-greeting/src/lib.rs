//! Greeting feature. Depends only on `essential-core`.

use essential_core::{Error, Result};

/// Build a greeting for `name`, signed by `app_name`.
///
/// ```
/// let text = essential_feature_greeting::greet("Ada", "Essential").unwrap();
/// assert_eq!(text, "Hello, Ada! Welcome to Essential.");
/// ```
pub fn greet(name: &str, app_name: &str) -> Result<String> {
    let name = name.trim();
    if name.is_empty() {
        return Err(Error::Invalid("name must not be empty".into()));
    }
    Ok(format!("Hello, {name}! Welcome to {app_name}."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_by_name() {
        assert_eq!(greet(" Ada ", "X").unwrap(), "Hello, Ada! Welcome to X.");
    }

    #[test]
    fn rejects_blank_names() {
        assert!(matches!(greet("  ", "X"), Err(Error::Invalid(_))));
    }
}
