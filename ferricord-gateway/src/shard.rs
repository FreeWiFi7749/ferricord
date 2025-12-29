//! Shard management for Discord Gateway
//!
//! This module handles individual shard connections to the Discord Gateway.

use std::time::Duration;

use tokio::sync::mpsc;
use tokio::time::Instant;
use tracing::{debug, error, info, warn};

use ferricord_core::{Error, Result};
use ferricord_model::gateway::{
    GatewayEvent, GatewayOpcode, GatewayPayload, HelloEvent, Intents, ReadyEvent,
};

use crate::connection::{ConnectionState, GatewayConnection};

/// Configuration for a shard.
#[derive(Clone, Debug)]
pub struct ShardConfig {
    /// The bot token.
    pub token: String,
    /// The shard ID.
    pub shard_id: u32,
    /// Total number of shards.
    pub total_shards: u32,
    /// Gateway intents.
    pub intents: Intents,
    /// Large threshold for guild member chunking.
    pub large_threshold: u8,
}

impl ShardConfig {
    /// Create a new shard config.
    pub fn new(token: impl Into<String>, intents: Intents) -> Self {
        Self {
            token: token.into(),
            shard_id: 0,
            total_shards: 1,
            intents,
            large_threshold: 50,
        }
    }

    /// Set the shard ID and total shards.
    pub fn with_sharding(mut self, shard_id: u32, total_shards: u32) -> Self {
        self.shard_id = shard_id;
        self.total_shards = total_shards;
        self
    }

    /// Set the large threshold.
    pub fn with_large_threshold(mut self, threshold: u8) -> Self {
        self.large_threshold = threshold;
        self
    }
}

/// A single shard connection to the Discord Gateway.
pub struct Shard {
    /// Shard configuration.
    config: ShardConfig,
    /// The WebSocket connection.
    connection: GatewayConnection,
    /// Session ID for resuming.
    session_id: Option<String>,
    /// Resume gateway URL.
    resume_gateway_url: Option<String>,
    /// Last sequence number received.
    sequence: Option<u64>,
    /// Heartbeat interval in milliseconds.
    heartbeat_interval: Option<u64>,
    /// Last heartbeat sent time.
    last_heartbeat: Option<Instant>,
    /// Whether we received a heartbeat ACK.
    heartbeat_ack: bool,
    /// Event sender channel.
    event_tx: Option<mpsc::UnboundedSender<GatewayEvent>>,
}

impl Shard {
    /// Create a new shard.
    pub fn new(config: ShardConfig) -> Self {
        Self {
            config,
            connection: GatewayConnection::new(),
            session_id: None,
            resume_gateway_url: None,
            sequence: None,
            heartbeat_interval: None,
            last_heartbeat: None,
            heartbeat_ack: true,
            event_tx: None,
        }
    }

    /// Get the shard ID.
    pub fn id(&self) -> u32 {
        self.config.shard_id
    }

    /// Connect to the gateway.
    pub async fn connect(&mut self, gateway_url: &str) -> Result<()> {
        info!("Shard {} connecting to gateway", self.config.shard_id);
        self.connection.connect(gateway_url).await
    }

    /// Start the shard event loop.
    pub async fn run(
        &mut self,
        gateway_url: &str,
        event_tx: mpsc::UnboundedSender<GatewayEvent>,
    ) -> Result<()> {
        self.event_tx = Some(event_tx);
        self.connect(gateway_url).await?;

        loop {
            tokio::select! {
                _ = self.heartbeat_tick() => {
                    if let Err(e) = self.send_heartbeat().await {
                        error!("Shard {} heartbeat failed: {}", self.config.shard_id, e);
                        break;
                    }
                }
                result = self.connection.receive() => {
                    match result {
                        Ok(Some(payload)) => {
                            if let Err(e) = self.handle_payload(payload).await {
                                error!("Shard {} payload handling failed: {}", self.config.shard_id, e);
                            }
                        }
                        Ok(None) => {
                            if self.connection.state().await == ConnectionState::Disconnected {
                                warn!("Shard {} disconnected", self.config.shard_id);
                                break;
                            }
                        }
                        Err(e) => {
                            error!("Shard {} receive error: {}", self.config.shard_id, e);
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Wait for the next heartbeat tick.
    async fn heartbeat_tick(&self) {
        if let Some(interval_ms) = self.heartbeat_interval {
            tokio::time::sleep(Duration::from_millis(interval_ms)).await;
        } else {
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    }

    /// Handle a gateway payload.
    async fn handle_payload(&mut self, payload: GatewayPayload) -> Result<()> {
        if let Some(seq) = payload.s {
            self.sequence = Some(seq);
        }

        let opcode = GatewayOpcode::try_from(payload.op)
            .map_err(|op| Error::gateway(format!("Unknown opcode: {}", op)))?;

        match opcode {
            GatewayOpcode::Dispatch => {
                self.handle_dispatch(payload).await?;
            }
            GatewayOpcode::Heartbeat => {
                debug!("Shard {} received heartbeat request", self.config.shard_id);
                self.send_heartbeat().await?;
            }
            GatewayOpcode::Reconnect => {
                info!("Shard {} received reconnect", self.config.shard_id);
                self.emit_event(GatewayEvent::Reconnect);
            }
            GatewayOpcode::InvalidSession => {
                let resumable = payload.d.and_then(|d| d.as_bool()).unwrap_or(false);
                warn!(
                    "Shard {} invalid session, resumable: {}",
                    self.config.shard_id, resumable
                );
                self.emit_event(GatewayEvent::InvalidSession(resumable));

                if !resumable {
                    self.session_id = None;
                    self.sequence = None;
                }
            }
            GatewayOpcode::Hello => {
                let hello: HelloEvent = serde_json::from_value(payload.d.unwrap_or_default())?;
                debug!(
                    "Shard {} received hello, heartbeat_interval: {}ms",
                    self.config.shard_id, hello.heartbeat_interval
                );
                self.heartbeat_interval = Some(hello.heartbeat_interval);
                self.emit_event(GatewayEvent::Hello(hello));

                if self.session_id.is_some() {
                    self.send_resume().await?;
                } else {
                    self.send_identify().await?;
                }
            }
            GatewayOpcode::HeartbeatAck => {
                debug!("Shard {} received heartbeat ACK", self.config.shard_id);
                self.heartbeat_ack = true;
            }
            _ => {
                debug!(
                    "Shard {} received unhandled opcode: {:?}",
                    self.config.shard_id, opcode
                );
            }
        }

        Ok(())
    }

    /// Handle a dispatch event.
    async fn handle_dispatch(&mut self, payload: GatewayPayload) -> Result<()> {
        let event_name = payload.t.as_deref().unwrap_or("UNKNOWN");
        let data = payload.d.unwrap_or_default();

        debug!(
            "Shard {} received event: {}",
            self.config.shard_id, event_name
        );

        let event = match event_name {
            "READY" => {
                let ready: ReadyEvent = serde_json::from_value(data)?;
                self.session_id = Some(ready.session_id.clone());
                self.resume_gateway_url = Some(ready.resume_gateway_url.clone());
                info!(
                    "Shard {} ready, session_id: {}",
                    self.config.shard_id, ready.session_id
                );
                GatewayEvent::Ready(ready)
            }
            "RESUMED" => {
                info!("Shard {} resumed", self.config.shard_id);
                GatewayEvent::Resumed
            }
            "MESSAGE_CREATE" => {
                let message = serde_json::from_value(data)?;
                GatewayEvent::MessageCreate(Box::new(message))
            }
            "MESSAGE_UPDATE" => {
                let event = serde_json::from_value(data)?;
                GatewayEvent::MessageUpdate(event)
            }
            "MESSAGE_DELETE" => {
                let event = serde_json::from_value(data)?;
                GatewayEvent::MessageDelete(event)
            }
            "GUILD_CREATE" => {
                let guild = serde_json::from_value(data)?;
                GatewayEvent::GuildCreate(Box::new(guild))
            }
            "GUILD_UPDATE" => {
                let guild = serde_json::from_value(data)?;
                GatewayEvent::GuildUpdate(Box::new(guild))
            }
            "GUILD_DELETE" => {
                let guild = serde_json::from_value(data)?;
                GatewayEvent::GuildDelete(guild)
            }
            "CHANNEL_CREATE" => {
                let channel = serde_json::from_value(data)?;
                GatewayEvent::ChannelCreate(Box::new(channel))
            }
            "CHANNEL_UPDATE" => {
                let channel = serde_json::from_value(data)?;
                GatewayEvent::ChannelUpdate(Box::new(channel))
            }
            "CHANNEL_DELETE" => {
                let channel = serde_json::from_value(data)?;
                GatewayEvent::ChannelDelete(Box::new(channel))
            }
            "GUILD_MEMBER_ADD" => {
                let event = serde_json::from_value(data)?;
                GatewayEvent::GuildMemberAdd(event)
            }
            "GUILD_MEMBER_REMOVE" => {
                let event = serde_json::from_value(data)?;
                GatewayEvent::GuildMemberRemove(event)
            }
            "GUILD_MEMBER_UPDATE" => {
                let event = serde_json::from_value(data)?;
                GatewayEvent::GuildMemberUpdate(event)
            }
            "INTERACTION_CREATE" => GatewayEvent::InteractionCreate(data),
            "TYPING_START" => {
                let event = serde_json::from_value(data)?;
                GatewayEvent::TypingStart(event)
            }
            "VOICE_STATE_UPDATE" => {
                let state = serde_json::from_value(data)?;
                GatewayEvent::VoiceStateUpdate(state)
            }
            "VOICE_SERVER_UPDATE" => {
                let event = serde_json::from_value(data)?;
                GatewayEvent::VoiceServerUpdate(event)
            }
            "PRESENCE_UPDATE" => {
                let event = serde_json::from_value(data)?;
                GatewayEvent::PresenceUpdate(event)
            }
            _ => {
                debug!(
                    "Shard {} unhandled event: {}",
                    self.config.shard_id, event_name
                );
                GatewayEvent::Unknown(event_name.to_string(), data)
            }
        };

        self.emit_event(event);
        Ok(())
    }

    /// Emit an event to the event channel.
    fn emit_event(&self, event: GatewayEvent) {
        if let Some(tx) = &self.event_tx {
            if let Err(e) = tx.send(event) {
                error!("Shard {} failed to send event: {}", self.config.shard_id, e);
            }
        }
    }

    /// Send a heartbeat.
    async fn send_heartbeat(&mut self) -> Result<()> {
        if !self.heartbeat_ack {
            warn!(
                "Shard {} did not receive heartbeat ACK, reconnecting",
                self.config.shard_id
            );
            return Err(Error::gateway("Heartbeat ACK not received"));
        }

        self.heartbeat_ack = false;
        self.last_heartbeat = Some(Instant::now());

        let payload = GatewayPayload {
            op: GatewayOpcode::Heartbeat as u8,
            d: self.sequence.map(serde_json::Value::from),
            s: None,
            t: None,
        };

        debug!("Shard {} sending heartbeat", self.config.shard_id);
        self.connection.send(&payload).await
    }

    /// Send an identify payload.
    async fn send_identify(&self) -> Result<()> {
        info!("Shard {} sending identify", self.config.shard_id);

        let identify = serde_json::json!({
            "token": self.config.token,
            "intents": self.config.intents.bits(),
            "properties": {
                "os": std::env::consts::OS,
                "browser": "ferricord",
                "device": "ferricord"
            },
            "compress": true,
            "large_threshold": self.config.large_threshold,
            "shard": [self.config.shard_id, self.config.total_shards]
        });

        let payload = GatewayPayload {
            op: GatewayOpcode::Identify as u8,
            d: Some(identify),
            s: None,
            t: None,
        };

        self.connection.send(&payload).await
    }

    /// Send a resume payload.
    async fn send_resume(&self) -> Result<()> {
        let session_id = self
            .session_id
            .as_ref()
            .ok_or_else(|| Error::gateway("No session ID for resume"))?;

        info!(
            "Shard {} sending resume, session_id: {}",
            self.config.shard_id, session_id
        );

        let resume = serde_json::json!({
            "token": self.config.token,
            "session_id": session_id,
            "seq": self.sequence
        });

        let payload = GatewayPayload {
            op: GatewayOpcode::Resume as u8,
            d: Some(resume),
            s: None,
            t: None,
        };

        self.connection.send(&payload).await
    }

    /// Update presence.
    pub async fn update_presence(
        &self,
        status: &str,
        activity: Option<serde_json::Value>,
    ) -> Result<()> {
        let presence = serde_json::json!({
            "since": null,
            "activities": activity.map(|a| vec![a]).unwrap_or_default(),
            "status": status,
            "afk": false
        });

        let payload = GatewayPayload {
            op: GatewayOpcode::PresenceUpdate as u8,
            d: Some(presence),
            s: None,
            t: None,
        };

        self.connection.send(&payload).await
    }

    /// Request guild members.
    pub async fn request_guild_members(
        &self,
        guild_id: u64,
        query: Option<&str>,
        limit: u32,
    ) -> Result<()> {
        let request = serde_json::json!({
            "guild_id": guild_id.to_string(),
            "query": query.unwrap_or(""),
            "limit": limit
        });

        let payload = GatewayPayload {
            op: GatewayOpcode::RequestGuildMembers as u8,
            d: Some(request),
            s: None,
            t: None,
        };

        self.connection.send(&payload).await
    }
}
