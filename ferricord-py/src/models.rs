//! Python model wrappers
//!
//! This module provides Python-accessible wrappers for Discord models.

use ferricord_model::{Channel, Guild, Member, Message, Role, User};
use pyo3::prelude::*;

/// Python wrapper for User.
#[pyclass(name = "User")]
#[derive(Clone, Debug)]
pub struct PyUser {
    inner: User,
}

#[pymethods]
impl PyUser {
    /// The user's ID.
    #[getter]
    fn id(&self) -> u64 {
        self.inner.id.get()
    }

    /// The user's username.
    #[getter]
    fn username(&self) -> &str {
        &self.inner.username
    }

    /// The user's discriminator.
    #[getter]
    fn discriminator(&self) -> &str {
        &self.inner.discriminator
    }

    /// The user's display name.
    #[getter]
    fn display_name(&self) -> &str {
        self.inner.display_name()
    }

    /// The user's global name.
    #[getter]
    fn global_name(&self) -> Option<&str> {
        self.inner.global_name.as_deref()
    }

    /// The user's avatar hash.
    #[getter]
    fn avatar(&self) -> Option<&str> {
        self.inner.avatar.as_deref()
    }

    /// Whether the user is a bot.
    #[getter]
    fn bot(&self) -> bool {
        self.inner.bot
    }

    /// Whether the user is a system user.
    #[getter]
    fn system(&self) -> bool {
        self.inner.system
    }

    /// The user's tag (username#discriminator).
    fn tag(&self) -> String {
        self.inner.tag()
    }

    /// The user's avatar URL.
    fn avatar_url(&self) -> Option<String> {
        self.inner.avatar_url()
    }

    /// The user's default avatar URL.
    fn default_avatar_url(&self) -> String {
        self.inner.default_avatar_url()
    }

    /// The user's display avatar URL.
    fn display_avatar_url(&self) -> String {
        self.inner.display_avatar_url()
    }

    /// The user's mention string.
    fn mention(&self) -> String {
        self.inner.mention()
    }

    fn __repr__(&self) -> String {
        format!(
            "<User id={} username='{}'>",
            self.inner.id, self.inner.username
        )
    }

    fn __str__(&self) -> String {
        self.inner.tag()
    }
}

impl PyUser {
    pub fn new(user: User) -> Self {
        Self { inner: user }
    }

    pub fn inner(&self) -> &User {
        &self.inner
    }
}

/// Python wrapper for Message.
#[pyclass(name = "Message")]
#[derive(Clone, Debug)]
pub struct PyMessage {
    inner: Message,
}

#[pymethods]
impl PyMessage {
    /// The message's ID.
    #[getter]
    fn id(&self) -> u64 {
        self.inner.id.get()
    }

    /// The channel ID the message was sent in.
    #[getter]
    fn channel_id(&self) -> u64 {
        self.inner.channel_id.get()
    }

    /// The guild ID the message was sent in.
    #[getter]
    fn guild_id(&self) -> Option<u64> {
        self.inner.guild_id.map(|id| id.get())
    }

    /// The message content.
    #[getter]
    fn content(&self) -> &str {
        &self.inner.content
    }

    /// The message author.
    #[getter]
    fn author(&self) -> PyUser {
        PyUser::new(self.inner.author.clone())
    }

    /// When the message was sent.
    #[getter]
    fn timestamp(&self) -> &str {
        &self.inner.timestamp
    }

    /// When the message was edited.
    #[getter]
    fn edited_timestamp(&self) -> Option<&str> {
        self.inner.edited_timestamp.as_deref()
    }

    /// Whether this was a TTS message.
    #[getter]
    fn tts(&self) -> bool {
        self.inner.tts
    }

    /// Whether this message mentions everyone.
    #[getter]
    fn mention_everyone(&self) -> bool {
        self.inner.mention_everyone
    }

    /// Whether this message is pinned.
    #[getter]
    fn pinned(&self) -> bool {
        self.inner.pinned
    }

    /// The message type.
    #[getter]
    fn r#type(&self) -> u8 {
        self.inner.kind as u8
    }

    /// A URL that links to this message.
    fn jump_url(&self) -> String {
        self.inner.jump_url()
    }

    fn __repr__(&self) -> String {
        format!(
            "<Message id={} channel_id={} author='{}'>",
            self.inner.id, self.inner.channel_id, self.inner.author.username
        )
    }
}

impl PyMessage {
    pub fn new(message: Message) -> Self {
        Self { inner: message }
    }

    pub fn inner(&self) -> &Message {
        &self.inner
    }
}

/// Python wrapper for Guild.
#[pyclass(name = "Guild")]
#[derive(Clone, Debug)]
pub struct PyGuild {
    inner: Guild,
}

#[pymethods]
impl PyGuild {
    /// The guild's ID.
    #[getter]
    fn id(&self) -> u64 {
        self.inner.id.get()
    }

    /// The guild's name.
    #[getter]
    fn name(&self) -> &str {
        &self.inner.name
    }

    /// The guild's icon hash.
    #[getter]
    fn icon(&self) -> Option<&str> {
        self.inner.icon.as_deref()
    }

    /// The guild owner's ID.
    #[getter]
    fn owner_id(&self) -> u64 {
        self.inner.owner_id.get()
    }

    /// Whether the current user is the owner.
    #[getter]
    fn owner(&self) -> bool {
        self.inner.owner
    }

    /// The guild's description.
    #[getter]
    fn description(&self) -> Option<&str> {
        self.inner.description.as_deref()
    }

    /// The guild's vanity URL code.
    #[getter]
    fn vanity_url_code(&self) -> Option<&str> {
        self.inner.vanity_url_code.as_deref()
    }

    /// The guild's preferred locale.
    #[getter]
    fn preferred_locale(&self) -> &str {
        &self.inner.preferred_locale
    }

    /// The guild's premium tier (boost level).
    #[getter]
    fn premium_tier(&self) -> u8 {
        self.inner.premium_tier as u8
    }

    /// The number of boosts the guild has.
    #[getter]
    fn premium_subscription_count(&self) -> Option<u32> {
        self.inner.premium_subscription_count
    }

    /// The guild's icon URL.
    fn icon_url(&self) -> Option<String> {
        self.inner.icon_url()
    }

    /// The guild's banner URL.
    fn banner_url(&self) -> Option<String> {
        self.inner.banner_url()
    }

    fn __repr__(&self) -> String {
        format!("<Guild id={} name='{}'>", self.inner.id, self.inner.name)
    }

    fn __str__(&self) -> String {
        self.inner.name.clone()
    }
}

impl PyGuild {
    pub fn new(guild: Guild) -> Self {
        Self { inner: guild }
    }

    pub fn inner(&self) -> &Guild {
        &self.inner
    }
}

/// Python wrapper for Channel.
#[pyclass(name = "Channel")]
#[derive(Clone, Debug)]
pub struct PyChannel {
    inner: Channel,
}

#[pymethods]
impl PyChannel {
    /// The channel's ID.
    #[getter]
    fn id(&self) -> u64 {
        self.inner.id.get()
    }

    /// The channel's type.
    #[getter]
    fn r#type(&self) -> u8 {
        self.inner.kind as u8
    }

    /// The guild ID this channel belongs to.
    #[getter]
    fn guild_id(&self) -> Option<u64> {
        self.inner.guild_id.map(|id| id.get())
    }

    /// The channel's name.
    #[getter]
    fn name(&self) -> Option<&str> {
        self.inner.name.as_deref()
    }

    /// The channel's topic.
    #[getter]
    fn topic(&self) -> Option<&str> {
        self.inner.topic.as_deref()
    }

    /// Whether the channel is NSFW.
    #[getter]
    fn nsfw(&self) -> bool {
        self.inner.nsfw
    }

    /// The channel's position.
    #[getter]
    fn position(&self) -> Option<i32> {
        self.inner.position
    }

    /// The channel's mention string.
    fn mention(&self) -> String {
        self.inner.mention()
    }

    /// Whether this is a text-based channel.
    fn is_text_based(&self) -> bool {
        self.inner.is_text_based()
    }

    /// Whether this is a voice-based channel.
    fn is_voice_based(&self) -> bool {
        self.inner.is_voice_based()
    }

    /// Whether this is a thread.
    fn is_thread(&self) -> bool {
        self.inner.is_thread()
    }

    /// Whether this is a DM channel.
    fn is_dm(&self) -> bool {
        self.inner.is_dm()
    }

    fn __repr__(&self) -> String {
        format!(
            "<Channel id={} name='{}' type={}>",
            self.inner.id,
            self.inner.name.as_deref().unwrap_or("Unknown"),
            self.inner.kind as u8
        )
    }

    fn __str__(&self) -> String {
        self.inner
            .name
            .clone()
            .unwrap_or_else(|| format!("Channel {}", self.inner.id))
    }
}

impl PyChannel {
    pub fn new(channel: Channel) -> Self {
        Self { inner: channel }
    }

    pub fn inner(&self) -> &Channel {
        &self.inner
    }
}

/// Python wrapper for Member.
#[pyclass(name = "Member")]
#[derive(Clone, Debug)]
pub struct PyMember {
    inner: Member,
}

#[pymethods]
impl PyMember {
    /// The member's user.
    #[getter]
    fn user(&self) -> Option<PyUser> {
        self.inner.user.clone().map(PyUser::new)
    }

    /// The member's nickname.
    #[getter]
    fn nick(&self) -> Option<&str> {
        self.inner.nick.as_deref()
    }

    /// The member's display name.
    #[getter]
    fn display_name(&self) -> &str {
        self.inner.display_name()
    }

    /// When the member joined the guild.
    #[getter]
    fn joined_at(&self) -> &str {
        &self.inner.joined_at
    }

    /// When the member started boosting.
    #[getter]
    fn premium_since(&self) -> Option<&str> {
        self.inner.premium_since.as_deref()
    }

    /// Whether the member is deafened.
    #[getter]
    fn deaf(&self) -> bool {
        self.inner.deaf
    }

    /// Whether the member is muted.
    #[getter]
    fn mute(&self) -> bool {
        self.inner.mute
    }

    /// Whether the member is pending verification.
    #[getter]
    fn pending(&self) -> Option<bool> {
        self.inner.pending
    }

    /// The member's mention string.
    fn mention(&self) -> Option<String> {
        self.inner.mention()
    }

    fn __repr__(&self) -> String {
        let name = self
            .inner
            .user
            .as_ref()
            .map(|u| u.username.as_str())
            .unwrap_or("Unknown");
        format!("<Member name='{}'>", name)
    }
}

impl PyMember {
    pub fn new(member: Member) -> Self {
        Self { inner: member }
    }

    pub fn inner(&self) -> &Member {
        &self.inner
    }
}

/// Python wrapper for Role.
#[pyclass(name = "Role")]
#[derive(Clone, Debug)]
pub struct PyRole {
    inner: Role,
}

#[pymethods]
impl PyRole {
    /// The role's ID.
    #[getter]
    fn id(&self) -> u64 {
        self.inner.id.get()
    }

    /// The role's name.
    #[getter]
    fn name(&self) -> &str {
        &self.inner.name
    }

    /// The role's color.
    #[getter]
    fn color(&self) -> u32 {
        self.inner.color
    }

    /// Whether the role is hoisted.
    #[getter]
    fn hoist(&self) -> bool {
        self.inner.hoist
    }

    /// The role's position.
    #[getter]
    fn position(&self) -> i32 {
        self.inner.position
    }

    /// Whether the role is managed.
    #[getter]
    fn managed(&self) -> bool {
        self.inner.managed
    }

    /// Whether the role is mentionable.
    #[getter]
    fn mentionable(&self) -> bool {
        self.inner.mentionable
    }

    /// The role's mention string.
    fn mention(&self) -> String {
        self.inner.mention()
    }

    /// The role's color as a hex string.
    fn color_hex(&self) -> String {
        self.inner.color_hex()
    }

    fn __repr__(&self) -> String {
        format!("<Role id={} name='{}'>", self.inner.id, self.inner.name)
    }

    fn __str__(&self) -> String {
        self.inner.name.clone()
    }
}

impl PyRole {
    pub fn new(role: Role) -> Self {
        Self { inner: role }
    }

    pub fn inner(&self) -> &Role {
        &self.inner
    }
}
