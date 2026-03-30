//! Demonstrates playing background music via `BGMEvent` with fade transitions.
//!
//! Run from the workspace root (so `assets/` is accessible):
//!
//!   cargo run --example play_bgm -p kira_ext
//!
//! Controls:
//!   1   - Play "prepare" music
//!   2   - Play "battle" music
//!   ESC - Exit

use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use kira_ext::{BGMEvent, CurrentBGM};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "kira_ext - BGM Example".to_string(),
                resolution: bevy::window::WindowResolution::new(800u32, 400u32),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AudioPlugin)
        .add_plugins(kira_ext::plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_input, update_status_text))
        .run();
}

#[derive(Component)]
struct StatusText;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text::new(
            "kira_ext BGM Example\n\n\
            1   - Play \"prepare\" music\n\
            2   - Play \"battle\" music\n\
            ESC - Exit",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        TextColor(Color::WHITE),
        TextFont { font_size: FontSize::Px(22.0), ..default() },
    ));

    commands.spawn((
        Text::new("Now playing: (none)"),
        StatusText,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        TextColor(Color::srgb(0.8, 1.0, 0.8)),
        TextFont { font_size: FontSize::Px(20.0), ..default() },
    ));
}

fn handle_input(keyboard: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if keyboard.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
    if keyboard.just_pressed(KeyCode::Digit1) {
        commands.trigger(BGMEvent::new("prepare"));
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        commands.trigger(BGMEvent::new("battle"));
    }
}

fn update_status_text(
    current_bgm: Res<CurrentBGM>,
    mut status_q: Query<&mut Text, With<StatusText>>,
) {
    let Ok(mut text) = status_q.single_mut() else { return };
    let label = current_bgm.id.as_deref().unwrap_or("(none)");
    **text = format!("Now playing: {label}");
}
