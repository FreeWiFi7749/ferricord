//! Python Client implementation
//!
//! This module provides the main Client class for Python.

use std::collections::HashMap;
use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyTuple;
use tokio::sync::{mpsc, RwLock};
use tokio::task::JoinHandle;
use tracing::{error, info};

use ferricord_cache::Cache;
use ferricord_gateway::{Shard, ShardConfig};
use ferricord_http::HttpClient;
use ferricord_model::gateway::GatewayEvent;
use ferricord_model::id::GuildId;

use crate::intents::Intents;
use crate::models::{PyChannel, PyGuild, PyMessage, PyUser};

/// The main client for interacting with Discord.
///
/// Example:
///     ```python
///     from ferricord import Client, Intents
///
///     intents = Intents.default()
///     intents.message_content = True
///
///     client = Client(intents=intents)
///
///     @client.event
///     async def on_ready():
///         print(f"Logged in as {client.user}")
///
///     @client.event
///     async def on_message(message):
///         if message.content == "!ping":
///             await message.channel.send("Pong!")
///
///     client.run("YOUR_BOT_TOKEN")
///     ```
#[pyclass]
pub struct Client {
    /// Gateway intents.
    intents: Intents,
    /// Event handlers.
    event_handlers: Arc<RwLock<HashMap<String, PyObject>>>,
    /// Slash command handlers (command_name -> handler).
    slash_command_handlers: Arc<RwLock<HashMap<String, PyObject>>>,
    /// Component handlers (custom_id -> handler).
    component_handlers: Arc<RwLock<HashMap<String, PyObject>>>,
    /// Modal handlers (custom_id -> handler).
    modal_handlers: Arc<RwLock<HashMap<String, PyObject>>>,
    /// The HTTP client (reused across runs).
    http: Arc<RwLock<Option<Arc<HttpClient>>>>,
    /// The cache.
    cache: Arc<Cache>,
    /// Whether the client is running.
    running: Arc<RwLock<bool>>,
    /// Task handles for cleanup on close.
    task_handles: Arc<RwLock<Vec<JoinHandle<()>>>>,
    /// Application ID (set after connecting).
    application_id: Arc<RwLock<Option<u64>>>,
}

#[pymethods]
impl Client {
    /// Create a new Client.
    ///
    /// Args:
    ///     intents: Gateway intents to use.
    #[new]
    #[pyo3(signature = (intents=None))]
    fn new(intents: Option<Intents>) -> Self {
        let intents = intents.unwrap_or_else(Intents::default_intents);

        Self {
            intents,
            event_handlers: Arc::new(RwLock::new(HashMap::new())),
            slash_command_handlers: Arc::new(RwLock::new(HashMap::new())),
            component_handlers: Arc::new(RwLock::new(HashMap::new())),
            modal_handlers: Arc::new(RwLock::new(HashMap::new())),
            http: Arc::new(RwLock::new(None)),
            cache: Arc::new(Cache::new()),
            running: Arc::new(RwLock::new(false)),
            task_handles: Arc::new(RwLock::new(Vec::new())),
            application_id: Arc::new(RwLock::new(None)),
        }
    }

    /// Register an event handler.
    ///
    /// This is typically used as a decorator:
    ///     ```python
    ///     @client.event
    ///     async def on_message(message):
    ///         pass
    ///     ```
    fn event(&self, py: Python<'_>, func: PyObject) -> PyResult<PyObject> {
        let func_name = func.getattr(py, "__name__")?.extract::<String>(py)?;

        let event_name = if let Some(stripped) = func_name.strip_prefix("on_") {
            stripped.to_string()
        } else {
            func_name.clone()
        };

        let handlers = self.event_handlers.clone();
        let func_clone = func.clone_ref(py);

        if let Ok(mut guard) = handlers.try_write() {
            guard.insert(event_name, func_clone);
        } else {
            let handlers_clone = handlers.clone();
            pyo3_async_runtimes::tokio::get_runtime().spawn(async move {
                handlers_clone.write().await.insert(event_name, func_clone);
            });
        }

        Ok(func)
    }

    /// Run the client with the given token.
    ///
    /// This is a blocking call that runs the event loop.
    ///
    /// Args:
    ///     token: The bot token to use.
    fn run(&self, py: Python<'_>, token: String) -> PyResult<()> {
        let intents = self.intents.inner();
        let event_handlers = self.event_handlers.clone();
        let cache = self.cache.clone();
        let running = self.running.clone();
        let http_client = self.http.clone();
        let error_holder: Arc<RwLock<Option<String>>> = Arc::new(RwLock::new(None));
        let error_holder_clone = error_holder.clone();

        // Use a flag to signal shutdown from signal checker
        let shutdown_flag = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let shutdown_flag_clone = shutdown_flag.clone();

        // Spawn a thread to periodically check Python signals (Ctrl+C)
        // This is necessary because tokio::signal::ctrl_c() doesn't work well
        // when Python has already registered its own signal handler
        let signal_thread = std::thread::spawn(move || {
            while !shutdown_flag_clone.load(std::sync::atomic::Ordering::Relaxed) {
                std::thread::sleep(std::time::Duration::from_millis(100));
                // Check if Python received a signal (like SIGINT from Ctrl+C)
                let got_signal = Python::with_gil(|py| py.check_signals().is_err());
                if got_signal {
                    shutdown_flag_clone.store(true, std::sync::atomic::Ordering::Relaxed);
                    break;
                }
            }
        });

        let shutdown_flag_async = shutdown_flag.clone();

        py.allow_threads(|| {
            pyo3_async_runtimes::tokio::get_runtime().block_on(async move {
                *running.write().await = true;

                let http = match HttpClient::new(&token) {
                    Ok(http) => Arc::new(http),
                    Err(e) => {
                        error!("Failed to create HTTP client: {}", e);
                        *error_holder_clone.write().await =
                            Some(format!("Failed to create HTTP client: {}", e));
                        return;
                    }
                };

                // Store HTTP client for REST API operations
                *http_client.write().await = Some(http.clone());

                let gateway_info = match http.get_gateway_bot().await {
                    Ok(info) => info,
                    Err(e) => {
                        error!("Failed to get gateway info: {}", e);
                        *error_holder_clone.write().await =
                            Some(format!("Failed to get gateway info: {}", e));
                        return;
                    }
                };

                info!("Gateway URL: {}", gateway_info.url);
                info!("Recommended shards: {}", gateway_info.shards);

                let config = ShardConfig::new(token, intents);
                let mut shard = Shard::new(config);

                let (event_tx, mut event_rx) = mpsc::unbounded_channel::<GatewayEvent>();

                // Clone error_holder for the shard task
                let shard_error_holder = error_holder_clone.clone();
                let shard_running = running.clone();

                let shard_handle = tokio::spawn(async move {
                    match shard.run(&gateway_info.url, event_tx).await {
                        Ok(()) => {
                            info!("Shard disconnected normally");
                        }
                        Err(e) => {
                            error!("Shard error: {}", e);
                            *shard_error_holder.write().await = Some(format!("Shard error: {}", e));
                        }
                    }
                    // Signal that we should stop when shard exits
                    *shard_running.write().await = false;
                });

                // Pin the shard handle so we can poll it in the select
                let mut shard_handle = std::pin::pin!(shard_handle);
                let mut got_sigint = false;

                // Create an interval for checking the shutdown flag
                let mut signal_check_interval =
                    tokio::time::interval(std::time::Duration::from_millis(100));

                while *running.read().await {
                    tokio::select! {
                        // Check if signal thread detected Ctrl+C
                        _ = signal_check_interval.tick() => {
                            if shutdown_flag_async.load(std::sync::atomic::Ordering::Relaxed) {
                                info!("Received Ctrl+C, shutting down...");
                                got_sigint = true;
                                break;
                            }
                        }
                        // Wait for shard task to complete (error or normal disconnect)
                        result = &mut shard_handle => {
                            match result {
                                Ok(()) => info!("Shard task completed normally"),
                                Err(e) => error!("Shard task panicked: {:?}", e),
                            }
                            break;
                        }
                        // Process incoming events from the shard
                        Some(event) = event_rx.recv() => {
                            Self::handle_event(&event_handlers, &cache, event).await;
                        }
                    }
                }

                // Clean up
                if !shard_handle.is_finished() {
                    shard_handle.abort();
                }

                // Store whether we got SIGINT so we can raise KeyboardInterrupt
                if got_sigint {
                    *error_holder_clone.write().await = Some("KeyboardInterrupt".to_string());
                }
            });
        });

        // Signal the signal thread to stop and wait for it
        shutdown_flag.store(true, std::sync::atomic::Ordering::Relaxed);
        let _ = signal_thread.join();

        if let Some(err) = pyo3_async_runtimes::tokio::get_runtime()
            .block_on(async { error_holder.read().await.clone() })
        {
            // Raise KeyboardInterrupt for Ctrl+C, RuntimeError for other errors
            if err == "KeyboardInterrupt" {
                return Err(pyo3::exceptions::PyKeyboardInterrupt::new_err(
                    "Received Ctrl+C",
                ));
            }
            return Err(pyo3::exceptions::PyRuntimeError::new_err(err));
        }

        Ok(())
    }

    /// Start the client asynchronously.
    ///
    /// Unlike `run()`, this returns immediately and runs in the background.
    ///
    /// Args:
    ///     token: The bot token to use.
    fn start<'py>(&mut self, py: Python<'py>, token: String) -> PyResult<Bound<'py, PyAny>> {
        let intents = self.intents.inner();
        let event_handlers = self.event_handlers.clone();
        let cache = self.cache.clone();
        let running = self.running.clone();
        let task_handles = self.task_handles.clone();
        let http_client = self.http.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            *running.write().await = true;

            let http = HttpClient::new(&token)
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            let http = Arc::new(http);

            // Store HTTP client for REST API operations
            *http_client.write().await = Some(http.clone());

            let gateway_info = http
                .get_gateway_bot()
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            info!("Gateway URL: {}", gateway_info.url);

            let config = ShardConfig::new(token, intents);
            let mut shard = Shard::new(config);

            let (event_tx, mut event_rx) = mpsc::unbounded_channel::<GatewayEvent>();

            let shard_handle = tokio::spawn(async move {
                if let Err(e) = shard.run(&gateway_info.url, event_tx).await {
                    error!("Shard error: {}", e);
                }
            });

            let event_handle = tokio::spawn(async move {
                while *running.read().await {
                    if let Some(event) = event_rx.recv().await {
                        Self::handle_event(&event_handlers, &cache, event).await;
                    }
                }
            });

            // Store handles for cleanup on close()
            let mut handles = task_handles.write().await;
            handles.push(shard_handle);
            handles.push(event_handle);

            Ok(())
        })
    }

    /// Close the client connection.
    ///
    /// This will stop all running tasks and clean up resources.
    fn close<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let running = self.running.clone();
        let task_handles = self.task_handles.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            // Signal tasks to stop
            *running.write().await = false;

            // Take ownership of handles and abort them
            let handles: Vec<JoinHandle<()>> = {
                let mut guard = task_handles.write().await;
                std::mem::take(&mut *guard)
            };

            // Abort and await all tasks
            for handle in handles {
                handle.abort();
                // Ignore JoinError from abort
                let _ = handle.await;
            }

            Ok(())
        })
    }

    /// Get the current user.
    #[getter]
    fn user(&self) -> Option<PyUser> {
        self.cache.current_user().map(|u| PyUser::new(u.into()))
    }

    /// Get all guilds the bot is in.
    ///
    /// Note: This clones all guilds which can be expensive for large bots.
    /// Consider using `guild_ids()` for lightweight access.
    #[getter]
    fn guilds(&self) -> Vec<PyGuild> {
        self.cache
            .guilds()
            .into_iter()
            .map(|g| PyGuild::new((*g).clone()))
            .collect()
    }

    /// Get all guild IDs (lightweight).
    ///
    /// This is more efficient than `guilds` for large bots as it doesn't
    /// clone the full guild objects.
    fn guild_ids(&self) -> Vec<u64> {
        self.cache
            .guilds()
            .into_iter()
            .map(|g| g.id.get())
            .collect()
    }

    /// Get a specific guild by ID.
    ///
    /// Args:
    ///     guild_id: The ID of the guild to get.
    ///
    /// Returns:
    ///     The guild if found, None otherwise.
    fn get_guild(&self, guild_id: u64) -> Option<PyGuild> {
        self.cache
            .guild(GuildId::new(guild_id))
            .map(|g| PyGuild::new((*g).clone()))
    }

    /// Get the number of guilds.
    #[getter]
    fn guild_count(&self) -> usize {
        self.cache.guild_count()
    }

    /// Get cache statistics.
    fn cache_stats(&self) -> String {
        self.cache.stats().to_string()
    }

    // ========== REST API Methods (Phase 2) ==========

    /// Send a message to a channel.
    ///
    /// Args:
    ///     channel_id: The ID of the channel to send the message to.
    ///     content: The message content.
    ///
    /// Returns:
    ///     The sent message.
    fn send_message<'py>(
        &self,
        py: Python<'py>,
        channel_id: u64,
        content: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let http = self.http.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let http_guard = http.read().await;
            let http = http_guard.as_ref().ok_or_else(|| {
                pyo3::exceptions::PyRuntimeError::new_err("Client is not connected")
            })?;

            let message = http
                .send_message(ferricord_model::ChannelId::new(channel_id), content)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Python::with_gil(|py| {
                PyMessage::new(message)
                    .into_pyobject(py)
                    .map(|o| o.into_any().unbind())
            })
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("{:?}", e)))
        })
    }

    /// Edit a message.
    ///
    /// Args:
    ///     channel_id: The ID of the channel the message is in.
    ///     message_id: The ID of the message to edit.
    ///     content: The new message content.
    ///
    /// Returns:
    ///     The edited message.
    fn edit_message<'py>(
        &self,
        py: Python<'py>,
        channel_id: u64,
        message_id: u64,
        content: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let http = self.http.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let http_guard = http.read().await;
            let http = http_guard.as_ref().ok_or_else(|| {
                pyo3::exceptions::PyRuntimeError::new_err("Client is not connected")
            })?;

            let message = http
                .edit_message(
                    ferricord_model::ChannelId::new(channel_id),
                    ferricord_model::MessageId::new(message_id),
                    content,
                )
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Python::with_gil(|py| {
                PyMessage::new(message)
                    .into_pyobject(py)
                    .map(|o| o.into_any().unbind())
            })
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("{:?}", e)))
        })
    }

    /// Delete a message.
    ///
    /// Args:
    ///     channel_id: The ID of the channel the message is in.
    ///     message_id: The ID of the message to delete.
    fn delete_message<'py>(
        &self,
        py: Python<'py>,
        channel_id: u64,
        message_id: u64,
    ) -> PyResult<Bound<'py, PyAny>> {
        let http = self.http.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let http_guard = http.read().await;
            let http = http_guard.as_ref().ok_or_else(|| {
                pyo3::exceptions::PyRuntimeError::new_err("Client is not connected")
            })?;

            http.delete_message(
                ferricord_model::ChannelId::new(channel_id),
                ferricord_model::MessageId::new(message_id),
            )
            .await
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Add a reaction to a message.
    ///
    /// Args:
    ///     channel_id: The ID of the channel the message is in.
    ///     message_id: The ID of the message to react to.
    ///     emoji: The emoji to react with (e.g., "👍" or "custom:123456789").
    fn add_reaction<'py>(
        &self,
        py: Python<'py>,
        channel_id: u64,
        message_id: u64,
        emoji: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let http = self.http.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let http_guard = http.read().await;
            let http = http_guard.as_ref().ok_or_else(|| {
                pyo3::exceptions::PyRuntimeError::new_err("Client is not connected")
            })?;

            http.add_reaction(
                ferricord_model::ChannelId::new(channel_id),
                ferricord_model::MessageId::new(message_id),
                &emoji,
            )
            .await
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Remove own reaction from a message.
    ///
    /// Args:
    ///     channel_id: The ID of the channel the message is in.
    ///     message_id: The ID of the message to remove reaction from.
    ///     emoji: The emoji to remove.
    fn remove_reaction<'py>(
        &self,
        py: Python<'py>,
        channel_id: u64,
        message_id: u64,
        emoji: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let http = self.http.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let http_guard = http.read().await;
            let http = http_guard.as_ref().ok_or_else(|| {
                pyo3::exceptions::PyRuntimeError::new_err("Client is not connected")
            })?;

            http.remove_own_reaction(
                ferricord_model::ChannelId::new(channel_id),
                ferricord_model::MessageId::new(message_id),
                &emoji,
            )
            .await
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Trigger typing indicator in a channel.
    ///
    /// Args:
    ///     channel_id: The ID of the channel to show typing in.
    fn trigger_typing<'py>(&self, py: Python<'py>, channel_id: u64) -> PyResult<Bound<'py, PyAny>> {
        let http = self.http.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let http_guard = http.read().await;
            let http = http_guard.as_ref().ok_or_else(|| {
                pyo3::exceptions::PyRuntimeError::new_err("Client is not connected")
            })?;

            http.trigger_typing(ferricord_model::ChannelId::new(channel_id))
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Get a channel by ID.
    ///
    /// Args:
    ///     channel_id: The ID of the channel to get.
    ///
    /// Returns:
    ///     The channel.
    fn fetch_channel<'py>(&self, py: Python<'py>, channel_id: u64) -> PyResult<Bound<'py, PyAny>> {
        let http = self.http.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let http_guard = http.read().await;
            let http = http_guard.as_ref().ok_or_else(|| {
                pyo3::exceptions::PyRuntimeError::new_err("Client is not connected")
            })?;

            let channel = http
                .get_channel(ferricord_model::ChannelId::new(channel_id))
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Python::with_gil(|py| {
                PyChannel::new(channel)
                    .into_pyobject(py)
                    .map(|o| o.into_any().unbind())
            })
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("{:?}", e)))
        })
    }

    /// Get a user by ID.
    ///
    /// Args:
    ///     user_id: The ID of the user to get.
    ///
    /// Returns:
    ///     The user.
    fn fetch_user<'py>(&self, py: Python<'py>, user_id: u64) -> PyResult<Bound<'py, PyAny>> {
        let http = self.http.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let http_guard = http.read().await;
            let http = http_guard.as_ref().ok_or_else(|| {
                pyo3::exceptions::PyRuntimeError::new_err("Client is not connected")
            })?;

            let user = http
                .get_user(ferricord_model::UserId::new(user_id))
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Python::with_gil(|py| {
                PyUser::new(user)
                    .into_pyobject(py)
                    .map(|o| o.into_any().unbind())
            })
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("{:?}", e)))
        })
    }

    /// Create a DM channel with a user.
    ///
    /// Args:
    ///     user_id: The ID of the user to create a DM with.
    ///
    /// Returns:
    ///     The DM channel.
    fn create_dm<'py>(&self, py: Python<'py>, user_id: u64) -> PyResult<Bound<'py, PyAny>> {
        let http = self.http.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let http_guard = http.read().await;
            let http = http_guard.as_ref().ok_or_else(|| {
                pyo3::exceptions::PyRuntimeError::new_err("Client is not connected")
            })?;

            let channel = http
                .create_dm(ferricord_model::UserId::new(user_id))
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Python::with_gil(|py| {
                PyChannel::new(channel)
                    .into_pyobject(py)
                    .map(|o| o.into_any().unbind())
            })
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("{:?}", e)))
        })
    }

    // ========== Interaction Methods (Phase 2) ==========

    /// Respond to an interaction (slash command, component, modal).
    ///
    /// Args:
    ///     interaction_id: The ID of the interaction.
    ///     interaction_token: The token of the interaction.
    ///     content: The response content.
    ///     ephemeral: Whether the response should be ephemeral (only visible to the user).
    #[pyo3(signature = (interaction_id, interaction_token, content, ephemeral=false))]
    fn respond_to_interaction<'py>(
        &self,
        py: Python<'py>,
        interaction_id: u64,
        interaction_token: String,
        content: String,
        ephemeral: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let http = self.http.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let http_guard = http.read().await;
            let http = http_guard.as_ref().ok_or_else(|| {
                pyo3::exceptions::PyRuntimeError::new_err("Client is not connected")
            })?;

            let mut flags = 0u32;
            if ephemeral {
                flags |= 1 << 6; // EPHEMERAL flag
            }

            let response = ferricord_model::interaction::InteractionResponse {
                kind:
                    ferricord_model::interaction::InteractionResponseType::ChannelMessageWithSource,
                data: Some(ferricord_model::interaction::InteractionCallbackData {
                    content: Some(content),
                    embeds: Vec::new(),
                    components: Vec::new(),
                    flags: if flags > 0 { Some(flags) } else { None },
                    tts: None,
                    allowed_mentions: None,
                    attachments: Vec::new(),
                    choices: Vec::new(),
                    custom_id: None,
                    title: None,
                }),
            };

            http.create_interaction_response(
                &interaction_id.to_string(),
                &interaction_token,
                &response,
            )
            .await
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Defer an interaction response (shows "Bot is thinking...").
    ///
    /// Args:
    ///     interaction_id: The ID of the interaction.
    ///     interaction_token: The token of the interaction.
    ///     ephemeral: Whether the deferred response should be ephemeral.
    #[pyo3(signature = (interaction_id, interaction_token, ephemeral=false))]
    fn defer_interaction<'py>(
        &self,
        py: Python<'py>,
        interaction_id: u64,
        interaction_token: String,
        ephemeral: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let http = self.http.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let http_guard = http.read().await;
            let http = http_guard.as_ref().ok_or_else(|| {
                pyo3::exceptions::PyRuntimeError::new_err("Client is not connected")
            })?;

            let mut flags = 0u32;
            if ephemeral {
                flags |= 1 << 6; // EPHEMERAL flag
            }

            let response = ferricord_model::interaction::InteractionResponse {
                kind: ferricord_model::interaction::InteractionResponseType::DeferredChannelMessageWithSource,
                data: Some(ferricord_model::interaction::InteractionCallbackData {
                    content: None,
                    embeds: Vec::new(),
                    components: Vec::new(),
                    flags: if flags > 0 { Some(flags) } else { None },
                    tts: None,
                    allowed_mentions: None,
                    attachments: Vec::new(),
                    choices: Vec::new(),
                    custom_id: None,
                    title: None,
                }),
            };

            http.create_interaction_response(
                &interaction_id.to_string(),
                &interaction_token,
                &response,
            )
            .await
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Ok(Python::with_gil(|py| py.None()))
        })
    }

    /// Edit the original interaction response (use after deferring).
    ///
    /// Args:
    ///     interaction_token: The token of the interaction.
    ///     content: The new content.
    fn edit_interaction_response<'py>(
        &self,
        py: Python<'py>,
        interaction_token: String,
        content: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let http = self.http.clone();
        let app_id = self.application_id.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let http_guard = http.read().await;
            let http = http_guard.as_ref().ok_or_else(|| {
                pyo3::exceptions::PyRuntimeError::new_err("Client is not connected")
            })?;

            let app_id_guard = app_id.read().await;
            let app_id = app_id_guard.ok_or_else(|| {
                pyo3::exceptions::PyRuntimeError::new_err("Application ID not set")
            })?;

            let message = http
                .edit_original_interaction_response(
                    ferricord_model::UserId::new(app_id),
                    &interaction_token,
                    Some(content),
                    None,
                    None,
                )
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Python::with_gil(|py| {
                PyMessage::new(message)
                    .into_pyobject(py)
                    .map(|o| o.into_any().unbind())
            })
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("{:?}", e)))
        })
    }

    /// Register a slash command decorator.
    ///
    /// Example:
    ///     ```python
    ///     @client.slash_command(name="ping", description="Pong!")
    ///     async def ping(interaction):
    ///         await interaction.respond("Pong!")
    ///     ```
    #[pyo3(signature = (name, description, guild_id=None))]
    fn slash_command(
        &self,
        py: Python<'_>,
        name: String,
        description: String,
        guild_id: Option<u64>,
    ) -> PyResult<PyObject> {
        let handlers = self.slash_command_handlers.clone();
        let name_clone = name.clone();

        // Store command metadata for registration on ready
        let _description = description;
        let _guild_id = guild_id;

        // Return a decorator function
        let decorator = pyo3::types::PyCFunction::new_closure(
            py,
            Some(c"slash_command_decorator"),
            None,
            move |args: &pyo3::Bound<'_, pyo3::types::PyTuple>,
                  _kwargs: Option<&pyo3::Bound<'_, pyo3::types::PyDict>>|
                  -> PyResult<PyObject> {
                let py = args.py();
                let func = args.get_item(0)?;
                let func_obj = func.unbind();

                let handlers_clone = handlers.clone();
                let name_for_insert = name_clone.clone();

                // Register the handler
                if let Ok(mut guard) = handlers_clone.try_write() {
                    guard.insert(name_for_insert, func_obj.clone_ref(py));
                } else {
                    let handlers_spawn = handlers_clone.clone();
                    let func_spawn = func_obj.clone_ref(py);
                    pyo3_async_runtimes::tokio::get_runtime().spawn(async move {
                        handlers_spawn
                            .write()
                            .await
                            .insert(name_for_insert, func_spawn);
                    });
                }

                Ok(func_obj)
            },
        )?;

        Ok(decorator.into())
    }

    /// Register a component handler (button, select menu).
    ///
    /// Example:
    ///     ```python
    ///     @client.component(custom_id="my_button")
    ///     async def handle_button(interaction):
    ///         await interaction.respond("Button clicked!")
    ///     ```
    fn component(&self, py: Python<'_>, custom_id: String) -> PyResult<PyObject> {
        let handlers = self.component_handlers.clone();
        let custom_id_clone = custom_id.clone();

        let decorator = pyo3::types::PyCFunction::new_closure(
            py,
            Some(c"component_decorator"),
            None,
            move |args: &pyo3::Bound<'_, pyo3::types::PyTuple>,
                  _kwargs: Option<&pyo3::Bound<'_, pyo3::types::PyDict>>|
                  -> PyResult<PyObject> {
                let py = args.py();
                let func = args.get_item(0)?;
                let func_obj = func.unbind();

                let handlers_clone = handlers.clone();
                let id_for_insert = custom_id_clone.clone();

                if let Ok(mut guard) = handlers_clone.try_write() {
                    guard.insert(id_for_insert, func_obj.clone_ref(py));
                } else {
                    let handlers_spawn = handlers_clone.clone();
                    let func_spawn = func_obj.clone_ref(py);
                    pyo3_async_runtimes::tokio::get_runtime().spawn(async move {
                        handlers_spawn
                            .write()
                            .await
                            .insert(id_for_insert, func_spawn);
                    });
                }

                Ok(func_obj)
            },
        )?;

        Ok(decorator.into())
    }

    /// Register a modal handler.
    ///
    /// Example:
    ///     ```python
    ///     @client.modal(custom_id="my_modal")
    ///     async def handle_modal(interaction):
    ///         await interaction.respond("Modal submitted!")
    ///     ```
    fn modal(&self, py: Python<'_>, custom_id: String) -> PyResult<PyObject> {
        let handlers = self.modal_handlers.clone();
        let custom_id_clone = custom_id.clone();

        let decorator = pyo3::types::PyCFunction::new_closure(
            py,
            Some(c"modal_decorator"),
            None,
            move |args: &pyo3::Bound<'_, pyo3::types::PyTuple>,
                  _kwargs: Option<&pyo3::Bound<'_, pyo3::types::PyDict>>|
                  -> PyResult<PyObject> {
                let py = args.py();
                let func = args.get_item(0)?;
                let func_obj = func.unbind();

                let handlers_clone = handlers.clone();
                let id_for_insert = custom_id_clone.clone();

                if let Ok(mut guard) = handlers_clone.try_write() {
                    guard.insert(id_for_insert, func_obj.clone_ref(py));
                } else {
                    let handlers_spawn = handlers_clone.clone();
                    let func_spawn = func_obj.clone_ref(py);
                    pyo3_async_runtimes::tokio::get_runtime().spawn(async move {
                        handlers_spawn
                            .write()
                            .await
                            .insert(id_for_insert, func_spawn);
                    });
                }

                Ok(func_obj)
            },
        )?;

        Ok(decorator.into())
    }

    /// Get the application ID.
    #[getter]
    fn application_id_getter(&self) -> Option<u64> {
        pyo3_async_runtimes::tokio::get_runtime()
            .block_on(async { *self.application_id.read().await })
    }

    // ========== Cogs System (Phase 2) ==========

    /// Load a cog (extension) into the client.
    ///
    /// A cog is a Python class that contains commands and event listeners.
    /// The cog class should have methods decorated with @client.event or
    /// @client.slash_command that will be registered when the cog is loaded.
    ///
    /// Example:
    ///     ```python
    ///     class MyCog:
    ///         def __init__(self, client):
    ///             self.client = client
    ///
    ///         async def on_ready(self):
    ///             print("Cog ready!")
    ///
    ///     client.load_cog(MyCog(client))
    ///     ```
    fn load_cog(&self, py: Python<'_>, cog: PyObject) -> PyResult<()> {
        let handlers = self.event_handlers.clone();

        // Get all methods from the cog
        let cog_type = cog.bind(py).get_type();
        let dir_result = cog_type.dir();

        for attr_name in dir_result.iter() {
            let attr_name_str: String = attr_name.extract()?;

            // Skip private/magic methods
            if attr_name_str.starts_with('_') {
                continue;
            }

            // Check if it's an event handler (starts with "on_")
            if attr_name_str.starts_with("on_") {
                if let Ok(method) = cog.getattr(py, attr_name_str.as_str()) {
                    // Check if it's callable
                    if method.bind(py).is_callable() {
                        let event_name = attr_name_str.strip_prefix("on_").unwrap().to_string();

                        if let Ok(mut guard) = handlers.try_write() {
                            guard.insert(event_name, method);
                        } else {
                            let handlers_clone = handlers.clone();
                            let method_clone = method.clone_ref(py);
                            pyo3_async_runtimes::tokio::get_runtime().spawn(async move {
                                handlers_clone
                                    .write()
                                    .await
                                    .insert(event_name, method_clone);
                            });
                        }
                    }
                }
            }
        }

        info!("Loaded cog: {:?}", cog_type.name());
        Ok(())
    }

    /// Unload a cog from the client.
    ///
    /// This removes all event handlers and commands registered by the cog.
    fn unload_cog(&self, py: Python<'_>, cog: PyObject) -> PyResult<()> {
        let handlers = self.event_handlers.clone();

        // Get all methods from the cog
        let cog_type = cog.bind(py).get_type();
        let dir_result = cog_type.dir();

        for attr_name in dir_result.iter() {
            let attr_name_str: String = attr_name.extract()?;

            // Check if it's an event handler (starts with "on_")
            if attr_name_str.starts_with("on_") {
                let event_name = attr_name_str.strip_prefix("on_").unwrap().to_string();

                if let Ok(mut guard) = handlers.try_write() {
                    guard.remove(&event_name);
                } else {
                    let handlers_clone = handlers.clone();
                    pyo3_async_runtimes::tokio::get_runtime().spawn(async move {
                        handlers_clone.write().await.remove(&event_name);
                    });
                }
            }
        }

        info!("Unloaded cog: {:?}", cog_type.name());
        Ok(())
    }

    // ========== Metrics/Monitoring (Phase 2) ==========

    /// Get client metrics and statistics.
    ///
    /// Returns a dictionary with various metrics about the client's operation.
    /// Uses try_read() to avoid blocking, returns 0 if lock is unavailable.
    fn get_metrics(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new(py);

        // Cache stats (synchronous, always available)
        let stats = self.cache.stats();
        dict.set_item("guilds", stats.guilds)?;
        dict.set_item("channels", stats.channels)?;
        dict.set_item("users", stats.users)?;
        dict.set_item("messages", stats.messages)?;
        dict.set_item("members", stats.members)?;

        // Event handler count (non-blocking)
        let handler_count = self
            .event_handlers
            .try_read()
            .map(|guard| guard.len())
            .unwrap_or(0);
        dict.set_item("event_handlers", handler_count)?;

        // Slash command handler count (non-blocking)
        let slash_count = self
            .slash_command_handlers
            .try_read()
            .map(|guard| guard.len())
            .unwrap_or(0);
        dict.set_item("slash_commands", slash_count)?;

        // Component handler count (non-blocking)
        let component_count = self
            .component_handlers
            .try_read()
            .map(|guard| guard.len())
            .unwrap_or(0);
        dict.set_item("component_handlers", component_count)?;

        // Modal handler count (non-blocking)
        let modal_count = self
            .modal_handlers
            .try_read()
            .map(|guard| guard.len())
            .unwrap_or(0);
        dict.set_item("modal_handlers", modal_count)?;

        // Running status (non-blocking)
        let running = self.running.try_read().map(|guard| *guard).unwrap_or(false);
        dict.set_item("running", running)?;

        Ok(dict.into())
    }

    fn __repr__(&self) -> String {
        format!("<Client intents={}>", self.intents.inner().bits())
    }
}

impl Client {
    #[allow(deprecated)]
    async fn handle_event(
        handlers: &Arc<RwLock<HashMap<String, PyObject>>>,
        cache: &Arc<Cache>,
        event: GatewayEvent,
    ) {
        let (event_name, args): (&str, Vec<PyObject>) = Python::with_gil(|py| match &event {
            GatewayEvent::Ready(ready) => {
                cache.set_current_user(ready.user.clone());
                ("ready", vec![])
            }
            GatewayEvent::Resumed => ("resumed", vec![]),
            GatewayEvent::MessageCreate(message) => {
                cache.insert_message((**message).clone());
                let py_message = PyMessage::new((**message).clone());
                ("message", vec![py_message.into_py(py)])
            }
            GatewayEvent::GuildCreate(guild) => {
                cache.insert_guild((**guild).clone());
                let py_guild = PyGuild::new((**guild).clone());
                ("guild_join", vec![py_guild.into_py(py)])
            }
            GatewayEvent::GuildUpdate(guild) => {
                cache.insert_guild((**guild).clone());
                let py_guild = PyGuild::new((**guild).clone());
                ("guild_update", vec![py_guild.into_py(py)])
            }
            GatewayEvent::GuildDelete(unavailable) => {
                cache.remove_guild(unavailable.id);
                ("guild_remove", vec![unavailable.id.get().into_py(py)])
            }
            GatewayEvent::ChannelCreate(channel) => {
                cache.insert_channel((**channel).clone());
                let py_channel = PyChannel::new((**channel).clone());
                ("channel_create", vec![py_channel.into_py(py)])
            }
            GatewayEvent::ChannelUpdate(channel) => {
                cache.insert_channel((**channel).clone());
                let py_channel = PyChannel::new((**channel).clone());
                ("channel_update", vec![py_channel.into_py(py)])
            }
            GatewayEvent::ChannelDelete(channel) => {
                cache.remove_channel(channel.id);
                let py_channel = PyChannel::new((**channel).clone());
                ("channel_delete", vec![py_channel.into_py(py)])
            }
            GatewayEvent::GuildMemberAdd(event) => {
                cache.insert_member(event.guild_id, event.member.clone());
                ("member_join", vec![event.guild_id.get().into_py(py)])
            }
            GatewayEvent::GuildMemberRemove(event) => {
                cache.remove_member(event.guild_id, event.user.id);
                let py_user = PyUser::new(event.user.clone());
                (
                    "member_remove",
                    vec![event.guild_id.get().into_py(py), py_user.into_py(py)],
                )
            }
            GatewayEvent::TypingStart(event) => (
                "typing",
                vec![
                    event.channel_id.get().into_py(py),
                    event.user_id.get().into_py(py),
                ],
            ),
            _ => ("unknown", vec![]),
        });

        if event_name == "unknown" {
            return;
        }

        let handlers_guard = handlers.read().await;
        if let Some(handler) = handlers_guard.get(event_name) {
            Python::with_gil(|py| {
                let result = if args.is_empty() {
                    handler.call0(py)
                } else {
                    match PyTuple::new(py, &args) {
                        Ok(tuple) => handler.call1(py, tuple),
                        Err(e) => {
                            error!("Failed to create PyTuple for event handler: {:?}", e);
                            return;
                        }
                    }
                };

                match result {
                    Ok(coro) => {
                        // Try to convert to a Rust future first (works in async context)
                        match pyo3_async_runtimes::tokio::into_future(coro.bind(py).clone()) {
                            Ok(future) => {
                                tokio::spawn(async move {
                                    if let Err(e) = future.await {
                                        error!("Event handler error: {:?}", e);
                                    }
                                });
                            }
                            Err(_) => {
                                // Fallback: use asyncio.run() for blocking context
                                // This handles the case where there's no Python event loop
                                if let Ok(asyncio) = py.import("asyncio") {
                                    if let Err(e) = asyncio.call_method1("run", (coro.bind(py),)) {
                                        error!("Event handler error: {:?}", e);
                                    }
                                } else {
                                    error!("Failed to import asyncio for event handler");
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to call event handler: {:?}", e);
                    }
                }
            });
        }
    }
}

/// An auto-sharded client that automatically manages multiple shards.
///
/// This client automatically determines the number of shards needed based on
/// Discord's recommendations and manages them all transparently.
///
/// Example:
///     ```python
///     from ferricord import AutoShardedClient, Intents
///
///     intents = Intents.default()
///     client = AutoShardedClient(intents=intents)
///
///     @client.event
///     async def on_ready():
///         print(f"Logged in with {client.shard_count} shards")
///
///     client.run("YOUR_BOT_TOKEN")
///     ```
#[pyclass]
pub struct AutoShardedClient {
    /// Gateway intents.
    intents: Intents,
    /// Event handlers.
    event_handlers: Arc<RwLock<HashMap<String, PyObject>>>,
    /// The HTTP client (reused across runs).
    http: Arc<RwLock<Option<Arc<HttpClient>>>>,
    /// The cache (shared across all shards).
    cache: Arc<Cache>,
    /// Whether the client is running.
    running: Arc<RwLock<bool>>,
    /// Number of shards (set after connecting).
    shard_count: Arc<RwLock<u32>>,
    /// Shard IDs that are currently connected.
    connected_shards: Arc<RwLock<Vec<u32>>>,
}

#[pymethods]
impl AutoShardedClient {
    /// Create a new AutoShardedClient.
    ///
    /// Args:
    ///     intents: Gateway intents to use.
    #[new]
    #[pyo3(signature = (intents=None))]
    fn new(intents: Option<Intents>) -> Self {
        let intents = intents.unwrap_or_else(Intents::default_intents);

        Self {
            intents,
            event_handlers: Arc::new(RwLock::new(HashMap::new())),
            http: Arc::new(RwLock::new(None)),
            cache: Arc::new(Cache::new()),
            running: Arc::new(RwLock::new(false)),
            shard_count: Arc::new(RwLock::new(0)),
            connected_shards: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Register an event handler.
    fn event(&self, py: Python<'_>, func: PyObject) -> PyResult<PyObject> {
        let func_name = func.getattr(py, "__name__")?.extract::<String>(py)?;

        let event_name = if let Some(stripped) = func_name.strip_prefix("on_") {
            stripped.to_string()
        } else {
            func_name.clone()
        };

        let handlers = self.event_handlers.clone();
        let func_clone = func.clone_ref(py);

        if let Ok(mut guard) = handlers.try_write() {
            guard.insert(event_name, func_clone);
        } else {
            let handlers_clone = handlers.clone();
            pyo3_async_runtimes::tokio::get_runtime().spawn(async move {
                handlers_clone.write().await.insert(event_name, func_clone);
            });
        }

        Ok(func)
    }

    /// Run the auto-sharded client with the given token.
    ///
    /// This automatically determines the number of shards needed and manages them.
    fn run(&self, py: Python<'_>, token: String) -> PyResult<()> {
        let intents = self.intents.inner();
        let event_handlers = self.event_handlers.clone();
        let cache = self.cache.clone();
        let running = self.running.clone();
        let http_client = self.http.clone();
        let shard_count_holder = self.shard_count.clone();
        let connected_shards = self.connected_shards.clone();
        let error_holder: Arc<RwLock<Option<String>>> = Arc::new(RwLock::new(None));
        let error_holder_clone = error_holder.clone();

        py.allow_threads(|| {
            pyo3_async_runtimes::tokio::get_runtime().block_on(async move {
                *running.write().await = true;

                let http = match HttpClient::new(&token) {
                    Ok(http) => Arc::new(http),
                    Err(e) => {
                        error!("Failed to create HTTP client: {}", e);
                        *error_holder_clone.write().await =
                            Some(format!("Failed to create HTTP client: {}", e));
                        return;
                    }
                };

                // Store HTTP client for REST API operations
                *http_client.write().await = Some(http.clone());

                let gateway_info = match http.get_gateway_bot().await {
                    Ok(info) => info,
                    Err(e) => {
                        error!("Failed to get gateway info: {}", e);
                        *error_holder_clone.write().await =
                            Some(format!("Failed to get gateway info: {}", e));
                        return;
                    }
                };

                let num_shards = gateway_info.shards;
                *shard_count_holder.write().await = num_shards;

                info!(
                    "Gateway URL: {}, Recommended shards: {}",
                    gateway_info.url, num_shards
                );

                let (event_tx, mut event_rx) = mpsc::unbounded_channel::<GatewayEvent>();

                // Spawn all shards
                let mut shard_handles = Vec::new();
                for shard_id in 0..num_shards {
                    let token_clone = token.clone();
                    let gateway_url = gateway_info.url.clone();
                    let event_tx_clone = event_tx.clone();
                    let shard_running = running.clone();
                    let shard_error_holder = error_holder_clone.clone();
                    let connected_shards_clone = connected_shards.clone();

                    let config =
                        ShardConfig::new(token_clone, intents).with_sharding(shard_id, num_shards);
                    let mut shard = Shard::new(config);

                    let handle = tokio::spawn(async move {
                        // Mark shard as connected
                        connected_shards_clone.write().await.push(shard_id);

                        match shard.run(&gateway_url, event_tx_clone).await {
                            Ok(()) => {
                                info!("Shard {} disconnected normally", shard_id);
                            }
                            Err(e) => {
                                error!("Shard {} error: {}", shard_id, e);
                                *shard_error_holder.write().await =
                                    Some(format!("Shard {} error: {}", shard_id, e));
                            }
                        }

                        // Remove shard from connected list
                        let mut shards = connected_shards_clone.write().await;
                        shards.retain(|&id| id != shard_id);

                        // Signal stop if all shards disconnected
                        if shards.is_empty() {
                            *shard_running.write().await = false;
                        }
                    });

                    shard_handles.push(handle);

                    // Stagger shard connections (Discord recommends 5 seconds between)
                    if shard_id < num_shards - 1 {
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    }
                }

                let mut got_sigint = false;

                while *running.read().await {
                    tokio::select! {
                        _ = tokio::signal::ctrl_c() => {
                            info!("Received Ctrl+C, shutting down...");
                            got_sigint = true;
                            break;
                        }
                        Some(event) = event_rx.recv() => {
                            Client::handle_event(&event_handlers, &cache, event).await;
                        }
                    }
                }

                // Clean up all shards
                for handle in shard_handles {
                    if !handle.is_finished() {
                        handle.abort();
                    }
                }

                if got_sigint {
                    *error_holder_clone.write().await = Some("KeyboardInterrupt".to_string());
                }
            });
        });

        if let Some(err) = pyo3_async_runtimes::tokio::get_runtime()
            .block_on(async { error_holder.read().await.clone() })
        {
            if err == "KeyboardInterrupt" {
                return Err(pyo3::exceptions::PyKeyboardInterrupt::new_err(
                    "Received Ctrl+C",
                ));
            }
            return Err(pyo3::exceptions::PyRuntimeError::new_err(err));
        }

        Ok(())
    }

    /// Get the current user.
    #[getter]
    fn user(&self) -> Option<PyUser> {
        self.cache.current_user().map(|u| PyUser::new(u.into()))
    }

    /// Get all guilds the bot is in.
    #[getter]
    fn guilds(&self) -> Vec<PyGuild> {
        self.cache
            .guilds()
            .into_iter()
            .map(|g| PyGuild::new((*g).clone()))
            .collect()
    }

    /// Get the number of shards.
    #[getter]
    fn shard_count(&self) -> u32 {
        pyo3_async_runtimes::tokio::get_runtime().block_on(async { *self.shard_count.read().await })
    }

    /// Get the IDs of connected shards.
    #[getter]
    fn connected_shards(&self) -> Vec<u32> {
        pyo3_async_runtimes::tokio::get_runtime()
            .block_on(async { self.connected_shards.read().await.clone() })
    }

    /// Get the number of guilds.
    #[getter]
    fn guild_count(&self) -> usize {
        self.cache.guild_count()
    }

    /// Get cache statistics.
    fn cache_stats(&self) -> String {
        self.cache.stats().to_string()
    }

    /// Get client metrics and statistics.
    /// Uses try_read() to avoid blocking, returns 0 if lock is unavailable.
    fn get_metrics(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new(py);

        // Cache stats (synchronous, always available)
        let stats = self.cache.stats();
        dict.set_item("guilds", stats.guilds)?;
        dict.set_item("channels", stats.channels)?;
        dict.set_item("users", stats.users)?;
        dict.set_item("messages", stats.messages)?;
        dict.set_item("members", stats.members)?;

        // Shard info (non-blocking)
        let shard_count = self.shard_count.try_read().map(|guard| *guard).unwrap_or(0);
        dict.set_item("shard_count", shard_count)?;

        let connected = self
            .connected_shards
            .try_read()
            .map(|guard| guard.len())
            .unwrap_or(0);
        dict.set_item("connected_shards", connected)?;

        // Running status (non-blocking)
        let running = self.running.try_read().map(|guard| *guard).unwrap_or(false);
        dict.set_item("running", running)?;

        Ok(dict.into())
    }

    fn __repr__(&self) -> String {
        let shard_count = self.shard_count.try_read().map(|guard| *guard).unwrap_or(0);
        format!(
            "<AutoShardedClient intents={} shards={}>",
            self.intents.inner().bits(),
            shard_count
        )
    }
}
