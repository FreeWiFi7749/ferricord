//! Ferricord Python Bindings
//!
//! This crate provides Python bindings for Ferricord using PyO3.

mod client;
mod intents;
mod models;
mod types;

use pyo3::prelude::*;
use pyo3::types::PyModule;

/// Ferricord - A high-performance Discord API wrapper for Python.
///
/// This module provides Python bindings for the Ferricord library,
/// offering a discord.py-compatible API with Rust performance.
#[pymodule]
fn ferricord(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<client::Client>()?;
    m.add_class::<intents::Intents>()?;
    m.add_class::<models::PyUser>()?;
    m.add_class::<models::PyMessage>()?;
    m.add_class::<models::PyGuild>()?;
    m.add_class::<models::PyChannel>()?;
    m.add_class::<models::PyMember>()?;
    m.add_class::<models::PyRole>()?;

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
