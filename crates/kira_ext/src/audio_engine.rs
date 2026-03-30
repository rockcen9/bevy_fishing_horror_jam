use bevy::prelude::*;

#[cfg(feature = "backend")]
use bevy_kira_audio::{AudioApp, AudioChannel, AudioControl};

/// Typed channel for background music.
#[cfg(feature = "backend")]
#[derive(Resource, Default)]
pub struct MusicChannel;

/// Typed channel for non-spatial sound effects.
#[cfg(feature = "backend")]
#[derive(Resource, Default)]
pub struct SfxChannel;

/// Typed channel for UI sound effects.
#[cfg(feature = "backend")]
#[derive(Resource, Default)]
pub struct UiChannel;

/// Default master volume in decibels (~25% amplitude).
#[cfg(feature = "backend")]
pub const DEFAULT_VOLUME_DB: f32 = -12.0;

pub(crate) fn plugin(_app: &mut App) {
    #[cfg(feature = "backend")]
    if std::env::var("DISABLE_AUDIO").is_err() {
        _app.add_audio_channel::<MusicChannel>();
        _app.add_audio_channel::<SfxChannel>();
        _app.add_audio_channel::<UiChannel>();
        _app.add_systems(Startup, initialize_audio);
    }
}

#[cfg(feature = "backend")]
fn initialize_audio(
    music: Res<AudioChannel<MusicChannel>>,
    sfx: Res<AudioChannel<SfxChannel>>,
    ui: Res<AudioChannel<UiChannel>>,
) {
    music.set_volume(DEFAULT_VOLUME_DB);
    sfx.set_volume(DEFAULT_VOLUME_DB);
    ui.set_volume(DEFAULT_VOLUME_DB);
}
