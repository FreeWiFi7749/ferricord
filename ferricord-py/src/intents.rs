//! Gateway Intents for Python

use ferricord_model::gateway::Intents as RustIntents;
use pyo3::prelude::*;

/// Gateway intents for filtering events.
///
/// Intents are used to specify which events your bot wants to receive
/// from the Discord Gateway. Some intents are privileged and require
/// verification for bots in 100+ guilds.
///
/// Example:
///     ```python
///     from ferricord import Intents
///
///     # Use default intents (non-privileged)
///     intents = Intents.default()
///
///     # Enable message content (privileged)
///     intents.message_content = True
///
///     # Create client with intents
///     client = Client(intents=intents)
///     ```
#[pyclass(name = "Intents")]
#[derive(Clone, Debug)]
pub struct Intents {
    inner: RustIntents,
}

#[pymethods]
impl Intents {
    /// Create a new Intents with no intents enabled.
    #[new]
    fn new() -> Self {
        Self {
            inner: RustIntents::empty(),
        }
    }

    /// Create default intents (all non-privileged intents).
    #[staticmethod]
    fn default() -> Self {
        Self {
            inner: RustIntents::default_intents(),
        }
    }

    /// Create intents with all intents enabled (including privileged).
    #[staticmethod]
    fn all() -> Self {
        Self {
            inner: RustIntents::all_intents(),
        }
    }

    /// Create intents with no intents enabled.
    #[staticmethod]
    fn none() -> Self {
        Self {
            inner: RustIntents::none_intents(),
        }
    }

    /// Get the raw integer value of the intents.
    #[getter]
    fn value(&self) -> u32 {
        self.inner.bits()
    }

    /// Whether GUILDS intent is enabled.
    #[getter]
    fn guilds(&self) -> bool {
        self.inner.contains(RustIntents::GUILDS)
    }

    #[setter]
    fn set_guilds(&mut self, value: bool) {
        if value {
            self.inner.insert(RustIntents::GUILDS);
        } else {
            self.inner.remove(RustIntents::GUILDS);
        }
    }

    /// Whether GUILD_MEMBERS intent is enabled (privileged).
    #[getter]
    fn members(&self) -> bool {
        self.inner.contains(RustIntents::GUILD_MEMBERS)
    }

    #[setter]
    fn set_members(&mut self, value: bool) {
        if value {
            self.inner.insert(RustIntents::GUILD_MEMBERS);
        } else {
            self.inner.remove(RustIntents::GUILD_MEMBERS);
        }
    }

    /// Whether GUILD_MODERATION intent is enabled.
    #[getter]
    fn moderation(&self) -> bool {
        self.inner.contains(RustIntents::GUILD_MODERATION)
    }

    #[setter]
    fn set_moderation(&mut self, value: bool) {
        if value {
            self.inner.insert(RustIntents::GUILD_MODERATION);
        } else {
            self.inner.remove(RustIntents::GUILD_MODERATION);
        }
    }

    /// Whether GUILD_EMOJIS_AND_STICKERS intent is enabled.
    #[getter]
    fn emojis_and_stickers(&self) -> bool {
        self.inner.contains(RustIntents::GUILD_EMOJIS_AND_STICKERS)
    }

    #[setter]
    fn set_emojis_and_stickers(&mut self, value: bool) {
        if value {
            self.inner.insert(RustIntents::GUILD_EMOJIS_AND_STICKERS);
        } else {
            self.inner.remove(RustIntents::GUILD_EMOJIS_AND_STICKERS);
        }
    }

    /// Whether GUILD_INTEGRATIONS intent is enabled.
    #[getter]
    fn integrations(&self) -> bool {
        self.inner.contains(RustIntents::GUILD_INTEGRATIONS)
    }

    #[setter]
    fn set_integrations(&mut self, value: bool) {
        if value {
            self.inner.insert(RustIntents::GUILD_INTEGRATIONS);
        } else {
            self.inner.remove(RustIntents::GUILD_INTEGRATIONS);
        }
    }

    /// Whether GUILD_WEBHOOKS intent is enabled.
    #[getter]
    fn webhooks(&self) -> bool {
        self.inner.contains(RustIntents::GUILD_WEBHOOKS)
    }

    #[setter]
    fn set_webhooks(&mut self, value: bool) {
        if value {
            self.inner.insert(RustIntents::GUILD_WEBHOOKS);
        } else {
            self.inner.remove(RustIntents::GUILD_WEBHOOKS);
        }
    }

    /// Whether GUILD_INVITES intent is enabled.
    #[getter]
    fn invites(&self) -> bool {
        self.inner.contains(RustIntents::GUILD_INVITES)
    }

    #[setter]
    fn set_invites(&mut self, value: bool) {
        if value {
            self.inner.insert(RustIntents::GUILD_INVITES);
        } else {
            self.inner.remove(RustIntents::GUILD_INVITES);
        }
    }

    /// Whether GUILD_VOICE_STATES intent is enabled.
    #[getter]
    fn voice_states(&self) -> bool {
        self.inner.contains(RustIntents::GUILD_VOICE_STATES)
    }

    #[setter]
    fn set_voice_states(&mut self, value: bool) {
        if value {
            self.inner.insert(RustIntents::GUILD_VOICE_STATES);
        } else {
            self.inner.remove(RustIntents::GUILD_VOICE_STATES);
        }
    }

    /// Whether GUILD_PRESENCES intent is enabled (privileged).
    #[getter]
    fn presences(&self) -> bool {
        self.inner.contains(RustIntents::GUILD_PRESENCES)
    }

    #[setter]
    fn set_presences(&mut self, value: bool) {
        if value {
            self.inner.insert(RustIntents::GUILD_PRESENCES);
        } else {
            self.inner.remove(RustIntents::GUILD_PRESENCES);
        }
    }

    /// Whether GUILD_MESSAGES intent is enabled.
    #[getter]
    fn guild_messages(&self) -> bool {
        self.inner.contains(RustIntents::GUILD_MESSAGES)
    }

    #[setter]
    fn set_guild_messages(&mut self, value: bool) {
        if value {
            self.inner.insert(RustIntents::GUILD_MESSAGES);
        } else {
            self.inner.remove(RustIntents::GUILD_MESSAGES);
        }
    }

    /// Whether GUILD_MESSAGE_REACTIONS intent is enabled.
    #[getter]
    fn guild_reactions(&self) -> bool {
        self.inner.contains(RustIntents::GUILD_MESSAGE_REACTIONS)
    }

    #[setter]
    fn set_guild_reactions(&mut self, value: bool) {
        if value {
            self.inner.insert(RustIntents::GUILD_MESSAGE_REACTIONS);
        } else {
            self.inner.remove(RustIntents::GUILD_MESSAGE_REACTIONS);
        }
    }

    /// Whether GUILD_MESSAGE_TYPING intent is enabled.
    #[getter]
    fn guild_typing(&self) -> bool {
        self.inner.contains(RustIntents::GUILD_MESSAGE_TYPING)
    }

    #[setter]
    fn set_guild_typing(&mut self, value: bool) {
        if value {
            self.inner.insert(RustIntents::GUILD_MESSAGE_TYPING);
        } else {
            self.inner.remove(RustIntents::GUILD_MESSAGE_TYPING);
        }
    }

    /// Whether DIRECT_MESSAGES intent is enabled.
    #[getter]
    fn dm_messages(&self) -> bool {
        self.inner.contains(RustIntents::DIRECT_MESSAGES)
    }

    #[setter]
    fn set_dm_messages(&mut self, value: bool) {
        if value {
            self.inner.insert(RustIntents::DIRECT_MESSAGES);
        } else {
            self.inner.remove(RustIntents::DIRECT_MESSAGES);
        }
    }

    /// Whether DIRECT_MESSAGE_REACTIONS intent is enabled.
    #[getter]
    fn dm_reactions(&self) -> bool {
        self.inner.contains(RustIntents::DIRECT_MESSAGE_REACTIONS)
    }

    #[setter]
    fn set_dm_reactions(&mut self, value: bool) {
        if value {
            self.inner.insert(RustIntents::DIRECT_MESSAGE_REACTIONS);
        } else {
            self.inner.remove(RustIntents::DIRECT_MESSAGE_REACTIONS);
        }
    }

    /// Whether DIRECT_MESSAGE_TYPING intent is enabled.
    #[getter]
    fn dm_typing(&self) -> bool {
        self.inner.contains(RustIntents::DIRECT_MESSAGE_TYPING)
    }

    #[setter]
    fn set_dm_typing(&mut self, value: bool) {
        if value {
            self.inner.insert(RustIntents::DIRECT_MESSAGE_TYPING);
        } else {
            self.inner.remove(RustIntents::DIRECT_MESSAGE_TYPING);
        }
    }

    /// Whether MESSAGE_CONTENT intent is enabled (privileged).
    #[getter]
    fn message_content(&self) -> bool {
        self.inner.contains(RustIntents::MESSAGE_CONTENT)
    }

    #[setter]
    fn set_message_content(&mut self, value: bool) {
        if value {
            self.inner.insert(RustIntents::MESSAGE_CONTENT);
        } else {
            self.inner.remove(RustIntents::MESSAGE_CONTENT);
        }
    }

    /// Whether GUILD_SCHEDULED_EVENTS intent is enabled.
    #[getter]
    fn scheduled_events(&self) -> bool {
        self.inner.contains(RustIntents::GUILD_SCHEDULED_EVENTS)
    }

    #[setter]
    fn set_scheduled_events(&mut self, value: bool) {
        if value {
            self.inner.insert(RustIntents::GUILD_SCHEDULED_EVENTS);
        } else {
            self.inner.remove(RustIntents::GUILD_SCHEDULED_EVENTS);
        }
    }

    /// Whether AUTO_MODERATION_CONFIGURATION intent is enabled.
    #[getter]
    fn auto_moderation_configuration(&self) -> bool {
        self.inner
            .contains(RustIntents::AUTO_MODERATION_CONFIGURATION)
    }

    #[setter]
    fn set_auto_moderation_configuration(&mut self, value: bool) {
        if value {
            self.inner
                .insert(RustIntents::AUTO_MODERATION_CONFIGURATION);
        } else {
            self.inner
                .remove(RustIntents::AUTO_MODERATION_CONFIGURATION);
        }
    }

    /// Whether AUTO_MODERATION_EXECUTION intent is enabled.
    #[getter]
    fn auto_moderation_execution(&self) -> bool {
        self.inner.contains(RustIntents::AUTO_MODERATION_EXECUTION)
    }

    #[setter]
    fn set_auto_moderation_execution(&mut self, value: bool) {
        if value {
            self.inner.insert(RustIntents::AUTO_MODERATION_EXECUTION);
        } else {
            self.inner.remove(RustIntents::AUTO_MODERATION_EXECUTION);
        }
    }

    fn __repr__(&self) -> String {
        format!("<Intents value={}>", self.inner.bits())
    }

    fn __or__(&self, other: &Intents) -> Intents {
        Intents {
            inner: self.inner | other.inner,
        }
    }

    fn __and__(&self, other: &Intents) -> Intents {
        Intents {
            inner: self.inner & other.inner,
        }
    }

    fn __xor__(&self, other: &Intents) -> Intents {
        Intents {
            inner: self.inner ^ other.inner,
        }
    }

    fn __invert__(&self) -> Intents {
        Intents { inner: !self.inner }
    }
}

impl Intents {
    /// Get the inner Rust intents.
    pub fn inner(&self) -> RustIntents {
        self.inner
    }

    /// Create default intents (all non-privileged intents).
    pub fn default_intents() -> Self {
        Self {
            inner: RustIntents::default_intents(),
        }
    }
}
