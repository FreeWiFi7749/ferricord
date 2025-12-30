//! Ferricord Gateway - Discord Gateway WebSocket client
//!
//! This crate provides a WebSocket client for connecting to the Discord Gateway.

pub mod connection;
pub mod event;
pub mod shard;

pub use connection::GatewayConnection;
pub use event::EventHandler;
pub use shard::{Shard, ShardConfig};
