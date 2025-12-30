//! Discord API route definitions
//!
//! This module defines the routes for the Discord REST API.

use ferricord_model::{ChannelId, GuildId, MessageId, UserId, WebhookId};

/// Base URL for the Discord API.
pub const API_BASE: &str = "https://discord.com/api/v10";

/// Route builder for Discord API endpoints.
#[derive(Clone, Debug)]
pub struct Route {
    /// The HTTP method for this route.
    pub method: Method,
    /// The path for this route.
    pub path: String,
    /// The rate limit bucket key for this route.
    pub bucket: String,
}

/// HTTP methods.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl Route {
    /// Create a new route.
    fn new(method: Method, path: impl Into<String>, bucket: impl Into<String>) -> Self {
        Self {
            method,
            path: path.into(),
            bucket: bucket.into(),
        }
    }

    /// Get the full URL for this route.
    pub fn url(&self) -> String {
        format!("{}{}", API_BASE, self.path)
    }
}

/// Gateway routes.
pub mod gateway {
    use super::*;

    /// Get gateway URL.
    pub fn get() -> Route {
        Route::new(Method::Get, "/gateway", "gateway")
    }

    /// Get gateway URL with bot info.
    pub fn get_bot() -> Route {
        Route::new(Method::Get, "/gateway/bot", "gateway_bot")
    }
}

/// Channel routes.
pub mod channels {
    use super::*;

    /// Get a channel by ID.
    pub fn get(channel_id: ChannelId) -> Route {
        Route::new(
            Method::Get,
            format!("/channels/{}", channel_id),
            format!("channels:{}", channel_id),
        )
    }

    /// Modify a channel.
    pub fn modify(channel_id: ChannelId) -> Route {
        Route::new(
            Method::Patch,
            format!("/channels/{}", channel_id),
            format!("channels:{}", channel_id),
        )
    }

    /// Delete a channel.
    pub fn delete(channel_id: ChannelId) -> Route {
        Route::new(
            Method::Delete,
            format!("/channels/{}", channel_id),
            format!("channels:{}", channel_id),
        )
    }

    /// Get messages in a channel.
    pub fn get_messages(channel_id: ChannelId) -> Route {
        Route::new(
            Method::Get,
            format!("/channels/{}/messages", channel_id),
            format!("channels:{}:messages", channel_id),
        )
    }

    /// Get a specific message.
    pub fn get_message(channel_id: ChannelId, message_id: MessageId) -> Route {
        Route::new(
            Method::Get,
            format!("/channels/{}/messages/{}", channel_id, message_id),
            format!("channels:{}:messages", channel_id),
        )
    }

    /// Create a message in a channel.
    pub fn create_message(channel_id: ChannelId) -> Route {
        Route::new(
            Method::Post,
            format!("/channels/{}/messages", channel_id),
            format!("channels:{}:messages", channel_id),
        )
    }

    /// Edit a message.
    pub fn edit_message(channel_id: ChannelId, message_id: MessageId) -> Route {
        Route::new(
            Method::Patch,
            format!("/channels/{}/messages/{}", channel_id, message_id),
            format!("channels:{}:messages", channel_id),
        )
    }

    /// Delete a message.
    pub fn delete_message(channel_id: ChannelId, message_id: MessageId) -> Route {
        Route::new(
            Method::Delete,
            format!("/channels/{}/messages/{}", channel_id, message_id),
            format!("channels:{}:messages:delete", channel_id),
        )
    }

    /// Bulk delete messages.
    pub fn bulk_delete_messages(channel_id: ChannelId) -> Route {
        Route::new(
            Method::Post,
            format!("/channels/{}/messages/bulk-delete", channel_id),
            format!("channels:{}:messages:bulk-delete", channel_id),
        )
    }

    /// Create a reaction.
    pub fn create_reaction(channel_id: ChannelId, message_id: MessageId, emoji: &str) -> Route {
        Route::new(
            Method::Put,
            format!(
                "/channels/{}/messages/{}/reactions/{}/@me",
                channel_id, message_id, emoji
            ),
            format!("channels:{}:messages:reactions", channel_id),
        )
    }

    /// Delete own reaction.
    pub fn delete_own_reaction(channel_id: ChannelId, message_id: MessageId, emoji: &str) -> Route {
        Route::new(
            Method::Delete,
            format!(
                "/channels/{}/messages/{}/reactions/{}/@me",
                channel_id, message_id, emoji
            ),
            format!("channels:{}:messages:reactions", channel_id),
        )
    }

    /// Trigger typing indicator.
    pub fn trigger_typing(channel_id: ChannelId) -> Route {
        Route::new(
            Method::Post,
            format!("/channels/{}/typing", channel_id),
            format!("channels:{}:typing", channel_id),
        )
    }

    /// Get pinned messages.
    pub fn get_pinned_messages(channel_id: ChannelId) -> Route {
        Route::new(
            Method::Get,
            format!("/channels/{}/pins", channel_id),
            format!("channels:{}:pins", channel_id),
        )
    }

    /// Pin a message.
    pub fn pin_message(channel_id: ChannelId, message_id: MessageId) -> Route {
        Route::new(
            Method::Put,
            format!("/channels/{}/pins/{}", channel_id, message_id),
            format!("channels:{}:pins", channel_id),
        )
    }

    /// Unpin a message.
    pub fn unpin_message(channel_id: ChannelId, message_id: MessageId) -> Route {
        Route::new(
            Method::Delete,
            format!("/channels/{}/pins/{}", channel_id, message_id),
            format!("channels:{}:pins", channel_id),
        )
    }
}

/// Guild routes.
pub mod guilds {
    use super::*;

    /// Get a guild.
    pub fn get(guild_id: GuildId) -> Route {
        Route::new(
            Method::Get,
            format!("/guilds/{}", guild_id),
            format!("guilds:{}", guild_id),
        )
    }

    /// Modify a guild.
    pub fn modify(guild_id: GuildId) -> Route {
        Route::new(
            Method::Patch,
            format!("/guilds/{}", guild_id),
            format!("guilds:{}", guild_id),
        )
    }

    /// Get guild channels.
    pub fn get_channels(guild_id: GuildId) -> Route {
        Route::new(
            Method::Get,
            format!("/guilds/{}/channels", guild_id),
            format!("guilds:{}:channels", guild_id),
        )
    }

    /// Create a guild channel.
    pub fn create_channel(guild_id: GuildId) -> Route {
        Route::new(
            Method::Post,
            format!("/guilds/{}/channels", guild_id),
            format!("guilds:{}:channels", guild_id),
        )
    }

    /// Get guild member.
    pub fn get_member(guild_id: GuildId, user_id: UserId) -> Route {
        Route::new(
            Method::Get,
            format!("/guilds/{}/members/{}", guild_id, user_id),
            format!("guilds:{}:members", guild_id),
        )
    }

    /// List guild members.
    pub fn list_members(guild_id: GuildId) -> Route {
        Route::new(
            Method::Get,
            format!("/guilds/{}/members", guild_id),
            format!("guilds:{}:members", guild_id),
        )
    }

    /// Modify guild member.
    pub fn modify_member(guild_id: GuildId, user_id: UserId) -> Route {
        Route::new(
            Method::Patch,
            format!("/guilds/{}/members/{}", guild_id, user_id),
            format!("guilds:{}:members", guild_id),
        )
    }

    /// Kick a member.
    pub fn remove_member(guild_id: GuildId, user_id: UserId) -> Route {
        Route::new(
            Method::Delete,
            format!("/guilds/{}/members/{}", guild_id, user_id),
            format!("guilds:{}:members", guild_id),
        )
    }

    /// Get guild bans.
    pub fn get_bans(guild_id: GuildId) -> Route {
        Route::new(
            Method::Get,
            format!("/guilds/{}/bans", guild_id),
            format!("guilds:{}:bans", guild_id),
        )
    }

    /// Get a guild ban.
    pub fn get_ban(guild_id: GuildId, user_id: UserId) -> Route {
        Route::new(
            Method::Get,
            format!("/guilds/{}/bans/{}", guild_id, user_id),
            format!("guilds:{}:bans", guild_id),
        )
    }

    /// Create a guild ban.
    pub fn create_ban(guild_id: GuildId, user_id: UserId) -> Route {
        Route::new(
            Method::Put,
            format!("/guilds/{}/bans/{}", guild_id, user_id),
            format!("guilds:{}:bans", guild_id),
        )
    }

    /// Remove a guild ban.
    pub fn remove_ban(guild_id: GuildId, user_id: UserId) -> Route {
        Route::new(
            Method::Delete,
            format!("/guilds/{}/bans/{}", guild_id, user_id),
            format!("guilds:{}:bans", guild_id),
        )
    }

    /// Get guild roles.
    pub fn get_roles(guild_id: GuildId) -> Route {
        Route::new(
            Method::Get,
            format!("/guilds/{}/roles", guild_id),
            format!("guilds:{}:roles", guild_id),
        )
    }

    /// Create a guild role.
    pub fn create_role(guild_id: GuildId) -> Route {
        Route::new(
            Method::Post,
            format!("/guilds/{}/roles", guild_id),
            format!("guilds:{}:roles", guild_id),
        )
    }

    /// Leave a guild.
    pub fn leave(guild_id: GuildId) -> Route {
        Route::new(
            Method::Delete,
            format!("/users/@me/guilds/{}", guild_id),
            "users:@me:guilds".to_string(),
        )
    }
}

/// User routes.
pub mod users {
    use super::*;

    /// Get current user.
    pub fn get_current() -> Route {
        Route::new(Method::Get, "/users/@me", "users:@me")
    }

    /// Get a user.
    pub fn get(user_id: UserId) -> Route {
        Route::new(
            Method::Get,
            format!("/users/{}", user_id),
            "users".to_string(),
        )
    }

    /// Modify current user.
    pub fn modify_current() -> Route {
        Route::new(Method::Patch, "/users/@me", "users:@me")
    }

    /// Get current user guilds.
    pub fn get_current_guilds() -> Route {
        Route::new(Method::Get, "/users/@me/guilds", "users:@me:guilds")
    }

    /// Create DM channel.
    pub fn create_dm() -> Route {
        Route::new(Method::Post, "/users/@me/channels", "users:@me:channels")
    }
}

/// Webhook routes.
pub mod webhooks {
    use super::*;

    /// Execute a webhook.
    pub fn execute(webhook_id: WebhookId, token: &str) -> Route {
        Route::new(
            Method::Post,
            format!("/webhooks/{}/{}", webhook_id, token),
            format!("webhooks:{}:{}", webhook_id, token),
        )
    }

    /// Execute a webhook and wait for message.
    pub fn execute_wait(webhook_id: WebhookId, token: &str) -> Route {
        Route::new(
            Method::Post,
            format!("/webhooks/{}/{}?wait=true", webhook_id, token),
            format!("webhooks:{}:{}", webhook_id, token),
        )
    }
}

/// Interaction routes.
pub mod interactions {
    use super::*;

    /// Create interaction response.
    pub fn create_response(interaction_id: &str, token: &str) -> Route {
        Route::new(
            Method::Post,
            format!("/interactions/{}/{}/callback", interaction_id, token),
            format!("interactions:{}:{}", interaction_id, token),
        )
    }

    /// Get original interaction response.
    pub fn get_original_response(application_id: UserId, token: &str) -> Route {
        Route::new(
            Method::Get,
            format!("/webhooks/{}/{}/messages/@original", application_id, token),
            format!("webhooks:{}:{}", application_id, token),
        )
    }

    /// Edit original interaction response.
    pub fn edit_original_response(application_id: UserId, token: &str) -> Route {
        Route::new(
            Method::Patch,
            format!("/webhooks/{}/{}/messages/@original", application_id, token),
            format!("webhooks:{}:{}", application_id, token),
        )
    }

    /// Delete original interaction response.
    pub fn delete_original_response(application_id: UserId, token: &str) -> Route {
        Route::new(
            Method::Delete,
            format!("/webhooks/{}/{}/messages/@original", application_id, token),
            format!("webhooks:{}:{}", application_id, token),
        )
    }
}

/// Application command routes.
pub mod commands {
    use super::*;

    /// Get global application commands.
    pub fn get_global(application_id: UserId) -> Route {
        Route::new(
            Method::Get,
            format!("/applications/{}/commands", application_id),
            format!("applications:{}:commands", application_id),
        )
    }

    /// Create global application command.
    pub fn create_global(application_id: UserId) -> Route {
        Route::new(
            Method::Post,
            format!("/applications/{}/commands", application_id),
            format!("applications:{}:commands", application_id),
        )
    }

    /// Bulk overwrite global application commands.
    pub fn bulk_overwrite_global(application_id: UserId) -> Route {
        Route::new(
            Method::Put,
            format!("/applications/{}/commands", application_id),
            format!("applications:{}:commands", application_id),
        )
    }

    /// Get guild application commands.
    pub fn get_guild(application_id: UserId, guild_id: GuildId) -> Route {
        Route::new(
            Method::Get,
            format!(
                "/applications/{}/guilds/{}/commands",
                application_id, guild_id
            ),
            format!(
                "applications:{}:guilds:{}:commands",
                application_id, guild_id
            ),
        )
    }

    /// Create guild application command.
    pub fn create_guild(application_id: UserId, guild_id: GuildId) -> Route {
        Route::new(
            Method::Post,
            format!(
                "/applications/{}/guilds/{}/commands",
                application_id, guild_id
            ),
            format!(
                "applications:{}:guilds:{}:commands",
                application_id, guild_id
            ),
        )
    }

    /// Bulk overwrite guild application commands.
    pub fn bulk_overwrite_guild(application_id: UserId, guild_id: GuildId) -> Route {
        Route::new(
            Method::Put,
            format!(
                "/applications/{}/guilds/{}/commands",
                application_id, guild_id
            ),
            format!(
                "applications:{}:guilds:{}:commands",
                application_id, guild_id
            ),
        )
    }
}
