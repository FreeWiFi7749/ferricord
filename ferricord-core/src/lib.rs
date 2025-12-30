//! Ferricord Core - Common utilities and error types
//!
//! This crate provides shared functionality used across all Ferricord crates.

pub mod error;

pub use error::{Error, Result};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::error::{Error, Result};
}
