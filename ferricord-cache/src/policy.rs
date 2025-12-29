//! Cache policy definitions
//!
//! This module defines different caching policies for controlling
//! what data is cached and how much memory is used.

use ferricord_model::{ChannelId, GuildId, UserId};

/// Kind of cache policy.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CachePolicyKind {
    /// Cache nothing except the current user.
    None,
    /// Cache only recently active guilds and channels.
    Partial,
    /// Cache everything.
    #[default]
    Full,
    /// Custom caching rules.
    Custom,
}

/// Trait for defining cache policies.
pub trait CachePolicy: Send + Sync {
    /// Returns the kind of cache policy.
    fn kind(&self) -> CachePolicyKind;

    /// Whether to cache a guild.
    fn should_cache_guild(&self, guild_id: GuildId) -> bool;

    /// Whether to cache a channel.
    fn should_cache_channel(&self, channel_id: ChannelId) -> bool;

    /// Whether to cache a member.
    fn should_cache_member(&self, guild_id: GuildId, user_id: UserId) -> bool;

    /// Whether to cache a message.
    fn should_cache_message(&self, channel_id: ChannelId) -> bool;

    /// Maximum number of messages to cache per channel.
    fn max_messages_per_channel(&self) -> usize;

    /// Maximum number of members to cache per guild.
    fn max_members_per_guild(&self) -> Option<usize>;
}

/// No caching policy - only caches the current user.
#[derive(Clone, Debug, Default)]
pub struct NonePolicy;

impl CachePolicy for NonePolicy {
    fn kind(&self) -> CachePolicyKind {
        CachePolicyKind::None
    }

    fn should_cache_guild(&self, _guild_id: GuildId) -> bool {
        false
    }

    fn should_cache_channel(&self, _channel_id: ChannelId) -> bool {
        false
    }

    fn should_cache_member(&self, _guild_id: GuildId, _user_id: UserId) -> bool {
        false
    }

    fn should_cache_message(&self, _channel_id: ChannelId) -> bool {
        false
    }

    fn max_messages_per_channel(&self) -> usize {
        0
    }

    fn max_members_per_guild(&self) -> Option<usize> {
        Some(0)
    }
}

/// Partial caching policy - caches recently active data.
#[derive(Clone, Debug)]
pub struct PartialPolicy {
    /// Maximum messages per channel.
    pub max_messages: usize,
    /// Maximum members per guild.
    pub max_members: usize,
}

impl Default for PartialPolicy {
    fn default() -> Self {
        Self {
            max_messages: 100,
            max_members: 1000,
        }
    }
}

impl CachePolicy for PartialPolicy {
    fn kind(&self) -> CachePolicyKind {
        CachePolicyKind::Partial
    }

    fn should_cache_guild(&self, _guild_id: GuildId) -> bool {
        true
    }

    fn should_cache_channel(&self, _channel_id: ChannelId) -> bool {
        true
    }

    fn should_cache_member(&self, _guild_id: GuildId, _user_id: UserId) -> bool {
        true
    }

    fn should_cache_message(&self, _channel_id: ChannelId) -> bool {
        true
    }

    fn max_messages_per_channel(&self) -> usize {
        self.max_messages
    }

    fn max_members_per_guild(&self) -> Option<usize> {
        Some(self.max_members)
    }
}

/// Full caching policy - caches everything.
#[derive(Clone, Debug)]
pub struct FullPolicy {
    /// Maximum messages per channel.
    pub max_messages: usize,
}

impl Default for FullPolicy {
    fn default() -> Self {
        Self {
            max_messages: 1000,
        }
    }
}

impl CachePolicy for FullPolicy {
    fn kind(&self) -> CachePolicyKind {
        CachePolicyKind::Full
    }

    fn should_cache_guild(&self, _guild_id: GuildId) -> bool {
        true
    }

    fn should_cache_channel(&self, _channel_id: ChannelId) -> bool {
        true
    }

    fn should_cache_member(&self, _guild_id: GuildId, _user_id: UserId) -> bool {
        true
    }

    fn should_cache_message(&self, _channel_id: ChannelId) -> bool {
        true
    }

    fn max_messages_per_channel(&self) -> usize {
        self.max_messages
    }

    fn max_members_per_guild(&self) -> Option<usize> {
        None
    }
}

/// Builder for creating custom cache policies.
#[derive(Clone, Debug)]
pub struct CustomPolicyBuilder {
    cache_guilds: bool,
    cache_channels: bool,
    cache_members: bool,
    cache_messages: bool,
    max_messages: usize,
    max_members: Option<usize>,
}

impl Default for CustomPolicyBuilder {
    fn default() -> Self {
        Self {
            cache_guilds: true,
            cache_channels: true,
            cache_members: true,
            cache_messages: true,
            max_messages: 1000,
            max_members: None,
        }
    }
}

impl CustomPolicyBuilder {
    /// Create a new custom policy builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set whether to cache guilds.
    pub fn cache_guilds(mut self, cache: bool) -> Self {
        self.cache_guilds = cache;
        self
    }

    /// Set whether to cache channels.
    pub fn cache_channels(mut self, cache: bool) -> Self {
        self.cache_channels = cache;
        self
    }

    /// Set whether to cache members.
    pub fn cache_members(mut self, cache: bool) -> Self {
        self.cache_members = cache;
        self
    }

    /// Set whether to cache messages.
    pub fn cache_messages(mut self, cache: bool) -> Self {
        self.cache_messages = cache;
        self
    }

    /// Set the maximum messages per channel.
    pub fn max_messages(mut self, max: usize) -> Self {
        self.max_messages = max;
        self
    }

    /// Set the maximum members per guild.
    pub fn max_members(mut self, max: Option<usize>) -> Self {
        self.max_members = max;
        self
    }

    /// Build the custom policy.
    pub fn build(self) -> CustomPolicy {
        CustomPolicy {
            cache_guilds: self.cache_guilds,
            cache_channels: self.cache_channels,
            cache_members: self.cache_members,
            cache_messages: self.cache_messages,
            max_messages: self.max_messages,
            max_members: self.max_members,
        }
    }
}

/// Custom caching policy with user-defined rules.
#[derive(Clone, Debug)]
pub struct CustomPolicy {
    cache_guilds: bool,
    cache_channels: bool,
    cache_members: bool,
    cache_messages: bool,
    max_messages: usize,
    max_members: Option<usize>,
}

impl CachePolicy for CustomPolicy {
    fn kind(&self) -> CachePolicyKind {
        CachePolicyKind::Custom
    }

    fn should_cache_guild(&self, _guild_id: GuildId) -> bool {
        self.cache_guilds
    }

    fn should_cache_channel(&self, _channel_id: ChannelId) -> bool {
        self.cache_channels
    }

    fn should_cache_member(&self, _guild_id: GuildId, _user_id: UserId) -> bool {
        self.cache_members
    }

    fn should_cache_message(&self, _channel_id: ChannelId) -> bool {
        self.cache_messages
    }

    fn max_messages_per_channel(&self) -> usize {
        self.max_messages
    }

    fn max_members_per_guild(&self) -> Option<usize> {
        self.max_members
    }
}
