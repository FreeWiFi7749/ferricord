//! User-related models

use crate::id::UserId;
use serde::{Deserialize, Serialize};

/// Represents a Discord user.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    /// The user's ID.
    pub id: UserId,
    /// The user's username, not unique across the platform.
    pub username: String,
    /// The user's Discord-tag (discriminator).
    pub discriminator: String,
    /// The user's display name, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_name: Option<String>,
    /// The user's avatar hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// Whether the user is a bot.
    #[serde(default)]
    pub bot: bool,
    /// Whether the user is an Official Discord System user.
    #[serde(default)]
    pub system: bool,
    /// Whether the user has two factor enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_enabled: Option<bool>,
    /// The user's banner hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    /// The user's banner color encoded as an integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<u32>,
    /// The user's chosen language option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// Whether the email on this account has been verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    /// The user's email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The flags on a user's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,
    /// The type of Nitro subscription on a user's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_type: Option<u8>,
    /// The public flags on a user's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_flags: Option<u64>,
    /// The user's avatar decoration hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_decoration: Option<String>,
}

impl User {
    /// Returns the user's tag (username#discriminator).
    pub fn tag(&self) -> String {
        if self.discriminator == "0" {
            self.username.clone()
        } else {
            format!("{}#{}", self.username, self.discriminator)
        }
    }

    /// Returns the user's display name (global_name if set, otherwise username).
    pub fn display_name(&self) -> &str {
        self.global_name.as_deref().unwrap_or(&self.username)
    }

    /// Returns the URL to the user's avatar.
    pub fn avatar_url(&self) -> Option<String> {
        self.avatar.as_ref().map(|hash| {
            let ext = if hash.starts_with("a_") { "gif" } else { "png" };
            format!(
                "https://cdn.discordapp.com/avatars/{}/{}.{}",
                self.id, hash, ext
            )
        })
    }

    /// Returns the URL to the user's default avatar.
    pub fn default_avatar_url(&self) -> String {
        let index = if self.discriminator == "0" {
            (self.id.get() >> 22) % 6
        } else {
            self.discriminator.parse::<u64>().unwrap_or(0) % 5
        };
        format!("https://cdn.discordapp.com/embed/avatars/{}.png", index)
    }

    /// Returns the URL to the user's avatar, or their default avatar if none is set.
    pub fn display_avatar_url(&self) -> String {
        self.avatar_url()
            .unwrap_or_else(|| self.default_avatar_url())
    }

    /// Returns the user's mention string.
    pub fn mention(&self) -> String {
        format!("<@{}>", self.id)
    }
}

/// Represents the current authenticated user.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CurrentUser {
    /// The user's ID.
    pub id: UserId,
    /// The user's username.
    pub username: String,
    /// The user's discriminator.
    pub discriminator: String,
    /// The user's display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_name: Option<String>,
    /// The user's avatar hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// Whether the user is a bot.
    #[serde(default)]
    pub bot: bool,
    /// Whether the user has two factor enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_enabled: Option<bool>,
    /// The user's banner hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    /// The user's banner color.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<u32>,
    /// The user's locale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// Whether the email is verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    /// The user's email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The user's flags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,
    /// The user's premium type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_type: Option<u8>,
    /// The user's public flags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_flags: Option<u64>,
}

impl CurrentUser {
    /// Returns the user's tag.
    pub fn tag(&self) -> String {
        if self.discriminator == "0" {
            self.username.clone()
        } else {
            format!("{}#{}", self.username, self.discriminator)
        }
    }

    /// Returns the user's mention string.
    pub fn mention(&self) -> String {
        format!("<@{}>", self.id)
    }
}

impl From<CurrentUser> for User {
    fn from(current: CurrentUser) -> Self {
        User {
            id: current.id,
            username: current.username,
            discriminator: current.discriminator,
            global_name: current.global_name,
            avatar: current.avatar,
            bot: current.bot,
            system: false,
            mfa_enabled: current.mfa_enabled,
            banner: current.banner,
            accent_color: current.accent_color,
            locale: current.locale,
            verified: current.verified,
            email: current.email,
            flags: current.flags,
            premium_type: current.premium_type,
            public_flags: current.public_flags,
            avatar_decoration: None,
        }
    }
}
