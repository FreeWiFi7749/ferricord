//! Cache storage implementation
//!
//! This module provides the main cache storage for Discord data.

use std::num::NonZeroUsize;
use std::sync::Arc;

use dashmap::DashMap;
use lru::LruCache;
use parking_lot::RwLock;
use tracing::debug;

use ferricord_model::{
    Channel, ChannelId, CurrentUser, Guild, GuildId, Member, Message, MessageId, Role, RoleId,
    User, UserId,
};

use crate::policy::{CachePolicy, FullPolicy};

/// The main cache for Discord data.
pub struct Cache {
    /// Cache policy.
    policy: Arc<dyn CachePolicy>,
    /// Current authenticated user.
    current_user: RwLock<Option<CurrentUser>>,
    /// Cached guilds.
    guilds: DashMap<GuildId, Arc<Guild>>,
    /// Cached channels.
    channels: DashMap<ChannelId, Arc<Channel>>,
    /// Cached users.
    users: DashMap<UserId, Arc<User>>,
    /// Cached members (guild_id -> user_id -> member).
    members: DashMap<GuildId, DashMap<UserId, Arc<Member>>>,
    /// Cached roles (guild_id -> role_id -> role).
    roles: DashMap<GuildId, DashMap<RoleId, Arc<Role>>>,
    /// Cached messages (channel_id -> LRU cache of messages).
    messages: DashMap<ChannelId, RwLock<LruCache<MessageId, Arc<Message>>>>,
    /// Guild ID to channel IDs mapping.
    guild_channels: DashMap<GuildId, Vec<ChannelId>>,
    /// User ID to DM channel ID mapping.
    dm_channels: DashMap<UserId, ChannelId>,
}

impl Cache {
    /// Create a new cache with the default policy.
    pub fn new() -> Self {
        Self::with_policy(Arc::new(FullPolicy::default()))
    }

    /// Create a new cache with a custom policy.
    pub fn with_policy(policy: Arc<dyn CachePolicy>) -> Self {
        Self {
            policy,
            current_user: RwLock::new(None),
            guilds: DashMap::new(),
            channels: DashMap::new(),
            users: DashMap::new(),
            members: DashMap::new(),
            roles: DashMap::new(),
            messages: DashMap::new(),
            guild_channels: DashMap::new(),
            dm_channels: DashMap::new(),
        }
    }

    // ========== Current User ==========

    /// Get the current user.
    pub fn current_user(&self) -> Option<CurrentUser> {
        self.current_user.read().clone()
    }

    /// Set the current user.
    pub fn set_current_user(&self, user: CurrentUser) {
        *self.current_user.write() = Some(user);
    }

    // ========== Guilds ==========

    /// Get a guild by ID.
    pub fn guild(&self, guild_id: GuildId) -> Option<Arc<Guild>> {
        self.guilds.get(&guild_id).map(|g| Arc::clone(&g))
    }

    /// Get all cached guilds.
    pub fn guilds(&self) -> Vec<Arc<Guild>> {
        self.guilds.iter().map(|g| Arc::clone(&g)).collect()
    }

    /// Get the number of cached guilds.
    pub fn guild_count(&self) -> usize {
        self.guilds.len()
    }

    /// Insert or update a guild.
    pub fn insert_guild(&self, guild: Guild) {
        if !self.policy.should_cache_guild(guild.id) {
            return;
        }

        let guild_id = guild.id;
        debug!("Caching guild: {} ({})", guild.name, guild_id);

        let channel_ids: Vec<ChannelId> = Vec::new();
        self.guild_channels.insert(guild_id, channel_ids);

        for role in &guild.roles {
            self.insert_role(guild_id, role.clone());
        }

        self.guilds.insert(guild_id, Arc::new(guild));
    }

    /// Remove a guild from the cache.
    pub fn remove_guild(&self, guild_id: GuildId) {
        debug!("Removing guild from cache: {}", guild_id);
        self.guilds.remove(&guild_id);
        self.members.remove(&guild_id);
        self.roles.remove(&guild_id);

        if let Some((_, channel_ids)) = self.guild_channels.remove(&guild_id) {
            for channel_id in channel_ids {
                self.channels.remove(&channel_id);
                self.messages.remove(&channel_id);
            }
        }
    }

    // ========== Channels ==========

    /// Get a channel by ID.
    pub fn channel(&self, channel_id: ChannelId) -> Option<Arc<Channel>> {
        self.channels.get(&channel_id).map(|c| Arc::clone(&c))
    }

    /// Get all channels in a guild.
    pub fn guild_channels(&self, guild_id: GuildId) -> Vec<Arc<Channel>> {
        self.guild_channels
            .get(&guild_id)
            .map(|ids| ids.iter().filter_map(|id| self.channel(*id)).collect())
            .unwrap_or_default()
    }

    /// Insert or update a channel.
    pub fn insert_channel(&self, channel: Channel) {
        if !self.policy.should_cache_channel(channel.id) {
            return;
        }

        let channel_id = channel.id;
        debug!("Caching channel: {:?} ({})", channel.name, channel_id);

        if let Some(guild_id) = channel.guild_id {
            self.guild_channels
                .entry(guild_id)
                .or_insert_with(Vec::new)
                .push(channel_id);
        }

        self.channels.insert(channel_id, Arc::new(channel));
    }

    /// Remove a channel from the cache.
    pub fn remove_channel(&self, channel_id: ChannelId) {
        debug!("Removing channel from cache: {}", channel_id);

        if let Some((_, channel)) = self.channels.remove(&channel_id) {
            if let Some(guild_id) = channel.guild_id {
                if let Some(mut ids) = self.guild_channels.get_mut(&guild_id) {
                    ids.retain(|id| *id != channel_id);
                }
            }
        }

        self.messages.remove(&channel_id);
    }

    // ========== Users ==========

    /// Get a user by ID.
    pub fn user(&self, user_id: UserId) -> Option<Arc<User>> {
        self.users.get(&user_id).map(|u| Arc::clone(&u))
    }

    /// Insert or update a user.
    pub fn insert_user(&self, user: User) {
        let user_id = user.id;
        self.users.insert(user_id, Arc::new(user));
    }

    // ========== Members ==========

    /// Get a member by guild and user ID.
    pub fn member(&self, guild_id: GuildId, user_id: UserId) -> Option<Arc<Member>> {
        self.members
            .get(&guild_id)
            .and_then(|members| members.get(&user_id).map(|m| Arc::clone(&m)))
    }

    /// Get all members in a guild.
    pub fn guild_members(&self, guild_id: GuildId) -> Vec<Arc<Member>> {
        self.members
            .get(&guild_id)
            .map(|members| members.iter().map(|m| Arc::clone(&m)).collect())
            .unwrap_or_default()
    }

    /// Get the number of cached members in a guild.
    pub fn member_count(&self, guild_id: GuildId) -> usize {
        self.members.get(&guild_id).map(|m| m.len()).unwrap_or(0)
    }

    /// Insert or update a member.
    pub fn insert_member(&self, guild_id: GuildId, member: Member) {
        let user_id = member.user.as_ref().map(|u| u.id).unwrap_or_default();

        if !self.policy.should_cache_member(guild_id, user_id) {
            return;
        }

        if let Some(max) = self.policy.max_members_per_guild() {
            if self.member_count(guild_id) >= max {
                return;
            }
        }

        if let Some(ref user) = member.user {
            self.insert_user(user.clone());
        }

        self.members
            .entry(guild_id)
            .or_insert_with(DashMap::new)
            .insert(user_id, Arc::new(member));
    }

    /// Remove a member from the cache.
    pub fn remove_member(&self, guild_id: GuildId, user_id: UserId) {
        if let Some(members) = self.members.get(&guild_id) {
            members.remove(&user_id);
        }
    }

    // ========== Roles ==========

    /// Get a role by guild and role ID.
    pub fn role(&self, guild_id: GuildId, role_id: RoleId) -> Option<Arc<Role>> {
        self.roles
            .get(&guild_id)
            .and_then(|roles| roles.get(&role_id).map(|r| Arc::clone(&r)))
    }

    /// Get all roles in a guild.
    pub fn guild_roles(&self, guild_id: GuildId) -> Vec<Arc<Role>> {
        self.roles
            .get(&guild_id)
            .map(|roles| roles.iter().map(|r| Arc::clone(&r)).collect())
            .unwrap_or_default()
    }

    /// Insert or update a role.
    pub fn insert_role(&self, guild_id: GuildId, role: Role) {
        let role_id = role.id;
        self.roles
            .entry(guild_id)
            .or_insert_with(DashMap::new)
            .insert(role_id, Arc::new(role));
    }

    /// Remove a role from the cache.
    pub fn remove_role(&self, guild_id: GuildId, role_id: RoleId) {
        if let Some(roles) = self.roles.get(&guild_id) {
            roles.remove(&role_id);
        }
    }

    // ========== Messages ==========

    /// Get a message by channel and message ID.
    pub fn message(&self, channel_id: ChannelId, message_id: MessageId) -> Option<Arc<Message>> {
        self.messages
            .get(&channel_id)
            .and_then(|cache| cache.read().peek(&message_id).cloned())
    }

    /// Insert or update a message.
    pub fn insert_message(&self, message: Message) {
        let channel_id = message.channel_id;

        if !self.policy.should_cache_message(channel_id) {
            return;
        }

        let message_id = message.id;
        let max_messages = self.policy.max_messages_per_channel();

        if max_messages == 0 {
            return;
        }

        self.messages
            .entry(channel_id)
            .or_insert_with(|| {
                RwLock::new(LruCache::new(
                    NonZeroUsize::new(max_messages).unwrap_or(NonZeroUsize::new(1).unwrap()),
                ))
            })
            .write()
            .put(message_id, Arc::new(message));
    }

    /// Remove a message from the cache.
    pub fn remove_message(&self, channel_id: ChannelId, message_id: MessageId) {
        if let Some(cache) = self.messages.get(&channel_id) {
            cache.write().pop(&message_id);
        }
    }

    // ========== DM Channels ==========

    /// Get a DM channel ID for a user.
    pub fn dm_channel(&self, user_id: UserId) -> Option<ChannelId> {
        self.dm_channels.get(&user_id).map(|c| *c)
    }

    /// Set a DM channel for a user.
    pub fn set_dm_channel(&self, user_id: UserId, channel_id: ChannelId) {
        self.dm_channels.insert(user_id, channel_id);
    }

    // ========== Statistics ==========

    /// Get cache statistics.
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            guilds: self.guilds.len(),
            channels: self.channels.len(),
            users: self.users.len(),
            members: self.members.iter().map(|m| m.len()).sum(),
            roles: self.roles.iter().map(|r| r.len()).sum(),
            messages: self.messages.iter().map(|m| m.read().len()).sum(),
        }
    }

    /// Clear all cached data.
    pub fn clear(&self) {
        self.guilds.clear();
        self.channels.clear();
        self.users.clear();
        self.members.clear();
        self.roles.clear();
        self.messages.clear();
        self.guild_channels.clear();
        self.dm_channels.clear();
    }
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache statistics.
#[derive(Clone, Debug, Default)]
pub struct CacheStats {
    /// Number of cached guilds.
    pub guilds: usize,
    /// Number of cached channels.
    pub channels: usize,
    /// Number of cached users.
    pub users: usize,
    /// Number of cached members.
    pub members: usize,
    /// Number of cached roles.
    pub roles: usize,
    /// Number of cached messages.
    pub messages: usize,
}

impl std::fmt::Display for CacheStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Guilds: {}, Channels: {}, Users: {}, Members: {}, Roles: {}, Messages: {}",
            self.guilds, self.channels, self.users, self.members, self.roles, self.messages
        )
    }
}
