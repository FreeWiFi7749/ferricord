//! Ferricord Python Bindings
//!
//! This crate provides Python bindings for Ferricord using PyO3.

mod client;
mod intents;
mod models;
mod types;

use std::sync::Once;

use pyo3::prelude::*;
use pyo3::types::PyModule;
use tracing_subscriber::EnvFilter;

static INIT_LOGGING: Once = Once::new();

/// Initialize the tracing subscriber for logging.
/// This is called once when the module is first imported.
fn init_logging() {
    INIT_LOGGING.call_once(|| {
        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));

        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_target(true)
            .with_thread_ids(false)
            .with_file(false)
            .with_line_number(false)
            .init();
    });
}

/// Ferricord - A high-performance Discord API wrapper for Python.
///
/// This module provides Python bindings for the Ferricord library,
/// offering a discord.py-compatible API with Rust performance.
#[pymodule]
fn ferricord(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Initialize logging on module import
    init_logging();

    m.add_class::<client::Client>()?;
    m.add_class::<client::AutoShardedClient>()?;
    m.add_class::<intents::Intents>()?;
    m.add_class::<models::PyUser>()?;
    m.add_class::<models::PyMessage>()?;
    m.add_class::<models::PyGuild>()?;
    m.add_class::<models::PyChannel>()?;
    m.add_class::<models::PyMember>()?;
    m.add_class::<models::PyRole>()?;
    m.add_class::<models::PyInteraction>()?;
    m.add_class::<models::PyInteractionData>()?;
    m.add_class::<models::PyInteractionOption>()?;
    m.add_class::<models::PyInteractionResponse>()?;

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
