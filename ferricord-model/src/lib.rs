//! Ferricord Model - Discord API model definitions
//!
//! This crate provides type-safe representations of Discord API objects.

pub mod id;
pub mod user;
pub mod guild;
pub mod channel;
pub mod message;
pub mod gateway;
pub mod interaction;
pub mod voice;
pub mod permissions;

pub use id::*;
pub use user::*;
pub use guild::*;
pub use channel::*;
pub use message::*;
pub use gateway::*;
pub use interaction::*;
pub use voice::*;
pub use permissions::*;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::id::*;
    pub use crate::user::{User, CurrentUser};
    pub use crate::guild::{Guild, PartialGuild, Member, Role};
    pub use crate::channel::{Channel, ChannelType};
    pub use crate::message::{Message, Embed};
    pub use crate::gateway::{GatewayEvent, Intents};
    pub use crate::permissions::Permissions;
}
