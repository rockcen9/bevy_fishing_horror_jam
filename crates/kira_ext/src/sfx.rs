use crate::{SFXCategory, SFXEvent};
use bevy::prelude::*;

#[allow(dead_code)]
struct SFXMessage {
    id: String,
    random_pitch: Option<(f32, f32)>,
    category: SFXCategory,
}

#[cfg(feature = "backend")]
use crate::audio_engine::{SfxChannel, UiChannel};
#[cfg(feature = "backend")]
use bevy_kira_audio::{AudioChannel, AudioControl, AudioSource};

/// Internal queue for pending SFX requests (filled by observer, drained by system).
#[derive(Resource, Default)]
pub(crate) struct SFXQueue(Vec<SFXMessage>);

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<SFXQueue>();
    app.add_observer(sfx_event_system);
    #[cfg(feature = "backend")]
    if std::env::var("DISABLE_AUDIO").is_err() {
        app.add_systems(Update, sfx_message_system);
    }
}

fn sfx_event_system(trigger: On<SFXEvent>, mut queue: ResMut<SFXQueue>) {
    queue.0.push(SFXMessage {
        id: trigger.id.clone(),
        random_pitch: trigger.random_pitch,
        category: trigger.category,
    });
}

#[cfg(feature = "backend")]
fn sfx_message_system(
    mut queue: ResMut<SFXQueue>,
    sfx_channel: Res<AudioChannel<SfxChannel>>,
    ui_channel: Res<AudioChannel<UiChannel>>,
    asset_server: Res<AssetServer>,
    mut cooldowns: Local<bevy::platform::collections::HashMap<String, u32>>,
) {
    // Decrement all cooldowns each frame
    for cooldown in cooldowns.values_mut() {
        if *cooldown > 0 {
            *cooldown -= 1;
        }
    }

    let messages: Vec<SFXMessage> = queue.0.drain(..).collect();

    for message in messages {
        let current_cooldown = cooldowns.get(&message.id).copied().unwrap_or(0);
        if current_cooldown > 0 {
            continue;
        }

        debug!("SFX: '{}'", message.id);

        let path = match message.category {
            SFXCategory::UI => format!("audio/ui/{}.ogg", message.id),
            SFXCategory::Combat => format!("audio/sfx/{}.ogg", message.id),
        };
        let handle: Handle<AudioSource> = asset_server.load(&path);

        if let Some((min, max)) = message.random_pitch {
            use rand::RngExt;
            let pitch = rand::rng().random_range(min..max) as f64;
            match message.category {
                SFXCategory::UI => {
                    ui_channel.play(handle).with_playback_rate(pitch);
                }
                SFXCategory::Combat => {
                    sfx_channel.play(handle).with_playback_rate(pitch);
                }
            }
        } else {
            match message.category {
                SFXCategory::UI => {
                    ui_channel.play(handle);
                }
                SFXCategory::Combat => {
                    sfx_channel.play(handle);
                }
            }
        }

        cooldowns.insert(message.id, 6);
    }
}
