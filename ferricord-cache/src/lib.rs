//! Ferricord Cache - Caching layer for Discord data
//!
//! This crate provides a flexible caching system for Discord data with
//! configurable policies for memory management.

pub mod policy;
pub mod store;

pub use policy::{CachePolicy, CachePolicyKind};
pub use store::Cache;
