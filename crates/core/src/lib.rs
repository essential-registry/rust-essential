//! Shared building blocks: one error type, one result alias, and configuration.
//!
//! This crate has no IO and no knowledge of any feature. Feature crates depend on it;
//! it never depends on them.

mod config;
mod error;

pub use config::Config;
pub use error::{Error, Result};
