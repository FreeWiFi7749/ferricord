//! Ferricord Gateway - Discord Gateway WebSocket client
//!
//! This crate provides a WebSocket client for connecting to the Discord Gateway.

pub mod shard;
pub mod connection;
pub mod event;

pub use shard::{Shard, ShardConfig};
pub use connection::GatewayConnection;
pub use event::EventHandler;
