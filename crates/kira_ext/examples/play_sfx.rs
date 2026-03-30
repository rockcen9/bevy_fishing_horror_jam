//! Demonstrates playing sound effects via `SFXEvent`.
//!
//! Run from the workspace root (so `assets/` is accessible):
//!
//!   cargo run --example play_sfx -p kira_ext
//!
//! Controls:
//!   H   - Combat SFX: hit
//!   A   - Combat SFX: arrow  (random pitch 0.9–1.1)
//!   C   - UI SFX:     coin
//!   ESC - Exit

use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use kira_ext::SFXEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "kira_ext - SFX Example".to_string(),
                resolution: bevy::window::WindowResolution::new(800u32, 400u32),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AudioPlugin)
        .add_plugins(kira_ext::plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text::new(
            "kira_ext SFX Example\n\n\
            H   - Combat SFX: hit\n\
            A   - Combat SFX: arrow  (random pitch 0.9–1.1)\n\
            C   - UI SFX:     coin\n\
            ESC - Exit",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        TextColor(Color::WHITE),
        TextFont {
            font_size: FontSize::Px(22.0),
            ..default()
        },
    ));
}

fn handle_input(keyboard: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if keyboard.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
    if keyboard.just_pressed(KeyCode::KeyH) {
        commands.trigger(SFXEvent::sfx("hit"));
    }
    if keyboard.just_pressed(KeyCode::KeyA) {
        commands.trigger(SFXEvent::sfx("arrow").with_random_pitch(0.9, 1.1));
    }
    if keyboard.just_pressed(KeyCode::KeyC) {
        commands.trigger(SFXEvent::ui("coin"));
    }
}
