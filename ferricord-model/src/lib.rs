//! Ferricord Model - Discord API model definitions
//!
//! This crate provides type-safe representations of Discord API objects.

pub mod channel;
pub mod gateway;
pub mod guild;
pub mod id;
pub mod interaction;
pub mod message;
pub mod permissions;
pub mod user;
pub mod voice;

pub use channel::*;
pub use gateway::*;
pub use guild::*;
pub use id::*;
pub use interaction::*;
pub use message::*;
pub use permissions::*;
pub use user::*;
pub use voice::*;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::channel::{Channel, ChannelType};
    pub use crate::gateway::{GatewayEvent, Intents};
    pub use crate::guild::{Guild, Member, PartialGuild, Role};
    pub use crate::id::*;
    pub use crate::message::{Embed, Message};
    pub use crate::permissions::Permissions;
    pub use crate::user::{CurrentUser, User};
}
