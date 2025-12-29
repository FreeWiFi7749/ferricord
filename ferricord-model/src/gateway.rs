//! Gateway-related models

use crate::channel::Channel;
use crate::guild::{Emoji, Guild, Member, Role};
use crate::id::{ChannelId, GuildId, UserId};
use crate::message::Message;
use crate::user::{CurrentUser, User};
use crate::voice::VoiceState;
use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

bitflags! {
    /// Gateway intents for filtering events.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct Intents: u32 {
        /// Includes events: GUILD_CREATE, GUILD_UPDATE, GUILD_DELETE, GUILD_ROLE_CREATE,
        /// GUILD_ROLE_UPDATE, GUILD_ROLE_DELETE, CHANNEL_CREATE, CHANNEL_UPDATE,
        /// CHANNEL_DELETE, CHANNEL_PINS_UPDATE, THREAD_CREATE, THREAD_UPDATE,
        /// THREAD_DELETE, THREAD_LIST_SYNC, THREAD_MEMBER_UPDATE, THREAD_MEMBERS_UPDATE,
        /// STAGE_INSTANCE_CREATE, STAGE_INSTANCE_UPDATE, STAGE_INSTANCE_DELETE
        const GUILDS = 1 << 0;
        /// Includes events: GUILD_MEMBER_ADD, GUILD_MEMBER_UPDATE, GUILD_MEMBER_REMOVE,
        /// THREAD_MEMBERS_UPDATE
        /// **Privileged Intent**
        const GUILD_MEMBERS = 1 << 1;
        /// Includes events: GUILD_AUDIT_LOG_ENTRY_CREATE, GUILD_BAN_ADD, GUILD_BAN_REMOVE
        const GUILD_MODERATION = 1 << 2;
        /// Includes events: GUILD_EMOJIS_UPDATE, GUILD_STICKERS_UPDATE
        const GUILD_EMOJIS_AND_STICKERS = 1 << 3;
        /// Includes events: GUILD_INTEGRATIONS_UPDATE, INTEGRATION_CREATE,
        /// INTEGRATION_UPDATE, INTEGRATION_DELETE
        const GUILD_INTEGRATIONS = 1 << 4;
        /// Includes events: WEBHOOKS_UPDATE
        const GUILD_WEBHOOKS = 1 << 5;
        /// Includes events: INVITE_CREATE, INVITE_DELETE
        const GUILD_INVITES = 1 << 6;
        /// Includes events: VOICE_STATE_UPDATE
        const GUILD_VOICE_STATES = 1 << 7;
        /// Includes events: PRESENCE_UPDATE
        /// **Privileged Intent**
        const GUILD_PRESENCES = 1 << 8;
        /// Includes events: MESSAGE_CREATE, MESSAGE_UPDATE, MESSAGE_DELETE,
        /// MESSAGE_DELETE_BULK
        const GUILD_MESSAGES = 1 << 9;
        /// Includes events: MESSAGE_REACTION_ADD, MESSAGE_REACTION_REMOVE,
        /// MESSAGE_REACTION_REMOVE_ALL, MESSAGE_REACTION_REMOVE_EMOJI
        const GUILD_MESSAGE_REACTIONS = 1 << 10;
        /// Includes events: TYPING_START
        const GUILD_MESSAGE_TYPING = 1 << 11;
        /// Includes events: MESSAGE_CREATE, MESSAGE_UPDATE, MESSAGE_DELETE,
        /// CHANNEL_PINS_UPDATE
        const DIRECT_MESSAGES = 1 << 12;
        /// Includes events: MESSAGE_REACTION_ADD, MESSAGE_REACTION_REMOVE,
        /// MESSAGE_REACTION_REMOVE_ALL, MESSAGE_REACTION_REMOVE_EMOJI
        const DIRECT_MESSAGE_REACTIONS = 1 << 13;
        /// Includes events: TYPING_START
        const DIRECT_MESSAGE_TYPING = 1 << 14;
        /// Enables message content in MESSAGE_CREATE and MESSAGE_UPDATE events
        /// **Privileged Intent**
        const MESSAGE_CONTENT = 1 << 15;
        /// Includes events: GUILD_SCHEDULED_EVENT_CREATE, GUILD_SCHEDULED_EVENT_UPDATE,
        /// GUILD_SCHEDULED_EVENT_DELETE, GUILD_SCHEDULED_EVENT_USER_ADD,
        /// GUILD_SCHEDULED_EVENT_USER_REMOVE
        const GUILD_SCHEDULED_EVENTS = 1 << 16;
        /// Includes events: AUTO_MODERATION_RULE_CREATE, AUTO_MODERATION_RULE_UPDATE,
        /// AUTO_MODERATION_RULE_DELETE
        const AUTO_MODERATION_CONFIGURATION = 1 << 20;
        /// Includes events: AUTO_MODERATION_ACTION_EXECUTION
        const AUTO_MODERATION_EXECUTION = 1 << 21;
        /// Includes events: MESSAGE_POLL_VOTE_ADD, MESSAGE_POLL_VOTE_REMOVE
        const GUILD_MESSAGE_POLLS = 1 << 24;
        /// Includes events: MESSAGE_POLL_VOTE_ADD, MESSAGE_POLL_VOTE_REMOVE
        const DIRECT_MESSAGE_POLLS = 1 << 25;
    }
}

impl Intents {
    /// Returns the default set of intents (non-privileged).
    pub fn default_intents() -> Self {
        Self::GUILDS
            | Self::GUILD_MODERATION
            | Self::GUILD_EMOJIS_AND_STICKERS
            | Self::GUILD_INTEGRATIONS
            | Self::GUILD_WEBHOOKS
            | Self::GUILD_INVITES
            | Self::GUILD_VOICE_STATES
            | Self::GUILD_MESSAGES
            | Self::GUILD_MESSAGE_REACTIONS
            | Self::GUILD_MESSAGE_TYPING
            | Self::DIRECT_MESSAGES
            | Self::DIRECT_MESSAGE_REACTIONS
            | Self::DIRECT_MESSAGE_TYPING
            | Self::GUILD_SCHEDULED_EVENTS
            | Self::AUTO_MODERATION_CONFIGURATION
            | Self::AUTO_MODERATION_EXECUTION
    }

    /// Returns all intents including privileged ones.
    pub fn all_intents() -> Self {
        Self::all()
    }

    /// Returns no intents.
    pub fn none_intents() -> Self {
        Self::empty()
    }

    /// Returns the privileged intents.
    pub fn privileged() -> Self {
        Self::GUILD_MEMBERS | Self::GUILD_PRESENCES | Self::MESSAGE_CONTENT
    }

    /// Check if any privileged intents are enabled.
    pub fn has_privileged(&self) -> bool {
        self.intersects(Self::privileged())
    }
}

impl Serialize for Intents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.bits())
    }
}

impl<'de> Deserialize<'de> for Intents {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bits = u32::deserialize(deserializer)?;
        Ok(Self::from_bits_truncate(bits))
    }
}

/// Gateway event types.
#[derive(Clone, Debug)]
pub enum GatewayEvent {
    /// Defines the heartbeat interval.
    Hello(HelloEvent),
    /// Contains the initial state information.
    Ready(ReadyEvent),
    /// Response to Resume, replayed missed events.
    Resumed,
    /// Server is going away, client should reconnect.
    Reconnect,
    /// Failure response to Identify or Resume.
    InvalidSession(bool),
    /// New guild channel created.
    ChannelCreate(Box<Channel>),
    /// Channel was updated.
    ChannelUpdate(Box<Channel>),
    /// Channel was deleted.
    ChannelDelete(Box<Channel>),
    /// Message was pinned or unpinned.
    ChannelPinsUpdate(ChannelPinsUpdateEvent),
    /// Thread created, also sent when being added to a private thread.
    ThreadCreate(Box<Channel>),
    /// Thread was updated.
    ThreadUpdate(Box<Channel>),
    /// Thread was deleted.
    ThreadDelete(ThreadDeleteEvent),
    /// Sent when gaining access to a channel, contains all active threads.
    ThreadListSync(ThreadListSyncEvent),
    /// Thread member for the current user was updated.
    ThreadMemberUpdate(ThreadMemberUpdateEvent),
    /// Some user(s) were added to or removed from a thread.
    ThreadMembersUpdate(ThreadMembersUpdateEvent),
    /// Lazy-load for unavailable guild, guild became available, or user joined a new guild.
    GuildCreate(Box<Guild>),
    /// Guild was updated.
    GuildUpdate(Box<Guild>),
    /// Guild became unavailable, or user left/was removed from a guild.
    GuildDelete(UnavailableGuild),
    /// A guild audit log entry was created.
    GuildAuditLogEntryCreate(serde_json::Value),
    /// User was banned from a guild.
    GuildBanAdd(GuildBanEvent),
    /// User was unbanned from a guild.
    GuildBanRemove(GuildBanEvent),
    /// Guild emojis were updated.
    GuildEmojisUpdate(GuildEmojisUpdateEvent),
    /// Guild stickers were updated.
    GuildStickersUpdate(GuildStickersUpdateEvent),
    /// Guild integration was updated.
    GuildIntegrationsUpdate(GuildIntegrationsUpdateEvent),
    /// New user joined a guild.
    GuildMemberAdd(GuildMemberAddEvent),
    /// User was removed from a guild.
    GuildMemberRemove(GuildMemberRemoveEvent),
    /// Guild member was updated.
    GuildMemberUpdate(GuildMemberUpdateEvent),
    /// Response to Request Guild Members.
    GuildMembersChunk(GuildMembersChunkEvent),
    /// Guild role was created.
    GuildRoleCreate(GuildRoleEvent),
    /// Guild role was updated.
    GuildRoleUpdate(GuildRoleEvent),
    /// Guild role was deleted.
    GuildRoleDelete(GuildRoleDeleteEvent),
    /// Guild scheduled event was created.
    GuildScheduledEventCreate(serde_json::Value),
    /// Guild scheduled event was updated.
    GuildScheduledEventUpdate(serde_json::Value),
    /// Guild scheduled event was deleted.
    GuildScheduledEventDelete(serde_json::Value),
    /// User subscribed to a guild scheduled event.
    GuildScheduledEventUserAdd(serde_json::Value),
    /// User unsubscribed from a guild scheduled event.
    GuildScheduledEventUserRemove(serde_json::Value),
    /// Guild integration was created.
    IntegrationCreate(serde_json::Value),
    /// Guild integration was updated.
    IntegrationUpdate(serde_json::Value),
    /// Guild integration was deleted.
    IntegrationDelete(serde_json::Value),
    /// Invite to a channel was created.
    InviteCreate(Box<InviteCreateEvent>),
    /// Invite to a channel was deleted.
    InviteDelete(InviteDeleteEvent),
    /// Message was created.
    MessageCreate(Box<Message>),
    /// Message was edited.
    MessageUpdate(MessageUpdateEvent),
    /// Message was deleted.
    MessageDelete(MessageDeleteEvent),
    /// Multiple messages were deleted at once.
    MessageDeleteBulk(MessageDeleteBulkEvent),
    /// User reacted to a message.
    MessageReactionAdd(Box<MessageReactionAddEvent>),
    /// User removed a reaction from a message.
    MessageReactionRemove(MessageReactionRemoveEvent),
    /// All reactions were explicitly removed from a message.
    MessageReactionRemoveAll(MessageReactionRemoveAllEvent),
    /// All reactions for a given emoji were explicitly removed from a message.
    MessageReactionRemoveEmoji(MessageReactionRemoveEmojiEvent),
    /// User's presence or info was updated.
    PresenceUpdate(PresenceUpdateEvent),
    /// Stage instance was created.
    StageInstanceCreate(serde_json::Value),
    /// Stage instance was updated.
    StageInstanceUpdate(serde_json::Value),
    /// Stage instance was deleted.
    StageInstanceDelete(serde_json::Value),
    /// User started typing in a channel.
    TypingStart(TypingStartEvent),
    /// Properties about the user changed.
    UserUpdate(CurrentUser),
    /// Someone joined, left, or moved a voice channel.
    VoiceStateUpdate(VoiceState),
    /// Guild's voice server was updated.
    VoiceServerUpdate(VoiceServerUpdateEvent),
    /// Guild channel webhook was created, updated, or deleted.
    WebhooksUpdate(WebhooksUpdateEvent),
    /// User used an interaction.
    InteractionCreate(serde_json::Value),
    /// Auto moderation rule was created.
    AutoModerationRuleCreate(serde_json::Value),
    /// Auto moderation rule was updated.
    AutoModerationRuleUpdate(serde_json::Value),
    /// Auto moderation rule was deleted.
    AutoModerationRuleDelete(serde_json::Value),
    /// Auto moderation rule was triggered.
    AutoModerationActionExecution(serde_json::Value),
    /// Unknown event type.
    Unknown(String, serde_json::Value),
}

/// Hello event data.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HelloEvent {
    /// Interval (in milliseconds) an app should heartbeat with.
    pub heartbeat_interval: u64,
}

/// Ready event data.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReadyEvent {
    /// API version.
    #[serde(rename = "v")]
    pub version: u8,
    /// Information about the user including email.
    pub user: CurrentUser,
    /// Guilds the user is in.
    pub guilds: Vec<UnavailableGuild>,
    /// Used for resuming connections.
    pub session_id: String,
    /// Gateway URL for resuming connections.
    pub resume_gateway_url: String,
    /// Shard information associated with this session.
    #[serde(default)]
    pub shard: Option<[u32; 2]>,
    /// Contains id and flags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<PartialApplication>,
}

/// Partial application data.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PartialApplication {
    /// Application ID.
    pub id: UserId,
    /// Application flags.
    pub flags: u32,
}

/// Unavailable guild.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UnavailableGuild {
    /// Guild ID.
    pub id: GuildId,
    /// Whether the guild is unavailable.
    #[serde(default)]
    pub unavailable: bool,
}

/// Channel pins update event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChannelPinsUpdateEvent {
    /// Guild ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    /// Channel ID.
    pub channel_id: ChannelId,
    /// Time at which the most recent pinned message was pinned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<String>,
}

/// Thread delete event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThreadDeleteEvent {
    /// Thread ID.
    pub id: ChannelId,
    /// Guild ID.
    pub guild_id: GuildId,
    /// Parent channel ID.
    pub parent_id: ChannelId,
    /// Thread type.
    #[serde(rename = "type")]
    pub kind: u8,
}

/// Thread list sync event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThreadListSyncEvent {
    /// Guild ID.
    pub guild_id: GuildId,
    /// Parent channel IDs whose threads are being synced.
    #[serde(default)]
    pub channel_ids: Vec<ChannelId>,
    /// All active threads in the given channels.
    pub threads: Vec<Channel>,
    /// All thread member objects from the synced threads.
    pub members: Vec<crate::channel::ThreadMember>,
}

/// Thread member update event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThreadMemberUpdateEvent {
    /// Thread ID.
    pub id: ChannelId,
    /// Guild ID.
    pub guild_id: GuildId,
    /// ID of the user.
    pub user_id: UserId,
    /// Time the user last joined the thread.
    pub join_timestamp: String,
    /// Any user-thread settings.
    pub flags: u32,
}

/// Thread members update event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThreadMembersUpdateEvent {
    /// Thread ID.
    pub id: ChannelId,
    /// Guild ID.
    pub guild_id: GuildId,
    /// Approximate number of members in the thread.
    pub member_count: u32,
    /// Users who were added to the thread.
    #[serde(default)]
    pub added_members: Vec<crate::channel::ThreadMember>,
    /// ID of the users who were removed from the thread.
    #[serde(default)]
    pub removed_member_ids: Vec<UserId>,
}

/// Guild ban event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildBanEvent {
    /// Guild ID.
    pub guild_id: GuildId,
    /// The banned user.
    pub user: User,
}

/// Guild emojis update event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildEmojisUpdateEvent {
    /// Guild ID.
    pub guild_id: GuildId,
    /// Array of emojis.
    pub emojis: Vec<Emoji>,
}

/// Guild stickers update event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildStickersUpdateEvent {
    /// Guild ID.
    pub guild_id: GuildId,
    /// Array of stickers.
    pub stickers: Vec<serde_json::Value>,
}

/// Guild integrations update event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildIntegrationsUpdateEvent {
    /// Guild ID.
    pub guild_id: GuildId,
}

/// Guild member add event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildMemberAddEvent {
    /// Guild ID.
    pub guild_id: GuildId,
    /// The member that joined.
    #[serde(flatten)]
    pub member: Member,
}

/// Guild member remove event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildMemberRemoveEvent {
    /// Guild ID.
    pub guild_id: GuildId,
    /// The user who was removed.
    pub user: User,
}

/// Guild member update event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildMemberUpdateEvent {
    /// Guild ID.
    pub guild_id: GuildId,
    /// User role IDs.
    pub roles: Vec<crate::id::RoleId>,
    /// The user.
    pub user: User,
    /// Nickname of the user in the guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,
    /// Member's guild avatar hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// When the user joined the guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_at: Option<String>,
    /// When the user started boosting the guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<String>,
    /// Whether the user is deafened in voice channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deaf: Option<bool>,
    /// Whether the user is muted in voice channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute: Option<bool>,
    /// Whether the user has not yet passed the guild's Membership Screening.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,
    /// When the user's timeout will expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communication_disabled_until: Option<String>,
}

/// Guild members chunk event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildMembersChunkEvent {
    /// Guild ID.
    pub guild_id: GuildId,
    /// Set of guild members.
    pub members: Vec<Member>,
    /// Chunk index in the expected chunks for this response.
    pub chunk_index: u32,
    /// Total number of expected chunks for this response.
    pub chunk_count: u32,
    /// If passing an invalid ID to REQUEST_GUILD_MEMBERS, it will be returned here.
    #[serde(default)]
    pub not_found: Vec<UserId>,
    /// If passing true to REQUEST_GUILD_MEMBERS, presences of the returned members will be here.
    #[serde(default)]
    pub presences: Vec<serde_json::Value>,
    /// Nonce used in the Guild Members Request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

/// Guild role event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildRoleEvent {
    /// Guild ID.
    pub guild_id: GuildId,
    /// The role.
    pub role: Role,
}

/// Guild role delete event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildRoleDeleteEvent {
    /// Guild ID.
    pub guild_id: GuildId,
    /// Role ID.
    pub role_id: crate::id::RoleId,
}

/// Invite create event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InviteCreateEvent {
    /// Channel the invite is for.
    pub channel_id: ChannelId,
    /// Unique invite code.
    pub code: String,
    /// Time at which the invite was created.
    pub created_at: String,
    /// Guild of the invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    /// User that created the invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inviter: Option<User>,
    /// How long the invite is valid for (in seconds).
    pub max_age: u32,
    /// Maximum number of times the invite can be used.
    pub max_uses: u32,
    /// Type of target for this voice channel invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<u8>,
    /// User whose stream to display for this voice channel stream invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user: Option<User>,
    /// Embedded application to open for this voice channel embedded application invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_application: Option<serde_json::Value>,
    /// Whether or not the invite is temporary.
    pub temporary: bool,
    /// How many times the invite has been used.
    pub uses: u32,
}

/// Invite delete event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InviteDeleteEvent {
    /// Channel of the invite.
    pub channel_id: ChannelId,
    /// Guild of the invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    /// Unique invite code.
    pub code: String,
}

/// Message update event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageUpdateEvent {
    /// Message ID.
    pub id: crate::id::MessageId,
    /// Channel ID.
    pub channel_id: ChannelId,
    /// Guild ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    /// Other fields are optional and may be present.
    #[serde(flatten)]
    pub fields: serde_json::Value,
}

/// Message delete event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageDeleteEvent {
    /// Message ID.
    pub id: crate::id::MessageId,
    /// Channel ID.
    pub channel_id: ChannelId,
    /// Guild ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
}

/// Message delete bulk event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageDeleteBulkEvent {
    /// IDs of the messages.
    pub ids: Vec<crate::id::MessageId>,
    /// Channel ID.
    pub channel_id: ChannelId,
    /// Guild ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
}

/// Message reaction add event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageReactionAddEvent {
    /// User ID.
    pub user_id: UserId,
    /// Channel ID.
    pub channel_id: ChannelId,
    /// Message ID.
    pub message_id: crate::id::MessageId,
    /// Guild ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    /// Member who reacted if this happened in a guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,
    /// Emoji used to react.
    pub emoji: Emoji,
    /// ID of the user who authored the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_author_id: Option<UserId>,
    /// True if this is a super-reaction.
    #[serde(default)]
    pub burst: bool,
    /// Colors used for super-reaction animation.
    #[serde(default)]
    pub burst_colors: Vec<String>,
    /// Type of reaction.
    #[serde(rename = "type")]
    pub kind: u8,
}

/// Message reaction remove event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageReactionRemoveEvent {
    /// User ID.
    pub user_id: UserId,
    /// Channel ID.
    pub channel_id: ChannelId,
    /// Message ID.
    pub message_id: crate::id::MessageId,
    /// Guild ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    /// Emoji used to react.
    pub emoji: Emoji,
    /// True if this was a super-reaction.
    #[serde(default)]
    pub burst: bool,
    /// Type of reaction.
    #[serde(rename = "type")]
    pub kind: u8,
}

/// Message reaction remove all event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageReactionRemoveAllEvent {
    /// Channel ID.
    pub channel_id: ChannelId,
    /// Message ID.
    pub message_id: crate::id::MessageId,
    /// Guild ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
}

/// Message reaction remove emoji event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageReactionRemoveEmojiEvent {
    /// Channel ID.
    pub channel_id: ChannelId,
    /// Message ID.
    pub message_id: crate::id::MessageId,
    /// Guild ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    /// Emoji that was removed.
    pub emoji: Emoji,
}

/// Presence update event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PresenceUpdateEvent {
    /// User whose presence is being updated.
    pub user: PartialUser,
    /// Guild ID.
    pub guild_id: GuildId,
    /// Either "idle", "dnd", "online", or "offline".
    pub status: String,
    /// User's current activities.
    pub activities: Vec<serde_json::Value>,
    /// User's platform-dependent status.
    pub client_status: ClientStatus,
}

/// Partial user for presence updates.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PartialUser {
    /// User ID.
    pub id: UserId,
    /// Other fields may be present.
    #[serde(flatten)]
    pub fields: serde_json::Value,
}

/// Client status.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientStatus {
    /// User's status set for an active desktop application session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desktop: Option<String>,
    /// User's status set for an active mobile application session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// User's status set for an active web application session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<String>,
}

/// Typing start event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TypingStartEvent {
    /// Channel ID.
    pub channel_id: ChannelId,
    /// Guild ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    /// User ID.
    pub user_id: UserId,
    /// Unix time (in seconds) of when the user started typing.
    pub timestamp: u64,
    /// Member who started typing if this happened in a guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,
}

/// Voice server update event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VoiceServerUpdateEvent {
    /// Voice connection token.
    pub token: String,
    /// Guild this voice server update is for.
    pub guild_id: GuildId,
    /// Voice server host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}

/// Webhooks update event.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebhooksUpdateEvent {
    /// Guild ID.
    pub guild_id: GuildId,
    /// Channel ID.
    pub channel_id: ChannelId,
}

/// Gateway payload structure.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GatewayPayload {
    /// Opcode for the payload.
    pub op: u8,
    /// Event data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<serde_json::Value>,
    /// Sequence number, used for resuming sessions and heartbeats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<u64>,
    /// Event name for this payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t: Option<String>,
}

/// Gateway opcodes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GatewayOpcode {
    /// An event was dispatched.
    Dispatch = 0,
    /// Fired periodically by the client to keep the connection alive.
    Heartbeat = 1,
    /// Starts a new session during the initial handshake.
    Identify = 2,
    /// Update the client's presence.
    PresenceUpdate = 3,
    /// Used to join/leave or move between voice channels.
    VoiceStateUpdate = 4,
    /// Resume a previous session that was disconnected.
    Resume = 6,
    /// You should attempt to reconnect and resume immediately.
    Reconnect = 7,
    /// Request information about offline guild members in a large guild.
    RequestGuildMembers = 8,
    /// The session has been invalidated.
    InvalidSession = 9,
    /// Sent immediately after connecting.
    Hello = 10,
    /// Sent in response to receiving a heartbeat.
    HeartbeatAck = 11,
}

impl TryFrom<u8> for GatewayOpcode {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Dispatch),
            1 => Ok(Self::Heartbeat),
            2 => Ok(Self::Identify),
            3 => Ok(Self::PresenceUpdate),
            4 => Ok(Self::VoiceStateUpdate),
            6 => Ok(Self::Resume),
            7 => Ok(Self::Reconnect),
            8 => Ok(Self::RequestGuildMembers),
            9 => Ok(Self::InvalidSession),
            10 => Ok(Self::Hello),
            11 => Ok(Self::HeartbeatAck),
            _ => Err(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intents_default() {
        let intents = Intents::default_intents();
        assert!(intents.contains(Intents::GUILDS));
        assert!(intents.contains(Intents::GUILD_MESSAGES));
        assert!(!intents.contains(Intents::GUILD_MEMBERS));
        assert!(!intents.contains(Intents::MESSAGE_CONTENT));
    }

    #[test]
    fn test_intents_privileged() {
        let intents = Intents::privileged();
        assert!(intents.contains(Intents::GUILD_MEMBERS));
        assert!(intents.contains(Intents::GUILD_PRESENCES));
        assert!(intents.contains(Intents::MESSAGE_CONTENT));
    }

    #[test]
    fn test_intents_serialization() {
        let intents = Intents::GUILDS | Intents::GUILD_MESSAGES;
        let json = serde_json::to_string(&intents).unwrap();
        assert_eq!(json, "513");
    }
}
