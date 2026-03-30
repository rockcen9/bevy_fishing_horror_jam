use bevy::prelude::*;

#[cfg(feature = "backend")]
use crate::audio_engine::MusicChannel;
#[cfg(feature = "backend")]
use bevy_kira_audio::{AudioChannel, AudioControl, AudioSource, AudioTween};
#[cfg(feature = "backend")]
use std::time::Duration;

/// Tracks the currently playing background music ID.
#[derive(Resource, Default)]
pub struct CurrentBGM {
    pub id: Option<String>,
}

#[cfg(feature = "backend")]
pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<CurrentBGM>();
    if std::env::var("DISABLE_AUDIO").is_err() {
        app.add_observer(play_bgm);
    }
}

#[cfg(not(feature = "backend"))]
pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<CurrentBGM>();
}

#[cfg(feature = "backend")]
fn play_bgm(
    trigger: On<crate::BGMEvent>,
    music_channel: Res<AudioChannel<MusicChannel>>,
    mut current_bgm: ResMut<CurrentBGM>,
    asset_server: Res<AssetServer>,
) {
    if current_bgm.id.as_ref() == Some(&trigger.id) {
        debug!("BGM '{}' already playing, skipping", trigger.id);
        return;
    }

    debug!("Playing BGM: '{}'", trigger.id);

    let fade = AudioTween::linear(Duration::from_secs_f32(1.5));

    music_channel
        .stop()
        .fade_out(AudioTween::linear(Duration::from_secs_f32(0.5)));

    let handle: Handle<AudioSource> = asset_server.load(format!("audio/music/{}.ogg", trigger.id));
    music_channel.play(handle).looped().fade_in(fade);

    current_bgm.id = Some(trigger.id.clone());
}
