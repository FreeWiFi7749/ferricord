//! Gateway WebSocket connection handling
//!
//! This module handles the low-level WebSocket connection to the Discord Gateway.

use std::sync::Arc;

use flate2::{Decompress, FlushDecompress};
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tracing::{debug, error, trace, warn};

use ferricord_core::{Error, Result};
use ferricord_model::gateway::GatewayPayload;

const MAX_ZLIB_BUFFER_SIZE: usize = 10 * 1024 * 1024;
/// Zlib flush suffix that indicates end of a message
const ZLIB_SUFFIX: [u8; 4] = [0x00, 0x00, 0xff, 0xff];

/// Type alias for the WebSocket stream.
pub type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Gateway connection state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionState {
    /// Not connected.
    Disconnected,
    /// Connecting to the gateway.
    Connecting,
    /// Connected and ready.
    Connected,
    /// Resuming a previous session.
    Resuming,
    /// Connection is closing.
    Closing,
}

/// Gateway WebSocket connection.
pub struct GatewayConnection {
    /// The WebSocket sender.
    sender: Arc<Mutex<Option<SplitSink<WsStream, WsMessage>>>>,
    /// The WebSocket receiver.
    receiver: Arc<Mutex<Option<SplitStream<WsStream>>>>,
    /// Current connection state.
    state: Arc<Mutex<ConnectionState>>,
    /// Zlib decompression buffer (accumulates compressed data until flush suffix).
    zlib_buffer: Arc<Mutex<Vec<u8>>>,
    /// Persistent zlib decompressor that maintains state across messages.
    /// Discord's zlib-stream is stateful - the compressor keeps its dictionary
    /// across payloads and only inserts Z_SYNC_FLUSH markers between messages.
    decompressor: Arc<Mutex<Decompress>>,
}

impl GatewayConnection {
    /// Create a new gateway connection.
    pub fn new() -> Self {
        Self {
            sender: Arc::new(Mutex::new(None)),
            receiver: Arc::new(Mutex::new(None)),
            state: Arc::new(Mutex::new(ConnectionState::Disconnected)),
            zlib_buffer: Arc::new(Mutex::new(Vec::new())),
            // Initialize decompressor with zlib header expected (true)
            decompressor: Arc::new(Mutex::new(Decompress::new(true))),
        }
    }

    /// Connect to the gateway.
    pub async fn connect(&self, url: &str) -> Result<()> {
        *self.state.lock().await = ConnectionState::Connecting;

        // Reset zlib state for new connection - Discord starts a fresh zlib stream
        self.zlib_buffer.lock().await.clear();
        self.decompressor.lock().await.reset(true);

        // Build the gateway URL with proper path and query parameters
        // Discord expects: wss://gateway.discord.gg/?v=10&encoding=json&compress=zlib-stream
        // The URL from Discord API may not have a trailing slash, so we need to ensure
        // the path is "/" before adding query parameters
        let gateway_url = if url.contains('?') {
            // URL already has query params, append with &
            format!("{}&v=10&encoding=json&compress=zlib-stream", url)
        } else if url.ends_with('/') {
            // URL has trailing slash, just add query params
            format!("{}?v=10&encoding=json&compress=zlib-stream", url)
        } else {
            // URL has no trailing slash, add / before query params
            format!("{}/?v=10&encoding=json&compress=zlib-stream", url)
        };

        tracing::info!("Connecting to gateway: {}", gateway_url);

        let (ws_stream, _) = connect_async(&gateway_url)
            .await
            .map_err(|e| Error::websocket(format!("Failed to connect: {}", e)))?;

        let (sender, receiver) = ws_stream.split();

        *self.sender.lock().await = Some(sender);
        *self.receiver.lock().await = Some(receiver);
        *self.state.lock().await = ConnectionState::Connected;

        debug!("Connected to gateway");
        Ok(())
    }

    /// Disconnect from the gateway.
    pub async fn disconnect(&self) -> Result<()> {
        *self.state.lock().await = ConnectionState::Closing;

        if let Some(mut sender) = self.sender.lock().await.take() {
            let _ = sender.close().await;
        }

        *self.receiver.lock().await = None;
        *self.state.lock().await = ConnectionState::Disconnected;

        debug!("Disconnected from gateway");
        Ok(())
    }

    /// Send a payload to the gateway.
    pub async fn send(&self, payload: &GatewayPayload) -> Result<()> {
        let json = serde_json::to_string(payload)?;
        trace!("Sending: {}", json);

        let mut sender_guard = self.sender.lock().await;
        if let Some(sender) = sender_guard.as_mut() {
            sender
                .send(WsMessage::Text(json))
                .await
                .map_err(|e| Error::websocket(format!("Failed to send: {}", e)))?;
            Ok(())
        } else {
            Err(Error::websocket("Not connected"))
        }
    }

    /// Receive a payload from the gateway.
    pub async fn receive(&self) -> Result<Option<GatewayPayload>> {
        let mut receiver_guard = self.receiver.lock().await;
        let receiver = match receiver_guard.as_mut() {
            Some(r) => r,
            None => return Err(Error::websocket("Not connected")),
        };

        match receiver.next().await {
            Some(Ok(message)) => self.handle_message(message).await,
            Some(Err(e)) => {
                error!("WebSocket error: {}", e);
                Err(Error::websocket(format!("WebSocket error: {}", e)))
            }
            None => {
                warn!("WebSocket stream ended");
                Ok(None)
            }
        }
    }

    /// Handle a WebSocket message.
    async fn handle_message(&self, message: WsMessage) -> Result<Option<GatewayPayload>> {
        match message {
            WsMessage::Text(text) => {
                trace!("Received text: {}", text);
                let payload: GatewayPayload = serde_json::from_str(&text)?;
                Ok(Some(payload))
            }
            WsMessage::Binary(data) => {
                let mut buffer = self.zlib_buffer.lock().await;

                if buffer.len() + data.len() > MAX_ZLIB_BUFFER_SIZE {
                    buffer.clear();
                    self.decompressor.lock().await.reset(true);
                    return Err(Error::websocket(format!(
                        "Zlib buffer exceeded maximum size of {} bytes",
                        MAX_ZLIB_BUFFER_SIZE
                    )));
                }

                buffer.extend_from_slice(&data);

                // Check if the accumulated buffer ends with the zlib flush suffix
                // The suffix can be split across frames, so check the buffer not just the frame
                if buffer.len() >= 4 && buffer[buffer.len() - 4..] == ZLIB_SUFFIX {
                    let mut decompressor = self.decompressor.lock().await;

                    // Pre-allocate output buffer - Discord payloads typically decompress to ~10x size
                    let mut output = Vec::with_capacity(buffer.len() * 10);

                    // Use decompress_vec with SyncFlush to decompress incrementally
                    // This maintains the decompressor state (dictionary) across messages
                    match decompressor.decompress_vec(&buffer, &mut output, FlushDecompress::Sync) {
                        Ok(_status) => {
                            // Clear input buffer but keep decompressor state
                            buffer.clear();

                            let decompressed = String::from_utf8(output).map_err(|e| {
                                Error::websocket(format!(
                                    "Invalid UTF-8 in decompressed data: {}",
                                    e
                                ))
                            })?;

                            trace!("Received binary (decompressed): {}", decompressed);
                            let payload: GatewayPayload = serde_json::from_str(&decompressed)?;
                            Ok(Some(payload))
                        }
                        Err(e) => {
                            // On decompression error, reset both buffer and decompressor
                            buffer.clear();
                            decompressor.reset(true);
                            Err(Error::websocket(format!("Decompression failed: {}", e)))
                        }
                    }
                } else {
                    // Not a complete message yet, wait for more data
                    Ok(None)
                }
            }
            WsMessage::Ping(data) => {
                let mut sender_guard = self.sender.lock().await;
                if let Some(sender) = sender_guard.as_mut() {
                    let _ = sender.send(WsMessage::Pong(data)).await;
                }
                Ok(None)
            }
            WsMessage::Pong(_) => Ok(None),
            WsMessage::Close(frame) => {
                if let Some(frame) = frame {
                    warn!("Gateway closed: {} - {}", frame.code, frame.reason);
                } else {
                    warn!("Gateway closed without frame");
                }
                *self.state.lock().await = ConnectionState::Disconnected;
                Ok(None)
            }
            WsMessage::Frame(_) => Ok(None),
        }
    }

    /// Get the current connection state.
    pub async fn state(&self) -> ConnectionState {
        *self.state.lock().await
    }

    /// Check if connected.
    pub async fn is_connected(&self) -> bool {
        *self.state.lock().await == ConnectionState::Connected
    }
}

impl Default for GatewayConnection {
    fn default() -> Self {
        Self::new()
    }
}
