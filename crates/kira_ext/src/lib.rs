//! # Rock Kira
//!
//! Vertical-sliced audio crate using bevy_kira_audio with headless mode support.
//!
//! ## Usage
//!
//! ```rust
//! use bevy::prelude::*;
//! use rock_kira::{BGMEvent, SFXEvent};
//!
//! fn play_audio(mut commands: Commands) {
//!     // Play background music
//!     commands.trigger(BGMEvent::new("prepare"));
//!
//!     // Play sound effect
//!     commands.trigger(SFXEvent::new("coin"));
//!
//!     // Play UI sound effect
//!     commands.trigger(SFXEvent::ui("click"));
//!
//!     // Play sound effect with random pitch
//!     commands.trigger(SFXEvent::new("hit").with_random_pitch(0.9, 1.1));
//! }
//! ```

use bevy::prelude::*;

pub mod audio_engine;
pub mod bgm;
pub mod sfx;

pub use bgm::CurrentBGM;

// ============================================================================
// Shared Contracts (Always Compiled)
// ============================================================================

/// Background music event - triggers music playback
#[derive(Event, Clone, Debug)]
pub struct BGMEvent {
    pub id: String,
}

impl BGMEvent {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

/// Sound effect category
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SFXCategory {
    UI,
    Combat,
}

/// Sound effect event - triggers SFX playback
#[derive(Event, Clone, Debug)]
pub struct SFXEvent {
    pub id: String,
    pub category: SFXCategory,
    pub random_pitch: Option<(f32, f32)>,
}

impl SFXEvent {
    pub fn sfx(id: &str) -> Self {
        Self {
            id: id.to_string(),
            category: SFXCategory::Combat,
            random_pitch: None,
        }
    }

    pub fn ui(id: &str) -> Self {
        Self {
            id: id.to_string(),
            category: SFXCategory::UI,
            random_pitch: None,
        }
    }

    pub fn with_random_pitch(mut self, min: f32, max: f32) -> Self {
        self.random_pitch = Some((min, max));
        self
    }
}

// ============================================================================
// Main Plugin
// ============================================================================

pub fn plugin(app: &mut App) {
    #[cfg(feature = "backend")]
    {
        use bevy_kira_audio::{AudioSettings, BufferSize};
        app.insert_resource(AudioSettings {
            buffer_size: BufferSize::Fixed(2048),
            ..Default::default()
        });
        app.add_plugins(bevy_kira_audio::AudioPlugin);
    }
    audio_engine::plugin(app);
    bgm::plugin(app);
    sfx::plugin(app);
}
