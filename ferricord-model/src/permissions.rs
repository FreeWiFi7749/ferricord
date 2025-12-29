//! Permission-related models

use crate::id::{ChannelId, RoleId, UserId};
use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

bitflags! {
    /// Discord permission flags.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct Permissions: u64 {
        /// Allows creation of instant invites.
        const CREATE_INSTANT_INVITE = 1 << 0;
        /// Allows kicking members.
        const KICK_MEMBERS = 1 << 1;
        /// Allows banning members.
        const BAN_MEMBERS = 1 << 2;
        /// Allows all permissions and bypasses channel permission overwrites.
        const ADMINISTRATOR = 1 << 3;
        /// Allows management and editing of channels.
        const MANAGE_CHANNELS = 1 << 4;
        /// Allows management and editing of the guild.
        const MANAGE_GUILD = 1 << 5;
        /// Allows for the addition of reactions to messages.
        const ADD_REACTIONS = 1 << 6;
        /// Allows for viewing of audit logs.
        const VIEW_AUDIT_LOG = 1 << 7;
        /// Allows for using priority speaker in a voice channel.
        const PRIORITY_SPEAKER = 1 << 8;
        /// Allows the user to go live.
        const STREAM = 1 << 9;
        /// Allows guild members to view a channel.
        const VIEW_CHANNEL = 1 << 10;
        /// Allows for sending messages in a channel.
        const SEND_MESSAGES = 1 << 11;
        /// Allows for sending of /tts messages.
        const SEND_TTS_MESSAGES = 1 << 12;
        /// Allows for deletion of other users messages.
        const MANAGE_MESSAGES = 1 << 13;
        /// Links sent by users with this permission will be auto-embedded.
        const EMBED_LINKS = 1 << 14;
        /// Allows for uploading images and files.
        const ATTACH_FILES = 1 << 15;
        /// Allows for reading of message history.
        const READ_MESSAGE_HISTORY = 1 << 16;
        /// Allows for using the @everyone tag to notify all users in a channel.
        const MENTION_EVERYONE = 1 << 17;
        /// Allows the usage of custom emojis from other servers.
        const USE_EXTERNAL_EMOJIS = 1 << 18;
        /// Allows for viewing guild insights.
        const VIEW_GUILD_INSIGHTS = 1 << 19;
        /// Allows for joining of a voice channel.
        const CONNECT = 1 << 20;
        /// Allows for speaking in a voice channel.
        const SPEAK = 1 << 21;
        /// Allows for muting members in a voice channel.
        const MUTE_MEMBERS = 1 << 22;
        /// Allows for deafening of members in a voice channel.
        const DEAFEN_MEMBERS = 1 << 23;
        /// Allows for moving of members between voice channels.
        const MOVE_MEMBERS = 1 << 24;
        /// Allows for using voice-activity-detection in a voice channel.
        const USE_VAD = 1 << 25;
        /// Allows for modification of own nickname.
        const CHANGE_NICKNAME = 1 << 26;
        /// Allows for modification of other users nicknames.
        const MANAGE_NICKNAMES = 1 << 27;
        /// Allows management and editing of roles.
        const MANAGE_ROLES = 1 << 28;
        /// Allows management and editing of webhooks.
        const MANAGE_WEBHOOKS = 1 << 29;
        /// Allows management and editing of emojis, stickers, and soundboard sounds.
        const MANAGE_GUILD_EXPRESSIONS = 1 << 30;
        /// Allows members to use application commands.
        const USE_APPLICATION_COMMANDS = 1 << 31;
        /// Allows for requesting to speak in stage channels.
        const REQUEST_TO_SPEAK = 1 << 32;
        /// Allows for creating, editing, and deleting scheduled events.
        const MANAGE_EVENTS = 1 << 33;
        /// Allows for deleting and archiving threads, and viewing all private threads.
        const MANAGE_THREADS = 1 << 34;
        /// Allows for creating public and announcement threads.
        const CREATE_PUBLIC_THREADS = 1 << 35;
        /// Allows for creating private threads.
        const CREATE_PRIVATE_THREADS = 1 << 36;
        /// Allows the usage of custom stickers from other servers.
        const USE_EXTERNAL_STICKERS = 1 << 37;
        /// Allows for sending messages in threads.
        const SEND_MESSAGES_IN_THREADS = 1 << 38;
        /// Allows for using Activities in a voice channel.
        const USE_EMBEDDED_ACTIVITIES = 1 << 39;
        /// Allows for timing out users to prevent them from sending or reacting to messages.
        const MODERATE_MEMBERS = 1 << 40;
        /// Allows for viewing role subscription insights.
        const VIEW_CREATOR_MONETIZATION_ANALYTICS = 1 << 41;
        /// Allows for using soundboard in a voice channel.
        const USE_SOUNDBOARD = 1 << 42;
        /// Allows for creating emojis, stickers, and soundboard sounds.
        const CREATE_GUILD_EXPRESSIONS = 1 << 43;
        /// Allows for creating scheduled events.
        const CREATE_EVENTS = 1 << 44;
        /// Allows the usage of custom soundboard sounds from other servers.
        const USE_EXTERNAL_SOUNDS = 1 << 45;
        /// Allows sending voice messages.
        const SEND_VOICE_MESSAGES = 1 << 46;
        /// Allows sending polls.
        const SEND_POLLS = 1 << 49;
        /// Allows user-installed apps to send public responses.
        const USE_EXTERNAL_APPS = 1 << 50;
    }
}

impl Permissions {
    /// Returns all permissions.
    pub fn all_permissions() -> Self {
        Self::all()
    }

    /// Returns no permissions.
    pub fn none_permissions() -> Self {
        Self::empty()
    }

    /// Returns the default permissions for @everyone role.
    pub fn default_permissions() -> Self {
        Self::VIEW_CHANNEL
            | Self::CREATE_INSTANT_INVITE
            | Self::CHANGE_NICKNAME
            | Self::SEND_MESSAGES
            | Self::SEND_MESSAGES_IN_THREADS
            | Self::EMBED_LINKS
            | Self::ATTACH_FILES
            | Self::ADD_REACTIONS
            | Self::USE_EXTERNAL_EMOJIS
            | Self::USE_EXTERNAL_STICKERS
            | Self::READ_MESSAGE_HISTORY
            | Self::CONNECT
            | Self::SPEAK
            | Self::USE_VAD
            | Self::USE_APPLICATION_COMMANDS
            | Self::USE_EMBEDDED_ACTIVITIES
            | Self::SEND_VOICE_MESSAGES
    }

    /// Check if the administrator permission is set.
    pub fn is_administrator(&self) -> bool {
        self.contains(Self::ADMINISTRATOR)
    }
}

impl Serialize for Permissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.bits().to_string())
    }
}

impl<'de> Deserialize<'de> for Permissions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let bits = s.parse::<u64>().map_err(serde::de::Error::custom)?;
        Ok(Self::from_bits_truncate(bits))
    }
}

/// Permission overwrite for a channel.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PermissionOverwrite {
    /// Role or user ID.
    pub id: String,
    /// Either 0 (role) or 1 (member).
    #[serde(rename = "type")]
    pub kind: u8,
    /// Permission bit set for allowed permissions.
    pub allow: Permissions,
    /// Permission bit set for denied permissions.
    pub deny: Permissions,
}

impl PermissionOverwrite {
    /// Create a new role permission overwrite.
    pub fn role(role_id: RoleId, allow: Permissions, deny: Permissions) -> Self {
        Self {
            id: role_id.to_string(),
            kind: 0,
            allow,
            deny,
        }
    }

    /// Create a new member permission overwrite.
    pub fn member(user_id: UserId, allow: Permissions, deny: Permissions) -> Self {
        Self {
            id: user_id.to_string(),
            kind: 1,
            allow,
            deny,
        }
    }

    /// Returns true if this is a role overwrite.
    pub fn is_role(&self) -> bool {
        self.kind == 0
    }

    /// Returns true if this is a member overwrite.
    pub fn is_member(&self) -> bool {
        self.kind == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permissions_serialization() {
        let perms = Permissions::SEND_MESSAGES | Permissions::VIEW_CHANNEL;
        let json = serde_json::to_string(&perms).unwrap();
        assert_eq!(json, "\"3072\"");
    }

    #[test]
    fn test_permissions_deserialization() {
        let json = "\"3072\"";
        let perms: Permissions = serde_json::from_str(json).unwrap();
        assert!(perms.contains(Permissions::SEND_MESSAGES));
        assert!(perms.contains(Permissions::VIEW_CHANNEL));
    }

    #[test]
    fn test_administrator_check() {
        let admin = Permissions::ADMINISTRATOR;
        assert!(admin.is_administrator());

        let non_admin = Permissions::SEND_MESSAGES;
        assert!(!non_admin.is_administrator());
    }
}
