//! Guild (server) related models

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::id::{ChannelId, EmojiId, GuildId, RoleId, UserId};
use crate::user::User;
use crate::permissions::Permissions;

/// Represents a Discord guild (server).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Guild {
    /// Guild ID.
    pub id: GuildId,
    /// Guild name (2-100 characters).
    pub name: String,
    /// Icon hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Icon hash, returned when in the template object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_hash: Option<String>,
    /// Splash hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splash: Option<String>,
    /// Discovery splash hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_splash: Option<String>,
    /// True if the user is the owner of the guild.
    #[serde(default)]
    pub owner: bool,
    /// ID of owner.
    pub owner_id: UserId,
    /// Total permissions for the user in the guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    /// ID of AFK channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk_channel_id: Option<ChannelId>,
    /// AFK timeout in seconds.
    pub afk_timeout: u32,
    /// True if the server widget is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_enabled: Option<bool>,
    /// The channel ID that the widget will generate an invite to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_channel_id: Option<ChannelId>,
    /// Verification level required for the guild.
    pub verification_level: VerificationLevel,
    /// Default message notifications level.
    pub default_message_notifications: DefaultMessageNotificationLevel,
    /// Explicit content filter level.
    pub explicit_content_filter: ExplicitContentFilterLevel,
    /// Roles in the guild.
    #[serde(default)]
    pub roles: Vec<Role>,
    /// Custom guild emojis.
    #[serde(default)]
    pub emojis: Vec<Emoji>,
    /// Enabled guild features.
    #[serde(default)]
    pub features: Vec<String>,
    /// Required MFA level for the guild.
    pub mfa_level: MfaLevel,
    /// Application ID of the guild creator if it is bot-created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<UserId>,
    /// The ID of the channel where guild notices are posted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_channel_id: Option<ChannelId>,
    /// System channel flags.
    #[serde(default)]
    pub system_channel_flags: u32,
    /// The ID of the channel where Community guilds can display rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules_channel_id: Option<ChannelId>,
    /// The maximum number of presences for the guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_presences: Option<u32>,
    /// The maximum number of members for the guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_members: Option<u32>,
    /// The vanity URL code for the guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vanity_url_code: Option<String>,
    /// The description of a guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Banner hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    /// Premium tier (Server Boost level).
    pub premium_tier: PremiumTier,
    /// The number of boosts this guild currently has.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_count: Option<u32>,
    /// The preferred locale of a Community guild.
    pub preferred_locale: String,
    /// The ID of the channel where admins and moderators receive notices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_updates_channel_id: Option<ChannelId>,
    /// The maximum amount of users in a video channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_video_channel_users: Option<u32>,
    /// The maximum amount of users in a stage video channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_stage_video_channel_users: Option<u32>,
    /// Approximate number of members in this guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_member_count: Option<u32>,
    /// Approximate number of non-offline members in this guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_presence_count: Option<u32>,
    /// The NSFW level of the guild.
    #[serde(default)]
    pub nsfw_level: NsfwLevel,
    /// Whether the guild has the boost progress bar enabled.
    #[serde(default)]
    pub premium_progress_bar_enabled: bool,
    /// The ID of the channel where admins and moderators receive safety alerts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_alerts_channel_id: Option<ChannelId>,
}

impl Guild {
    /// Returns the URL to the guild's icon.
    pub fn icon_url(&self) -> Option<String> {
        self.icon.as_ref().map(|hash| {
            let ext = if hash.starts_with("a_") { "gif" } else { "png" };
            format!(
                "https://cdn.discordapp.com/icons/{}/{}.{}",
                self.id, hash, ext
            )
        })
    }

    /// Returns the URL to the guild's banner.
    pub fn banner_url(&self) -> Option<String> {
        self.banner.as_ref().map(|hash| {
            let ext = if hash.starts_with("a_") { "gif" } else { "png" };
            format!(
                "https://cdn.discordapp.com/banners/{}/{}.{}",
                self.id, hash, ext
            )
        })
    }

    /// Returns the URL to the guild's splash.
    pub fn splash_url(&self) -> Option<String> {
        self.splash.as_ref().map(|hash| {
            format!(
                "https://cdn.discordapp.com/splashes/{}/{}.png",
                self.id, hash
            )
        })
    }
}

/// Partial guild object.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PartialGuild {
    /// Guild ID.
    pub id: GuildId,
    /// Guild name.
    pub name: String,
    /// Icon hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Whether the user is the owner.
    #[serde(default)]
    pub owner: bool,
    /// Permissions for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    /// Guild features.
    #[serde(default)]
    pub features: Vec<String>,
}

/// Represents a guild member.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Member {
    /// The user this guild member represents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// This user's guild nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,
    /// The member's guild avatar hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// Array of role object IDs.
    #[serde(default)]
    pub roles: Vec<RoleId>,
    /// When the user joined the guild.
    pub joined_at: String,
    /// When the user started boosting the guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<String>,
    /// Whether the user is deafened in voice channels.
    pub deaf: bool,
    /// Whether the user is muted in voice channels.
    pub mute: bool,
    /// Guild member flags.
    #[serde(default)]
    pub flags: u32,
    /// Whether the user has not yet passed the guild's Membership Screening.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,
    /// Total permissions of the member in the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    /// When the user's timeout will expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communication_disabled_until: Option<String>,
}

impl Member {
    /// Returns the member's display name (nick if set, otherwise username).
    pub fn display_name(&self) -> &str {
        self.nick.as_deref().unwrap_or_else(|| {
            self.user
                .as_ref()
                .map(|u| u.display_name())
                .unwrap_or("Unknown")
        })
    }

    /// Returns the member's mention string.
    pub fn mention(&self) -> Option<String> {
        self.user.as_ref().map(|u| u.mention())
    }
}

/// Represents a role within a guild.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Role {
    /// Role ID.
    pub id: RoleId,
    /// Role name.
    pub name: String,
    /// Integer representation of hexadecimal color code.
    pub color: u32,
    /// If this role is pinned in the user listing.
    pub hoist: bool,
    /// Role icon hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Role unicode emoji.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_emoji: Option<String>,
    /// Position of this role.
    pub position: i32,
    /// Permission bit set.
    pub permissions: Permissions,
    /// Whether this role is managed by an integration.
    pub managed: bool,
    /// Whether this role is mentionable.
    pub mentionable: bool,
    /// The tags this role has.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<RoleTags>,
    /// Role flags.
    #[serde(default)]
    pub flags: u32,
}

impl Role {
    /// Returns the role's mention string.
    pub fn mention(&self) -> String {
        format!("<@&{}>", self.id)
    }

    /// Returns the role's color as a hex string.
    pub fn color_hex(&self) -> String {
        format!("#{:06X}", self.color)
    }
}

/// Tags for a role.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RoleTags {
    /// The ID of the bot this role belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<UserId>,
    /// The ID of the integration this role belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<UserId>,
    /// Whether this is the guild's Booster role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscriber: Option<()>,
    /// The ID of this role's subscription sku and listing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_listing_id: Option<UserId>,
    /// Whether this role is available for purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_for_purchase: Option<()>,
    /// Whether this role is a guild's linked role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_connections: Option<()>,
}

/// Represents a custom emoji.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Emoji {
    /// Emoji ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<EmojiId>,
    /// Emoji name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Roles allowed to use this emoji.
    #[serde(default)]
    pub roles: Vec<RoleId>,
    /// User that created this emoji.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// Whether this emoji must be wrapped in colons.
    #[serde(default)]
    pub require_colons: bool,
    /// Whether this emoji is managed.
    #[serde(default)]
    pub managed: bool,
    /// Whether this emoji is animated.
    #[serde(default)]
    pub animated: bool,
    /// Whether this emoji can be used.
    #[serde(default = "default_true")]
    pub available: bool,
}

fn default_true() -> bool {
    true
}

impl Emoji {
    /// Returns the emoji's URL if it's a custom emoji.
    pub fn url(&self) -> Option<String> {
        self.id.map(|id| {
            let ext = if self.animated { "gif" } else { "png" };
            format!("https://cdn.discordapp.com/emojis/{}.{}", id, ext)
        })
    }

    /// Returns the emoji's string representation for use in messages.
    pub fn to_reaction_string(&self) -> String {
        match (self.id, &self.name) {
            (Some(id), Some(name)) => {
                if self.animated {
                    format!("<a:{}:{}>", name, id)
                } else {
                    format!("<:{}:{}>", name, id)
                }
            }
            (None, Some(name)) => name.clone(),
            _ => String::new(),
        }
    }
}

/// Verification level required for a guild.
#[derive(Clone, Copy, Debug, Default, Deserialize_repr, Serialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum VerificationLevel {
    /// Unrestricted.
    #[default]
    None = 0,
    /// Must have verified email on account.
    Low = 1,
    /// Must be registered on Discord for longer than 5 minutes.
    Medium = 2,
    /// Must be a member of the server for longer than 10 minutes.
    High = 3,
    /// Must have a verified phone number.
    VeryHigh = 4,
}

/// Default message notification level.
#[derive(Clone, Copy, Debug, Default, Deserialize_repr, Serialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum DefaultMessageNotificationLevel {
    /// Members will receive notifications for all messages.
    #[default]
    AllMessages = 0,
    /// Members will receive notifications only for messages that @mention them.
    OnlyMentions = 1,
}

/// Explicit content filter level.
#[derive(Clone, Copy, Debug, Default, Deserialize_repr, Serialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum ExplicitContentFilterLevel {
    /// Media content will not be scanned.
    #[default]
    Disabled = 0,
    /// Media content sent by members without roles will be scanned.
    MembersWithoutRoles = 1,
    /// Media content sent by all members will be scanned.
    AllMembers = 2,
}

/// MFA level required for the guild.
#[derive(Clone, Copy, Debug, Default, Deserialize_repr, Serialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum MfaLevel {
    /// Guild has no MFA/2FA requirement for moderation actions.
    #[default]
    None = 0,
    /// Guild has a 2FA requirement for moderation actions.
    Elevated = 1,
}

/// Premium tier (Server Boost level).
#[derive(Clone, Copy, Debug, Default, Deserialize_repr, Serialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum PremiumTier {
    /// Guild has not unlocked any Server Boost perks.
    #[default]
    None = 0,
    /// Guild has unlocked Server Boost level 1 perks.
    Tier1 = 1,
    /// Guild has unlocked Server Boost level 2 perks.
    Tier2 = 2,
    /// Guild has unlocked Server Boost level 3 perks.
    Tier3 = 3,
}

/// NSFW level of a guild.
#[derive(Clone, Copy, Debug, Default, Deserialize_repr, Serialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum NsfwLevel {
    /// Default.
    #[default]
    Default = 0,
    /// Explicit.
    Explicit = 1,
    /// Safe.
    Safe = 2,
    /// Age restricted.
    AgeRestricted = 3,
}
