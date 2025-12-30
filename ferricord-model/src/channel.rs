//! Channel-related models

use crate::id::{ChannelId, GuildId, MessageId, UserId};
use crate::permissions::PermissionOverwrite;
use crate::user::User;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Represents a Discord channel.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Channel {
    /// The ID of this channel.
    pub id: ChannelId,
    /// The type of channel.
    #[serde(rename = "type")]
    pub kind: ChannelType,
    /// The ID of the guild (may be missing for some channel objects).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    /// Sorting position of the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    /// Explicit permission overwrites for members and roles.
    #[serde(default)]
    pub permission_overwrites: Vec<PermissionOverwrite>,
    /// The name of the channel (1-100 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The channel topic (0-4096 characters for forum/media channels, 0-1024 for others).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /// Whether the channel is NSFW.
    #[serde(default)]
    pub nsfw: bool,
    /// The ID of the last message sent in this channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<MessageId>,
    /// The bitrate (in bits) of the voice channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,
    /// The user limit of the voice channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<u32>,
    /// Amount of seconds a user has to wait before sending another message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u32>,
    /// The recipients of the DM.
    #[serde(default)]
    pub recipients: Vec<User>,
    /// Icon hash of the group DM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// ID of the creator of the group DM or thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<UserId>,
    /// Application ID of the group DM creator if it is bot-created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<UserId>,
    /// Whether the channel is managed by an application via the gdm.join OAuth2 scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    /// For group DM channels: the ID of the parent category.
    /// For threads: the ID of the text channel this thread was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<ChannelId>,
    /// When the last pinned message was pinned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<String>,
    /// Voice region ID for the voice channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtc_region: Option<String>,
    /// The camera video quality mode of the voice channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quality_mode: Option<VideoQualityMode>,
    /// Number of messages in a thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<u32>,
    /// Approximate count of users in a thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u32>,
    /// Thread-specific fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_metadata: Option<ThreadMetadata>,
    /// Thread member object for the current user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<ThreadMember>,
    /// Default duration for newly created threads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_auto_archive_duration: Option<u32>,
    /// Computed permissions for the invoking user in the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    /// Channel flags.
    #[serde(default)]
    pub flags: u32,
    /// Number of messages ever sent in a thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_message_sent: Option<u32>,
    /// The set of tags that can be used in a forum/media channel.
    #[serde(default)]
    pub available_tags: Vec<ForumTag>,
    /// The IDs of the set of tags that have been applied to a thread.
    #[serde(default)]
    pub applied_tags: Vec<String>,
    /// The emoji to show in the add reaction button on a thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_reaction_emoji: Option<DefaultReaction>,
    /// The initial rate_limit_per_user to set on newly created threads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_thread_rate_limit_per_user: Option<u32>,
    /// The default sort order type used to order posts in forum/media channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sort_order: Option<SortOrderType>,
    /// The default forum layout view used to display posts in forum channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_forum_layout: Option<ForumLayoutType>,
}

impl Channel {
    /// Returns the channel's mention string.
    pub fn mention(&self) -> String {
        format!("<#{}>", self.id)
    }

    /// Returns true if this is a text-based channel.
    pub fn is_text_based(&self) -> bool {
        matches!(
            self.kind,
            ChannelType::GuildText
                | ChannelType::Dm
                | ChannelType::GroupDm
                | ChannelType::GuildAnnouncement
                | ChannelType::AnnouncementThread
                | ChannelType::PublicThread
                | ChannelType::PrivateThread
                | ChannelType::GuildVoice
                | ChannelType::GuildStageVoice
        )
    }

    /// Returns true if this is a voice-based channel.
    pub fn is_voice_based(&self) -> bool {
        matches!(
            self.kind,
            ChannelType::GuildVoice | ChannelType::GuildStageVoice
        )
    }

    /// Returns true if this is a thread.
    pub fn is_thread(&self) -> bool {
        matches!(
            self.kind,
            ChannelType::AnnouncementThread
                | ChannelType::PublicThread
                | ChannelType::PrivateThread
        )
    }

    /// Returns true if this is a DM channel.
    pub fn is_dm(&self) -> bool {
        matches!(self.kind, ChannelType::Dm | ChannelType::GroupDm)
    }
}

/// Type of channel.
#[derive(Clone, Copy, Debug, Default, Deserialize_repr, Serialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum ChannelType {
    /// A text channel within a server.
    #[default]
    GuildText = 0,
    /// A direct message between users.
    Dm = 1,
    /// A voice channel within a server.
    GuildVoice = 2,
    /// A direct message between multiple users.
    GroupDm = 3,
    /// An organizational category that contains up to 50 channels.
    GuildCategory = 4,
    /// A channel that users can follow and crosspost into their own server.
    GuildAnnouncement = 5,
    /// A temporary sub-channel within a GUILD_ANNOUNCEMENT channel.
    AnnouncementThread = 10,
    /// A temporary sub-channel within a GUILD_TEXT or GUILD_FORUM channel.
    PublicThread = 11,
    /// A temporary sub-channel within a GUILD_TEXT channel.
    PrivateThread = 12,
    /// A voice channel for hosting events with an audience.
    GuildStageVoice = 13,
    /// The channel in a hub containing the listed servers.
    GuildDirectory = 14,
    /// Channel that can only contain threads.
    GuildForum = 15,
    /// Channel that can only contain threads, similar to GUILD_FORUM channels.
    GuildMedia = 16,
}

/// Video quality mode.
#[derive(Clone, Copy, Debug, Default, Deserialize_repr, Serialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum VideoQualityMode {
    /// Discord chooses the quality for optimal performance.
    #[default]
    Auto = 1,
    /// 720p.
    Full = 2,
}

/// Thread metadata.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThreadMetadata {
    /// Whether the thread is archived.
    pub archived: bool,
    /// The thread will stop showing in the channel list after auto_archive_duration minutes.
    pub auto_archive_duration: u32,
    /// Timestamp when the thread's archive status was last changed.
    pub archive_timestamp: String,
    /// Whether the thread is locked.
    pub locked: bool,
    /// Whether non-moderators can add other non-moderators to a thread.
    #[serde(default)]
    pub invitable: bool,
    /// Timestamp when the thread was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<String>,
}

/// Thread member.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThreadMember {
    /// ID of the thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ChannelId>,
    /// ID of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
    /// Time the user last joined the thread.
    pub join_timestamp: String,
    /// Any user-thread settings.
    pub flags: u32,
    /// Additional information about the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<crate::guild::Member>,
}

/// Forum tag.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ForumTag {
    /// The ID of the tag.
    pub id: String,
    /// The name of the tag (0-20 characters).
    pub name: String,
    /// Whether this tag can only be added to or removed from threads by a member with the MANAGE_THREADS permission.
    pub moderated: bool,
    /// The ID of a guild's custom emoji.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_id: Option<String>,
    /// The unicode character of the emoji.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_name: Option<String>,
}

/// Default reaction emoji.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DefaultReaction {
    /// The ID of a guild's custom emoji.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_id: Option<String>,
    /// The unicode character of the emoji.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_name: Option<String>,
}

/// Sort order type for forum channels.
#[derive(Clone, Copy, Debug, Default, Deserialize_repr, Serialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum SortOrderType {
    /// Sort forum posts by activity.
    #[default]
    LatestActivity = 0,
    /// Sort forum posts by creation time (from most recent to oldest).
    CreationDate = 1,
}

/// Forum layout type.
#[derive(Clone, Copy, Debug, Default, Deserialize_repr, Serialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum ForumLayoutType {
    /// No default has been set for forum channel.
    #[default]
    NotSet = 0,
    /// Display posts as a list.
    ListView = 1,
    /// Display posts as a collection of tiles.
    GalleryView = 2,
}
