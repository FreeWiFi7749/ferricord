//! Error types for Ferricord
//!
//! This module defines the error types used throughout the Ferricord library.

use thiserror::Error;

/// The main error type for Ferricord operations.
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP request failed
    #[error("HTTP error: {0}")]
    Http(String),

    /// WebSocket connection error
    #[error("WebSocket error: {0}")]
    WebSocket(String),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Gateway connection error
    #[error("Gateway error: {0}")]
    Gateway(String),

    /// Authentication error (invalid token, etc.)
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Rate limit exceeded
    #[error("Rate limited: retry after {retry_after_ms}ms")]
    RateLimited {
        /// Time to wait before retrying in milliseconds
        retry_after_ms: u64,
        /// Whether this is a global rate limit
        global: bool,
    },

    /// Discord API error response
    #[error("Discord API error {code}: {message}")]
    Discord {
        /// Discord error code
        code: i32,
        /// Error message
        message: String,
    },

    /// Invalid data received from Discord
    #[error("Invalid data: {0}")]
    InvalidData(String),

    /// Cache miss - requested data not found in cache
    #[error("Cache miss: {0}")]
    CacheMiss(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// URL parse error
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),
}

/// A specialized Result type for Ferricord operations.
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Create a new HTTP error
    pub fn http(msg: impl Into<String>) -> Self {
        Self::Http(msg.into())
    }

    /// Create a new WebSocket error
    pub fn websocket(msg: impl Into<String>) -> Self {
        Self::WebSocket(msg.into())
    }

    /// Create a new Gateway error
    pub fn gateway(msg: impl Into<String>) -> Self {
        Self::Gateway(msg.into())
    }

    /// Create a new authentication error
    pub fn auth(msg: impl Into<String>) -> Self {
        Self::Auth(msg.into())
    }

    /// Create a new Discord API error
    pub fn discord(code: i32, message: impl Into<String>) -> Self {
        Self::Discord {
            code,
            message: message.into(),
        }
    }

    /// Create a new invalid data error
    pub fn invalid_data(msg: impl Into<String>) -> Self {
        Self::InvalidData(msg.into())
    }

    /// Create a new cache miss error
    pub fn cache_miss(msg: impl Into<String>) -> Self {
        Self::CacheMiss(msg.into())
    }

    /// Create a new internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }

    /// Check if this error is a rate limit error
    pub fn is_rate_limited(&self) -> bool {
        matches!(self, Self::RateLimited { .. })
    }

    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::RateLimited { .. }
                | Self::WebSocket(_)
                | Self::Gateway(_)
                | Self::Http(_)
        )
    }
}
