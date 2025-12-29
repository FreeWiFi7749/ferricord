//! Ferricord HTTP - Discord REST API client
//!
//! This crate provides a rate-limited HTTP client for the Discord REST API.

pub mod client;
pub mod ratelimit;
pub mod routes;

pub use client::HttpClient;
pub use ratelimit::{RateLimiter, RateLimitInfo};
