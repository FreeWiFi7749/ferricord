//! Interaction-related models

use crate::channel::Channel;
use crate::guild::Member;
use crate::id::{ChannelId, CommandId, GuildId, InteractionId, UserId};
use crate::message::{Attachment, Component, Embed, Message};
use crate::permissions::Permissions;
use crate::user::User;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Represents an interaction from a user.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Interaction {
    /// ID of the interaction.
    pub id: InteractionId,
    /// ID of the application this interaction is for.
    pub application_id: UserId,
    /// Type of interaction.
    #[serde(rename = "type")]
    pub kind: InteractionType,
    /// Interaction data payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InteractionData>,
    /// Guild that the interaction was sent from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    /// Channel that the interaction was sent from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
    /// Channel that the interaction was sent from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,
    /// Guild member data for the invoking user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,
    /// User object for the invoking user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// Continuation token for responding to the interaction.
    pub token: String,
    /// Read-only property, always 1.
    pub version: u8,
    /// For components, the message they were attached to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    /// Bitwise set of permissions the app has in the source location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_permissions: Option<Permissions>,
    /// Selected language of the invoking user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// Guild's preferred locale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_locale: Option<String>,
    /// For monetized apps, any entitlements for the invoking user.
    #[serde(default)]
    pub entitlements: Vec<serde_json::Value>,
    /// Mapping of installation contexts that the interaction was authorized for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizing_integration_owners: Option<serde_json::Value>,
    /// Context where the interaction was triggered from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<InteractionContextType>,
}

impl Interaction {
    /// Returns the user who invoked this interaction.
    pub fn user(&self) -> Option<&User> {
        self.user
            .as_ref()
            .or_else(|| self.member.as_ref().and_then(|m| m.user.as_ref()))
    }

    /// Returns the user ID of who invoked this interaction.
    pub fn user_id(&self) -> Option<UserId> {
        self.user().map(|u| u.id)
    }

    /// Returns true if this interaction was invoked in a guild.
    pub fn is_guild(&self) -> bool {
        self.guild_id.is_some()
    }

    /// Returns true if this interaction was invoked in a DM.
    pub fn is_dm(&self) -> bool {
        self.guild_id.is_none()
    }
}

/// Type of interaction.
#[derive(Clone, Copy, Debug, Default, Deserialize_repr, Serialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum InteractionType {
    /// A ping.
    #[default]
    Ping = 1,
    /// A slash command.
    ApplicationCommand = 2,
    /// A message component (button, select menu).
    MessageComponent = 3,
    /// An autocomplete interaction.
    ApplicationCommandAutocomplete = 4,
    /// A modal submit.
    ModalSubmit = 5,
}

/// Interaction context type.
#[derive(Clone, Copy, Debug, Default, Deserialize_repr, Serialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum InteractionContextType {
    /// Interaction can be used within servers.
    #[default]
    Guild = 0,
    /// Interaction can be used within DMs with the app's bot user.
    BotDm = 1,
    /// Interaction can be used within Group DMs and DMs other than the app's bot user.
    PrivateChannel = 2,
}

/// Interaction data.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InteractionData {
    /// ID of the invoked command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<CommandId>,
    /// Name of the invoked command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Type of the invoked command.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<ApplicationCommandType>,
    /// Converted users + roles + channels + attachments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<ResolvedData>,
    /// Params + values from the user.
    #[serde(default)]
    pub options: Vec<ApplicationCommandInteractionDataOption>,
    /// ID of the guild the command is registered to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    /// ID of the user or message targeted by a user or message command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<UserId>,
    /// Custom ID of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
    /// Type of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<u8>,
    /// Values the user selected in a select menu.
    #[serde(default)]
    pub values: Vec<String>,
    /// Components from a modal submit.
    #[serde(default)]
    pub components: Vec<Component>,
}

/// Application command type.
#[derive(Clone, Copy, Debug, Default, Deserialize_repr, Serialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum ApplicationCommandType {
    /// Slash commands.
    #[default]
    ChatInput = 1,
    /// A UI-based command that shows up when you right click or tap on a user.
    User = 2,
    /// A UI-based command that shows up when you right click or tap on a message.
    Message = 3,
}

/// Resolved data from an interaction.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ResolvedData {
    /// IDs and User objects.
    #[serde(default)]
    pub users: std::collections::HashMap<String, User>,
    /// IDs and partial Member objects.
    #[serde(default)]
    pub members: std::collections::HashMap<String, Member>,
    /// IDs and Role objects.
    #[serde(default)]
    pub roles: std::collections::HashMap<String, crate::guild::Role>,
    /// IDs and partial Channel objects.
    #[serde(default)]
    pub channels: std::collections::HashMap<String, Channel>,
    /// IDs and partial Message objects.
    #[serde(default)]
    pub messages: std::collections::HashMap<String, Message>,
    /// IDs and attachment objects.
    #[serde(default)]
    pub attachments: std::collections::HashMap<String, Attachment>,
}

/// Application command interaction data option.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApplicationCommandInteractionDataOption {
    /// Name of the parameter.
    pub name: String,
    /// Value of application command option type.
    #[serde(rename = "type")]
    pub kind: ApplicationCommandOptionType,
    /// Value of the option resulting from user input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    /// Present if this option is a group or subcommand.
    #[serde(default)]
    pub options: Vec<ApplicationCommandInteractionDataOption>,
    /// True if this option is the currently focused option for autocomplete.
    #[serde(default)]
    pub focused: bool,
}

/// Application command option type.
#[derive(Clone, Copy, Debug, Default, Deserialize_repr, Serialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum ApplicationCommandOptionType {
    #[default]
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 9,
    Number = 10,
    Attachment = 11,
}

/// Application command structure.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApplicationCommand {
    /// Unique ID of command.
    pub id: CommandId,
    /// Type of command.
    #[serde(rename = "type", default)]
    pub kind: ApplicationCommandType,
    /// ID of the parent application.
    pub application_id: UserId,
    /// Guild ID of the command, if not global.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    /// Name of command, 1-32 characters.
    pub name: String,
    /// Localization dictionary for name field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<std::collections::HashMap<String, String>>,
    /// Description for CHAT_INPUT commands, 1-100 characters.
    pub description: String,
    /// Localization dictionary for description field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<std::collections::HashMap<String, String>>,
    /// Parameters for the command, max of 25.
    #[serde(default)]
    pub options: Vec<ApplicationCommandOption>,
    /// Set of permissions represented as a bit set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_member_permissions: Option<Permissions>,
    /// Indicates whether the command is available in DMs with the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_permission: Option<bool>,
    /// Whether the command is enabled by default when the app is added to a guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_permission: Option<bool>,
    /// Indicates whether the command is age-restricted.
    #[serde(default)]
    pub nsfw: bool,
    /// Installation contexts where the command is available.
    #[serde(default)]
    pub integration_types: Vec<u8>,
    /// Interaction context(s) where the command can be used.
    #[serde(default)]
    pub contexts: Vec<InteractionContextType>,
    /// Autoincrementing version identifier.
    pub version: String,
}

/// Application command option.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApplicationCommandOption {
    /// Type of option.
    #[serde(rename = "type")]
    pub kind: ApplicationCommandOptionType,
    /// 1-32 character name.
    pub name: String,
    /// Localization dictionary for the name field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<std::collections::HashMap<String, String>>,
    /// 1-100 character description.
    pub description: String,
    /// Localization dictionary for the description field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<std::collections::HashMap<String, String>>,
    /// If the parameter is required or optional.
    #[serde(default)]
    pub required: bool,
    /// Choices for STRING, INTEGER, and NUMBER types for the user to pick from.
    #[serde(default)]
    pub choices: Vec<ApplicationCommandOptionChoice>,
    /// If the option is a subcommand or subcommand group type, these nested options.
    #[serde(default)]
    pub options: Vec<ApplicationCommandOption>,
    /// If the option is a channel type, the channels shown will be restricted to these types.
    #[serde(default)]
    pub channel_types: Vec<crate::channel::ChannelType>,
    /// If the option is an INTEGER or NUMBER type, the minimum value permitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<serde_json::Number>,
    /// If the option is an INTEGER or NUMBER type, the maximum value permitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<serde_json::Number>,
    /// For option type STRING, the minimum allowed length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u32>,
    /// For option type STRING, the maximum allowed length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,
    /// If autocomplete interactions are enabled for this STRING, INTEGER, or NUMBER type option.
    #[serde(default)]
    pub autocomplete: bool,
}

/// Application command option choice.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApplicationCommandOptionChoice {
    /// 1-100 character choice name.
    pub name: String,
    /// Localization dictionary for the name field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<std::collections::HashMap<String, String>>,
    /// Value for the choice.
    pub value: serde_json::Value,
}

/// Interaction response type.
#[derive(Clone, Copy, Debug, Default, Deserialize_repr, Serialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum InteractionResponseType {
    /// ACK a Ping.
    #[default]
    Pong = 1,
    /// Respond to an interaction with a message.
    ChannelMessageWithSource = 4,
    /// ACK an interaction and edit a response later.
    DeferredChannelMessageWithSource = 5,
    /// For components, ACK an interaction and edit the original message later.
    DeferredUpdateMessage = 6,
    /// For components, edit the message the component was attached to.
    UpdateMessage = 7,
    /// Respond to an autocomplete interaction with suggested choices.
    ApplicationCommandAutocompleteResult = 8,
    /// Respond to an interaction with a popup modal.
    Modal = 9,
    /// Respond to an interaction with an upgrade button.
    PremiumRequired = 10,
}

/// Interaction response.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InteractionResponse {
    /// Type of response.
    #[serde(rename = "type")]
    pub kind: InteractionResponseType,
    /// An optional response message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InteractionCallbackData>,
}

/// Interaction callback data.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InteractionCallbackData {
    /// Is the response TTS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    /// Message content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Supports up to 10 embeds.
    #[serde(default)]
    pub embeds: Vec<Embed>,
    /// Allowed mentions object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,
    /// Message flags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u32>,
    /// Message components.
    #[serde(default)]
    pub components: Vec<Component>,
    /// Attachment objects with filename and description.
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    /// Autocomplete choices (max of 25 choices).
    #[serde(default)]
    pub choices: Vec<ApplicationCommandOptionChoice>,
    /// Custom ID for modal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
    /// Title for modal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Allowed mentions structure.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AllowedMentions {
    /// An array of allowed mention types to parse from the content.
    #[serde(default)]
    pub parse: Vec<String>,
    /// Array of role_ids to mention.
    #[serde(default)]
    pub roles: Vec<String>,
    /// Array of user_ids to mention.
    #[serde(default)]
    pub users: Vec<String>,
    /// For replies, whether to mention the author of the message being replied to.
    #[serde(default)]
    pub replied_user: bool,
}
