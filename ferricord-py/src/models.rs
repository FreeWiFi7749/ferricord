//! Python model wrappers
//!
//! This module provides Python-accessible wrappers for Discord models.

use ferricord_model::interaction::{
    ApplicationCommandInteractionDataOption, Interaction, InteractionData, InteractionType,
};
use ferricord_model::{Channel, Guild, Member, Message, Role, User};
use pyo3::prelude::*;
use pyo3::IntoPyObject;

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

/// Python wrapper for Interaction.
#[pyclass(name = "Interaction")]
#[derive(Clone, Debug)]
pub struct PyInteraction {
    inner: Interaction,
}

#[pymethods]
impl PyInteraction {
    /// The interaction's ID.
    #[getter]
    fn id(&self) -> u64 {
        self.inner.id.get()
    }

    /// The application ID this interaction is for.
    #[getter]
    fn application_id(&self) -> u64 {
        self.inner.application_id.get()
    }

    /// The type of interaction.
    #[getter]
    fn r#type(&self) -> u8 {
        self.inner.kind as u8
    }

    /// The guild ID where the interaction was sent.
    #[getter]
    fn guild_id(&self) -> Option<u64> {
        self.inner.guild_id.map(|id| id.get())
    }

    /// The channel ID where the interaction was sent.
    #[getter]
    fn channel_id(&self) -> Option<u64> {
        self.inner.channel_id.map(|id| id.get())
    }

    /// The user who invoked the interaction.
    #[getter]
    fn user(&self) -> Option<PyUser> {
        self.inner.user().cloned().map(PyUser::new)
    }

    /// The member who invoked the interaction (if in a guild).
    #[getter]
    fn member(&self) -> Option<PyMember> {
        self.inner.member.clone().map(PyMember::new)
    }

    /// The interaction token for responding.
    #[getter]
    fn token(&self) -> &str {
        &self.inner.token
    }

    /// The interaction data (command name, options, etc.).
    #[getter]
    fn data(&self) -> Option<PyInteractionData> {
        self.inner.data.clone().map(PyInteractionData::new)
    }

    /// The locale of the invoking user.
    #[getter]
    fn locale(&self) -> Option<&str> {
        self.inner.locale.as_deref()
    }

    /// The guild's preferred locale.
    #[getter]
    fn guild_locale(&self) -> Option<&str> {
        self.inner.guild_locale.as_deref()
    }

    /// Whether this interaction was invoked in a guild.
    fn is_guild(&self) -> bool {
        self.inner.is_guild()
    }

    /// Whether this interaction was invoked in a DM.
    fn is_dm(&self) -> bool {
        self.inner.is_dm()
    }

    /// Whether this is a slash command interaction.
    fn is_command(&self) -> bool {
        self.inner.kind == InteractionType::ApplicationCommand
    }

    /// Whether this is a component interaction (button, select menu).
    fn is_component(&self) -> bool {
        self.inner.kind == InteractionType::MessageComponent
    }

    /// Whether this is a modal submit interaction.
    fn is_modal_submit(&self) -> bool {
        self.inner.kind == InteractionType::ModalSubmit
    }

    /// Whether this is an autocomplete interaction.
    fn is_autocomplete(&self) -> bool {
        self.inner.kind == InteractionType::ApplicationCommandAutocomplete
    }

    fn __repr__(&self) -> String {
        format!(
            "<Interaction id={} type={}>",
            self.inner.id, self.inner.kind as u8
        )
    }
}

impl PyInteraction {
    pub fn new(interaction: Interaction) -> Self {
        Self { inner: interaction }
    }

    pub fn inner(&self) -> &Interaction {
        &self.inner
    }
}

/// Python wrapper for InteractionData.
#[pyclass(name = "InteractionData")]
#[derive(Clone, Debug)]
pub struct PyInteractionData {
    inner: InteractionData,
}

#[pymethods]
impl PyInteractionData {
    /// The command ID (for application commands).
    #[getter]
    fn id(&self) -> Option<u64> {
        self.inner.id.map(|id| id.get())
    }

    /// The command name.
    #[getter]
    fn name(&self) -> Option<&str> {
        self.inner.name.as_deref()
    }

    /// The custom ID (for components and modals).
    #[getter]
    fn custom_id(&self) -> Option<&str> {
        self.inner.custom_id.as_deref()
    }

    /// The component type (for component interactions).
    #[getter]
    fn component_type(&self) -> Option<u8> {
        self.inner.component_type
    }

    /// The values selected (for select menus).
    #[getter]
    fn values(&self) -> Vec<String> {
        self.inner.values.clone()
    }

    /// The command options.
    #[getter]
    fn options(&self) -> Vec<PyInteractionOption> {
        self.inner
            .options
            .iter()
            .map(|o| PyInteractionOption::new(o.clone()))
            .collect()
    }

    /// Get an option by name.
    fn get_option(&self, name: &str) -> Option<PyInteractionOption> {
        self.inner
            .options
            .iter()
            .find(|o| o.name == name)
            .cloned()
            .map(PyInteractionOption::new)
    }

    /// Get a string option value by name.
    fn get_string(&self, name: &str) -> Option<String> {
        self.inner
            .options
            .iter()
            .find(|o| o.name == name)
            .and_then(|o| {
                o.value
                    .as_ref()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
            })
    }

    /// Get an integer option value by name.
    fn get_integer(&self, name: &str) -> Option<i64> {
        self.inner
            .options
            .iter()
            .find(|o| o.name == name)
            .and_then(|o| o.value.as_ref().and_then(|v| v.as_i64()))
    }

    /// Get a number option value by name.
    fn get_number(&self, name: &str) -> Option<f64> {
        self.inner
            .options
            .iter()
            .find(|o| o.name == name)
            .and_then(|o| o.value.as_ref().and_then(|v| v.as_f64()))
    }

    /// Get a boolean option value by name.
    fn get_boolean(&self, name: &str) -> Option<bool> {
        self.inner
            .options
            .iter()
            .find(|o| o.name == name)
            .and_then(|o| o.value.as_ref().and_then(|v| v.as_bool()))
    }

    fn __repr__(&self) -> String {
        format!(
            "<InteractionData name='{}'>",
            self.inner.name.as_deref().unwrap_or("Unknown")
        )
    }
}

impl PyInteractionData {
    pub fn new(data: InteractionData) -> Self {
        Self { inner: data }
    }
}

/// Python wrapper for ApplicationCommandInteractionDataOption.
#[pyclass(name = "InteractionOption")]
#[derive(Clone, Debug)]
pub struct PyInteractionOption {
    inner: ApplicationCommandInteractionDataOption,
}

#[pymethods]
impl PyInteractionOption {
    /// The option name.
    #[getter]
    fn name(&self) -> &str {
        &self.inner.name
    }

    /// The option type.
    #[getter]
    fn r#type(&self) -> u8 {
        self.inner.kind as u8
    }

    /// The option value as a Python object.
    #[getter]
    fn value(&self) -> Option<PyObject> {
        Python::with_gil(|py| {
            self.inner.value.as_ref().and_then(|v| {
                if let Some(s) = v.as_str() {
                    s.into_pyobject(py).ok().map(|o| o.into_any().unbind())
                } else if let Some(i) = v.as_i64() {
                    i.into_pyobject(py).ok().map(|o| o.into_any().unbind())
                } else if let Some(f) = v.as_f64() {
                    f.into_pyobject(py).ok().map(|o| o.into_any().unbind())
                } else if let Some(b) = v.as_bool() {
                    // For bool, into_pyobject returns Result<Borrowed, Infallible>
                    // Use ok() to unwrap the infallible result, then convert
                    b.into_pyobject(py)
                        .ok()
                        .map(|o| o.as_any().clone().unbind())
                } else {
                    v.to_string()
                        .into_pyobject(py)
                        .ok()
                        .map(|o| o.into_any().unbind())
                }
            })
        })
    }

    /// Whether this option is focused (for autocomplete).
    #[getter]
    fn focused(&self) -> bool {
        self.inner.focused
    }

    /// Nested options (for subcommands).
    #[getter]
    fn options(&self) -> Vec<PyInteractionOption> {
        self.inner
            .options
            .iter()
            .map(|o| PyInteractionOption::new(o.clone()))
            .collect()
    }

    fn __repr__(&self) -> String {
        format!("<InteractionOption name='{}'>", self.inner.name)
    }
}

impl PyInteractionOption {
    pub fn new(option: ApplicationCommandInteractionDataOption) -> Self {
        Self { inner: option }
    }
}

/// Python wrapper for InteractionResponse builder.
#[pyclass(name = "InteractionResponse")]
#[derive(Clone, Debug, Default)]
pub struct PyInteractionResponse {
    pub content: Option<String>,
    pub ephemeral: bool,
}

#[pymethods]
impl PyInteractionResponse {
    #[new]
    #[pyo3(signature = (content=None, ephemeral=false))]
    fn new(content: Option<String>, ephemeral: bool) -> Self {
        Self { content, ephemeral }
    }

    /// Set the response content.
    fn set_content(&mut self, content: String) {
        self.content = Some(content);
    }

    /// Set whether the response is ephemeral.
    fn set_ephemeral(&mut self, ephemeral: bool) {
        self.ephemeral = ephemeral;
    }

    fn __repr__(&self) -> String {
        format!(
            "<InteractionResponse content='{}' ephemeral={}>",
            self.content.as_deref().unwrap_or(""),
            self.ephemeral
        )
    }
}
