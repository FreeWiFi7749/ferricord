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
    /// The HTTP client (reused across runs).
    /// TODO: Use this field in Phase 2 for REST API operations (message sending, etc.)
    #[allow(dead_code)]
    http: Arc<RwLock<Option<Arc<HttpClient>>>>,
    /// The cache.
    cache: Arc<Cache>,
    /// Whether the client is running.
    running: Arc<RwLock<bool>>,
    /// Task handles for cleanup on close.
    task_handles: Arc<RwLock<Vec<JoinHandle<()>>>>,
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
            http: Arc::new(RwLock::new(None)),
            cache: Arc::new(Cache::new()),
            running: Arc::new(RwLock::new(false)),
            task_handles: Arc::new(RwLock::new(Vec::new())),
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

                while *running.read().await {
                    tokio::select! {
                        // Handle Ctrl+C (SIGINT)
                        _ = tokio::signal::ctrl_c() => {
                            info!("Received Ctrl+C, shutting down...");
                            got_sigint = true;
                            break;
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

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            *running.write().await = true;

            let http = HttpClient::new(&token)
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            let http = Arc::new(http);

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
