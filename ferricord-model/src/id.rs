//! Discord ID types
//!
//! All Discord IDs are represented as newtypes wrapping u64 for type safety.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

macro_rules! impl_id {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Default)]
        pub struct $name(pub u64);

        impl $name {
            /// Create a new ID from a u64 value.
            pub const fn new(id: u64) -> Self {
                Self(id)
            }

            /// Get the inner u64 value.
            pub const fn get(self) -> u64 {
                self.0
            }

            /// Get the timestamp when this snowflake was created.
            /// Discord epoch is 2015-01-01T00:00:00.000Z
            pub fn created_at(self) -> chrono::DateTime<chrono::Utc> {
                const DISCORD_EPOCH: u64 = 1420070400000;
                let timestamp_ms = (self.0 >> 22) + DISCORD_EPOCH;
                chrono::DateTime::from_timestamp_millis(timestamp_ms as i64)
                    .unwrap_or_default()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<u64> for $name {
            fn from(id: u64) -> Self {
                Self(id)
            }
        }

        impl From<$name> for u64 {
            fn from(id: $name) -> Self {
                id.0
            }
        }

        impl FromStr for $name {
            type Err = std::num::ParseIntError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                s.parse::<u64>().map(Self)
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.0.to_string())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                s.parse::<u64>()
                    .map(Self)
                    .map_err(serde::de::Error::custom)
            }
        }
    };
}

impl_id!(UserId, "A unique identifier for a Discord user.");
impl_id!(GuildId, "A unique identifier for a Discord guild (server).");
impl_id!(ChannelId, "A unique identifier for a Discord channel.");
impl_id!(MessageId, "A unique identifier for a Discord message.");
impl_id!(RoleId, "A unique identifier for a Discord role.");
impl_id!(EmojiId, "A unique identifier for a Discord custom emoji.");
impl_id!(ApplicationId, "A unique identifier for a Discord application.");
impl_id!(WebhookId, "A unique identifier for a Discord webhook.");
impl_id!(AttachmentId, "A unique identifier for a Discord attachment.");
impl_id!(StickerPackId, "A unique identifier for a Discord sticker pack.");
impl_id!(StickerId, "A unique identifier for a Discord sticker.");
impl_id!(InteractionId, "A unique identifier for a Discord interaction.");
impl_id!(CommandId, "A unique identifier for an application command.");
impl_id!(IntegrationId, "A unique identifier for a Discord integration.");
impl_id!(StageInstanceId, "A unique identifier for a stage instance.");
impl_id!(ScheduledEventId, "A unique identifier for a scheduled event.");
impl_id!(AuditLogEntryId, "A unique identifier for an audit log entry.");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_serialization() {
        let id = UserId::new(123456789012345678);
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"123456789012345678\"");
    }

    #[test]
    fn test_id_deserialization() {
        let json = "\"123456789012345678\"";
        let id: UserId = serde_json::from_str(json).unwrap();
        assert_eq!(id.get(), 123456789012345678);
    }

    #[test]
    fn test_id_from_str() {
        let id: UserId = "123456789012345678".parse().unwrap();
        assert_eq!(id.get(), 123456789012345678);
    }
}
