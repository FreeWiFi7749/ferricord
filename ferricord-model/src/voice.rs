//! Voice-related models

use serde::{Deserialize, Serialize};
use crate::id::{ChannelId, GuildId, UserId};
use crate::guild::Member;

/// Represents a user's voice connection status.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VoiceState {
    /// Guild ID this voice state is for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    /// Channel ID this user is connected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,
    /// User ID this voice state is for.
    pub user_id: UserId,
    /// Guild member this voice state is for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,
    /// Session ID for this voice state.
    pub session_id: String,
    /// Whether this user is deafened by the server.
    pub deaf: bool,
    /// Whether this user is muted by the server.
    pub mute: bool,
    /// Whether this user is locally deafened.
    pub self_deaf: bool,
    /// Whether this user is locally muted.
    pub self_mute: bool,
    /// Whether this user is streaming using "Go Live".
    #[serde(default)]
    pub self_stream: bool,
    /// Whether this user's camera is enabled.
    pub self_video: bool,
    /// Whether this user's permission to speak is denied.
    pub suppress: bool,
    /// Time at which the user requested to speak.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_to_speak_timestamp: Option<String>,
}

impl VoiceState {
    /// Returns true if the user is connected to a voice channel.
    pub fn is_connected(&self) -> bool {
        self.channel_id.is_some()
    }

    /// Returns true if the user is deafened (server or self).
    pub fn is_deafened(&self) -> bool {
        self.deaf || self.self_deaf
    }

    /// Returns true if the user is muted (server or self).
    pub fn is_muted(&self) -> bool {
        self.mute || self.self_mute
    }
}

/// Represents a voice region.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VoiceRegion {
    /// Unique ID for the region.
    pub id: String,
    /// Name of the region.
    pub name: String,
    /// True for a single server that is closest to the current user's client.
    pub optimal: bool,
    /// Whether this is a deprecated voice region.
    pub deprecated: bool,
    /// Whether this is a custom voice region (used for events/etc).
    pub custom: bool,
}

/// Voice channel effect send event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VoiceChannelEffectSendEvent {
    /// ID of the channel the effect was sent in.
    pub channel_id: ChannelId,
    /// ID of the guild the effect was sent in.
    pub guild_id: GuildId,
    /// ID of the user who sent the effect.
    pub user_id: UserId,
    /// The emoji sent, for emoji reaction and soundboard effects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<crate::guild::Emoji>,
    /// The type of emoji animation, for emoji reaction and soundboard effects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_type: Option<u8>,
    /// The ID of the emoji animation, for emoji reaction and soundboard effects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_id: Option<u64>,
    /// The ID of the soundboard sound, for soundboard effects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound_id: Option<String>,
    /// The volume of the soundboard sound, from 0 to 1, for soundboard effects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound_volume: Option<f64>,
}

/// Stage instance structure.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StageInstance {
    /// ID of this Stage instance.
    pub id: String,
    /// Guild ID of the associated Stage channel.
    pub guild_id: GuildId,
    /// ID of the associated Stage channel.
    pub channel_id: ChannelId,
    /// Topic of the Stage instance (1-120 characters).
    pub topic: String,
    /// Privacy level of the Stage instance.
    pub privacy_level: StagePrivacyLevel,
    /// Whether or not Stage Discovery is disabled.
    #[serde(default)]
    pub discoverable_disabled: bool,
    /// ID of the scheduled event for this Stage instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_scheduled_event_id: Option<String>,
}

/// Stage privacy level.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(from = "u8", into = "u8")]
pub enum StagePrivacyLevel {
    /// The Stage instance is visible publicly.
    Public = 1,
    /// The Stage instance is visible to only guild members.
    #[default]
    GuildOnly = 2,
}

impl From<u8> for StagePrivacyLevel {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Public,
            _ => Self::GuildOnly,
        }
    }
}

impl From<StagePrivacyLevel> for u8 {
    fn from(value: StagePrivacyLevel) -> Self {
        match value {
            StagePrivacyLevel::Public => 1,
            StagePrivacyLevel::GuildOnly => 2,
        }
    }
}
