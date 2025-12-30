//! Ferricord Voice - Voice connection support
//!
//! This crate will provide voice connection support for Ferricord.
//! Voice support is planned for Phase 3 of development.
//!
//! ## Planned Features
//!
//! - Voice Gateway connection (WebSocket)
//! - Opus codec encoding/decoding (48kHz stereo)
//! - UDP RTP + libsodium encryption
//! - Audio playback and recording
//!
//! ## Example (Future API)
//!
//! ```ignore
//! let voice_client = channel.connect().await?;
//! let audio_source = ferricord::FFmpegPCMAudio::new("song.mp3");
//! voice_client.play(audio_source);
//! voice_client.disconnect().await?;
//! ```

/// Voice connection placeholder.
///
/// This module will contain the voice connection implementation in Phase 3.
pub mod connection {
    /// Placeholder for voice connection.
    pub struct VoiceConnection {
        _private: (),
    }

    impl VoiceConnection {
        /// Create a new voice connection (placeholder).
        pub fn new() -> Self {
            Self { _private: () }
        }
    }

    impl Default for VoiceConnection {
        fn default() -> Self {
            Self::new()
        }
    }
}

pub use connection::VoiceConnection;
