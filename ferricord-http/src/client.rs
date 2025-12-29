//! HTTP client for Discord REST API
//!
//! This module provides a rate-limited HTTP client for making requests to the Discord API.

use std::sync::Arc;
use reqwest::{Client, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use tracing::{debug, error, warn};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use ferricord_core::{Error, Result};
use ferricord_model::{ChannelId, Message, MessageId, User, Guild, Channel, Member, GuildId, UserId};

use crate::ratelimit::RateLimiter;
use crate::routes::{self, Method, Route};

/// User agent for API requests.
const USER_AGENT: &str = concat!(
    "DiscordBot (https://github.com/FreeWiFi7749/ferricord, ",
    env!("CARGO_PKG_VERSION"),
    ")"
);

/// HTTP client for Discord API.
#[derive(Clone)]
pub struct HttpClient {
    /// The underlying HTTP client.
    client: Client,
    /// The bot token.
    token: String,
    /// Rate limiter.
    rate_limiter: Arc<RateLimiter>,
}

impl HttpClient {
    /// Create a new HTTP client with the given token.
    pub fn new(token: impl Into<String>) -> Result<Self> {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .map_err(|e| Error::http(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            client,
            token: token.into(),
            rate_limiter: Arc::new(RateLimiter::new()),
        })
    }

    /// Make a request to the Discord API.
    async fn request<T: DeserializeOwned>(&self, route: Route) -> Result<T> {
        self.request_with_body::<(), T>(route, None).await
    }

    /// Make a request with a JSON body.
    async fn request_with_body<B: Serialize, T: DeserializeOwned>(
        &self,
        route: Route,
        body: Option<&B>,
    ) -> Result<T> {
        let _guard = self.rate_limiter.acquire(&route.bucket).await;

        let mut retries = 0;
        const MAX_RETRIES: u32 = 5;

        loop {
            let mut request = match route.method {
                Method::Get => self.client.get(&route.url()),
                Method::Post => self.client.post(&route.url()),
                Method::Put => self.client.put(&route.url()),
                Method::Patch => self.client.patch(&route.url()),
                Method::Delete => self.client.delete(&route.url()),
            };

            request = request
                .header("Authorization", format!("Bot {}", self.token))
                .header("Content-Type", "application/json");

            if let Some(body) = body {
                request = request.json(body);
            }

            let response = request
                .send()
                .await
                .map_err(|e| Error::http(format!("Request failed: {}", e)))?;

            self.update_rate_limits(&route.bucket, &response);

            match response.status() {
                StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => {
                    let text = response
                        .text()
                        .await
                        .map_err(|e| Error::http(format!("Failed to read response: {}", e)))?;

                    if text.is_empty() {
                        return serde_json::from_str("null")
                            .map_err(|e| Error::Json(e));
                    }

                    return serde_json::from_str(&text).map_err(|e| {
                        error!("Failed to parse response: {} - Body: {}", e, text);
                        Error::Json(e)
                    });
                }
                StatusCode::TOO_MANY_REQUESTS => {
                    let body: serde_json::Value = response
                        .json()
                        .await
                        .map_err(|e| Error::http(format!("Failed to parse rate limit response: {}", e)))?;

                    let retry_after = body["retry_after"].as_f64().unwrap_or(1.0);
                    let global = body["global"].as_bool().unwrap_or(false);

                    warn!(
                        "Rate limited on {}: retry_after={}, global={}",
                        route.bucket, retry_after, global
                    );

                    self.rate_limiter
                        .handle_rate_limit(&route.bucket, retry_after, global)
                        .await;

                    retries += 1;
                    if retries >= MAX_RETRIES {
                        return Err(Error::RateLimited {
                            retry_after_ms: (retry_after * 1000.0) as u64,
                            global,
                        });
                    }
                    continue;
                }
                StatusCode::UNAUTHORIZED => {
                    return Err(Error::auth("Invalid token"));
                }
                StatusCode::FORBIDDEN => {
                    let body: serde_json::Value = response.json().await.unwrap_or_default();
                    let message = body["message"].as_str().unwrap_or("Forbidden");
                    return Err(Error::discord(403, message));
                }
                StatusCode::NOT_FOUND => {
                    let body: serde_json::Value = response.json().await.unwrap_or_default();
                    let message = body["message"].as_str().unwrap_or("Not found");
                    return Err(Error::discord(404, message));
                }
                status => {
                    let body: serde_json::Value = response.json().await.unwrap_or_default();
                    let code = body["code"].as_i64().unwrap_or(status.as_u16() as i64) as i32;
                    let message = body["message"]
                        .as_str()
                        .unwrap_or("Unknown error")
                        .to_string();
                    return Err(Error::discord(code, message));
                }
            }
        }
    }

    /// Update rate limit state from response headers.
    fn update_rate_limits(&self, route: &str, response: &Response) {
        let headers = response.headers();

        let remaining = headers
            .get("x-ratelimit-remaining")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok());

        let limit = headers
            .get("x-ratelimit-limit")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok());

        let reset_after = headers
            .get("x-ratelimit-reset-after")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok());

        let bucket = headers
            .get("x-ratelimit-bucket")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_string());

        let global = headers
            .get("x-ratelimit-global")
            .and_then(|v| v.to_str().ok())
            .map(|v| v == "true")
            .unwrap_or(false);

        self.rate_limiter
            .update_from_headers(route, remaining, limit, reset_after, bucket, global);

        debug!(
            "Rate limit update for {}: remaining={:?}, limit={:?}, reset_after={:?}",
            route, remaining, limit, reset_after
        );
    }

    // ========== Gateway ==========

    /// Get the gateway URL.
    pub async fn get_gateway(&self) -> Result<GatewayInfo> {
        self.request(routes::gateway::get()).await
    }

    /// Get the gateway URL with bot info.
    pub async fn get_gateway_bot(&self) -> Result<GatewayBotInfo> {
        self.request(routes::gateway::get_bot()).await
    }

    // ========== Channels ==========

    /// Get a channel by ID.
    pub async fn get_channel(&self, channel_id: ChannelId) -> Result<Channel> {
        self.request(routes::channels::get(channel_id)).await
    }

    /// Get messages in a channel.
    pub async fn get_messages(
        &self,
        channel_id: ChannelId,
        limit: Option<u32>,
    ) -> Result<Vec<Message>> {
        let mut route = routes::channels::get_messages(channel_id);
        if let Some(limit) = limit {
            route.path = format!("{}?limit={}", route.path, limit.min(100));
        }
        self.request(route).await
    }

    /// Get a specific message.
    pub async fn get_message(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> Result<Message> {
        self.request(routes::channels::get_message(channel_id, message_id))
            .await
    }

    /// Send a message to a channel.
    pub async fn send_message(
        &self,
        channel_id: ChannelId,
        content: impl Into<String>,
    ) -> Result<Message> {
        #[derive(Serialize)]
        struct CreateMessage {
            content: String,
        }

        let body = CreateMessage {
            content: content.into(),
        };

        self.request_with_body(routes::channels::create_message(channel_id), Some(&body))
            .await
    }

    /// Send a message with embeds.
    pub async fn send_message_with_embeds(
        &self,
        channel_id: ChannelId,
        content: Option<String>,
        embeds: Vec<ferricord_model::Embed>,
    ) -> Result<Message> {
        #[derive(Serialize)]
        struct CreateMessage {
            #[serde(skip_serializing_if = "Option::is_none")]
            content: Option<String>,
            embeds: Vec<ferricord_model::Embed>,
        }

        let body = CreateMessage { content, embeds };

        self.request_with_body(routes::channels::create_message(channel_id), Some(&body))
            .await
    }

    /// Edit a message.
    pub async fn edit_message(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
        content: impl Into<String>,
    ) -> Result<Message> {
        #[derive(Serialize)]
        struct EditMessage {
            content: String,
        }

        let body = EditMessage {
            content: content.into(),
        };

        self.request_with_body(
            routes::channels::edit_message(channel_id, message_id),
            Some(&body),
        )
        .await
    }

    /// Delete a message.
    pub async fn delete_message(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> Result<()> {
        self.request(routes::channels::delete_message(channel_id, message_id))
            .await
    }

    /// Trigger typing indicator.
    pub async fn trigger_typing(&self, channel_id: ChannelId) -> Result<()> {
        self.request_with_body::<(), ()>(routes::channels::trigger_typing(channel_id), None)
            .await
    }

    /// Add a reaction to a message.
    pub async fn add_reaction(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: &str,
    ) -> Result<()> {
        let encoded_emoji = utf8_percent_encode(emoji, NON_ALPHANUMERIC).to_string();
        self.request(routes::channels::create_reaction(
            channel_id,
            message_id,
            &encoded_emoji,
        ))
        .await
    }

    /// Remove own reaction from a message.
    pub async fn remove_own_reaction(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: &str,
    ) -> Result<()> {
        let encoded_emoji = utf8_percent_encode(emoji, NON_ALPHANUMERIC).to_string();
        self.request(routes::channels::delete_own_reaction(
            channel_id,
            message_id,
            &encoded_emoji,
        ))
        .await
    }

    // ========== Guilds ==========

    /// Get a guild by ID.
    pub async fn get_guild(&self, guild_id: GuildId) -> Result<Guild> {
        self.request(routes::guilds::get(guild_id)).await
    }

    /// Get guild channels.
    pub async fn get_guild_channels(&self, guild_id: GuildId) -> Result<Vec<Channel>> {
        self.request(routes::guilds::get_channels(guild_id)).await
    }

    /// Get a guild member.
    pub async fn get_member(&self, guild_id: GuildId, user_id: UserId) -> Result<Member> {
        self.request(routes::guilds::get_member(guild_id, user_id))
            .await
    }

    /// List guild members.
    pub async fn list_members(
        &self,
        guild_id: GuildId,
        limit: Option<u32>,
    ) -> Result<Vec<Member>> {
        let mut route = routes::guilds::list_members(guild_id);
        if let Some(limit) = limit {
            route.path = format!("{}?limit={}", route.path, limit.min(1000));
        }
        self.request(route).await
    }

    /// Kick a member from a guild.
    pub async fn kick_member(&self, guild_id: GuildId, user_id: UserId) -> Result<()> {
        self.request(routes::guilds::remove_member(guild_id, user_id))
            .await
    }

    /// Ban a member from a guild.
    pub async fn ban_member(
        &self,
        guild_id: GuildId,
        user_id: UserId,
        delete_message_seconds: Option<u32>,
    ) -> Result<()> {
        #[derive(Serialize)]
        struct BanRequest {
            #[serde(skip_serializing_if = "Option::is_none")]
            delete_message_seconds: Option<u32>,
        }

        let body = BanRequest {
            delete_message_seconds,
        };

        self.request_with_body(routes::guilds::create_ban(guild_id, user_id), Some(&body))
            .await
    }

    /// Unban a member from a guild.
    pub async fn unban_member(&self, guild_id: GuildId, user_id: UserId) -> Result<()> {
        self.request(routes::guilds::remove_ban(guild_id, user_id))
            .await
    }

    /// Leave a guild.
    pub async fn leave_guild(&self, guild_id: GuildId) -> Result<()> {
        self.request(routes::guilds::leave(guild_id)).await
    }

    // ========== Users ==========

    /// Get the current user.
    pub async fn get_current_user(&self) -> Result<User> {
        self.request(routes::users::get_current()).await
    }

    /// Get a user by ID.
    pub async fn get_user(&self, user_id: UserId) -> Result<User> {
        self.request(routes::users::get(user_id)).await
    }

    /// Create a DM channel with a user.
    pub async fn create_dm(&self, user_id: UserId) -> Result<Channel> {
        #[derive(Serialize)]
        struct CreateDm {
            recipient_id: UserId,
        }

        let body = CreateDm {
            recipient_id: user_id,
        };

        self.request_with_body(routes::users::create_dm(), Some(&body))
            .await
    }
}

impl std::fmt::Debug for HttpClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpClient")
            .field("token", &"[REDACTED]")
            .finish()
    }
}

/// Gateway information.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct GatewayInfo {
    /// The WSS URL that can be used for connecting to the gateway.
    pub url: String,
}

/// Gateway bot information.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct GatewayBotInfo {
    /// The WSS URL that can be used for connecting to the gateway.
    pub url: String,
    /// Recommended number of shards to use when connecting.
    pub shards: u32,
    /// Information on the current session start limit.
    pub session_start_limit: SessionStartLimit,
}

/// Session start limit information.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct SessionStartLimit {
    /// Total number of session starts the current user is allowed.
    pub total: u32,
    /// Remaining number of session starts the current user is allowed.
    pub remaining: u32,
    /// Number of milliseconds after which the limit resets.
    pub reset_after: u64,
    /// Number of identify requests allowed per 5 seconds.
    pub max_concurrency: u32,
}
