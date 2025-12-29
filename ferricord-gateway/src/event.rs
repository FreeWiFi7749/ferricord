//! Event handling for Gateway events
//!
//! This module provides traits and utilities for handling Gateway events.

use async_trait::async_trait;
use ferricord_model::gateway::{
    GatewayEvent, ReadyEvent, MessageDeleteEvent, MessageUpdateEvent,
    GuildMemberAddEvent, GuildMemberRemoveEvent, GuildMemberUpdateEvent,
    TypingStartEvent, PresenceUpdateEvent, VoiceServerUpdateEvent,
};
use ferricord_model::{Message, Guild, Channel};
use ferricord_model::gateway::UnavailableGuild;
use ferricord_model::voice::VoiceState;

/// Trait for handling Gateway events.
///
/// Implement this trait to handle events from the Discord Gateway.
/// All methods have default implementations that do nothing.
#[async_trait]
pub trait EventHandler: Send + Sync {
    /// Called when the client has successfully connected and received the Ready event.
    async fn ready(&self, _ready: ReadyEvent) {}

    /// Called when the client has successfully resumed a session.
    async fn resumed(&self) {}

    /// Called when a message is created.
    async fn message_create(&self, _message: Message) {}

    /// Called when a message is updated.
    async fn message_update(&self, _event: MessageUpdateEvent) {}

    /// Called when a message is deleted.
    async fn message_delete(&self, _event: MessageDeleteEvent) {}

    /// Called when a guild becomes available or the bot joins a new guild.
    async fn guild_create(&self, _guild: Guild) {}

    /// Called when a guild is updated.
    async fn guild_update(&self, _guild: Guild) {}

    /// Called when a guild becomes unavailable or the bot leaves/is removed from a guild.
    async fn guild_delete(&self, _guild: UnavailableGuild) {}

    /// Called when a channel is created.
    async fn channel_create(&self, _channel: Channel) {}

    /// Called when a channel is updated.
    async fn channel_update(&self, _channel: Channel) {}

    /// Called when a channel is deleted.
    async fn channel_delete(&self, _channel: Channel) {}

    /// Called when a member joins a guild.
    async fn guild_member_add(&self, _event: GuildMemberAddEvent) {}

    /// Called when a member leaves a guild.
    async fn guild_member_remove(&self, _event: GuildMemberRemoveEvent) {}

    /// Called when a member is updated.
    async fn guild_member_update(&self, _event: GuildMemberUpdateEvent) {}

    /// Called when a user starts typing.
    async fn typing_start(&self, _event: TypingStartEvent) {}

    /// Called when a user's presence is updated.
    async fn presence_update(&self, _event: PresenceUpdateEvent) {}

    /// Called when a user's voice state is updated.
    async fn voice_state_update(&self, _state: VoiceState) {}

    /// Called when a voice server is updated.
    async fn voice_server_update(&self, _event: VoiceServerUpdateEvent) {}

    /// Called when an interaction is created.
    async fn interaction_create(&self, _interaction: serde_json::Value) {}

    /// Called for any event that doesn't have a specific handler.
    async fn unknown_event(&self, _name: String, _data: serde_json::Value) {}
}

/// Dispatch a gateway event to the appropriate handler method.
pub async fn dispatch_event<H: EventHandler>(handler: &H, event: GatewayEvent) {
    match event {
        GatewayEvent::Ready(ready) => handler.ready(ready).await,
        GatewayEvent::Resumed => handler.resumed().await,
        GatewayEvent::MessageCreate(message) => handler.message_create(message).await,
        GatewayEvent::MessageUpdate(event) => handler.message_update(event).await,
        GatewayEvent::MessageDelete(event) => handler.message_delete(event).await,
        GatewayEvent::GuildCreate(guild) => handler.guild_create(guild).await,
        GatewayEvent::GuildUpdate(guild) => handler.guild_update(guild).await,
        GatewayEvent::GuildDelete(guild) => handler.guild_delete(guild).await,
        GatewayEvent::ChannelCreate(channel) => handler.channel_create(channel).await,
        GatewayEvent::ChannelUpdate(channel) => handler.channel_update(channel).await,
        GatewayEvent::ChannelDelete(channel) => handler.channel_delete(channel).await,
        GatewayEvent::GuildMemberAdd(event) => handler.guild_member_add(event).await,
        GatewayEvent::GuildMemberRemove(event) => handler.guild_member_remove(event).await,
        GatewayEvent::GuildMemberUpdate(event) => handler.guild_member_update(event).await,
        GatewayEvent::TypingStart(event) => handler.typing_start(event).await,
        GatewayEvent::PresenceUpdate(event) => handler.presence_update(event).await,
        GatewayEvent::VoiceStateUpdate(state) => handler.voice_state_update(state).await,
        GatewayEvent::VoiceServerUpdate(event) => handler.voice_server_update(event).await,
        GatewayEvent::InteractionCreate(interaction) => handler.interaction_create(interaction).await,
        GatewayEvent::Unknown(name, data) => handler.unknown_event(name, data).await,
        _ => {}
    }
}
