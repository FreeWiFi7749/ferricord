//! Common type definitions for Python bindings

#![allow(dead_code)]

use ferricord_model::{ChannelId, GuildId, MessageId, RoleId, UserId};
use pyo3::prelude::*;

/// Convert a Rust ID to a Python int.
#[allow(deprecated)]
pub fn id_to_py(py: Python<'_>, id: u64) -> PyObject {
    id.into_py(py)
}

/// Convert a Python int to a UserId.
pub fn py_to_user_id(id: u64) -> UserId {
    UserId::new(id)
}

/// Convert a Python int to a GuildId.
pub fn py_to_guild_id(id: u64) -> GuildId {
    GuildId::new(id)
}

/// Convert a Python int to a ChannelId.
pub fn py_to_channel_id(id: u64) -> ChannelId {
    ChannelId::new(id)
}

/// Convert a Python int to a MessageId.
pub fn py_to_message_id(id: u64) -> MessageId {
    MessageId::new(id)
}

/// Convert a Python int to a RoleId.
pub fn py_to_role_id(id: u64) -> RoleId {
    RoleId::new(id)
}
