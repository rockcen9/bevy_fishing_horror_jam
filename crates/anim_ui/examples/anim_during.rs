//! Demonstrates `AnimDuring`: spawn a sprite bound to a state using a single
//! component that handles both the scale-in entrance and scale-out despawn.
//!
//! Run from the workspace root:
//!
//!   cargo run --example anim_during -p anim_ui
//!
//! Controls:
//!   SPACE - toggle between StateA and StateB
//!   ESC   - exit

use anim_ui::{AnimDuring, AnimUiPlugin};
use bevy::prelude::*;
use bevy_tweening::TweeningPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "anim_ui – anim_during".to_string(),
                resolution: bevy::window::WindowResolution::new(800u32, 600u32),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(TweeningPlugin)
        .add_plugins(AnimUiPlugin::new().with_state::<DemoState>())
        .init_state::<DemoState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(DemoState::A), spawn_sprite)
        .add_systems(OnEnter(DemoState::B), spawn_sprite)
        .add_systems(Update, (handle_input, update_label))
        .run();
}

// ── State ─────────────────────────────────────────────────────────────────────

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum DemoState {
    #[default]
    A,
    B,
}

// ── Markers ───────────────────────────────────────────────────────────────────

#[derive(Component)]
struct HintText;

// ── Setup ─────────────────────────────────────────────────────────────────────

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text::new("SPACE – toggle state   ESC – quit\nActive state: A"),
        HintText,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            left: Val::Px(16.0),
            ..default()
        },
        TextColor(Color::WHITE),
        TextFont {
            font_size: FontSize::Px(20.0),
            ..default()
        },
    ));
}

// ── Per-state sprite ──────────────────────────────────────────────────────────

fn spawn_sprite(mut commands: Commands, state: Res<State<DemoState>>) {
    let (color, x) = match state.get() {
        DemoState::A => (Color::srgb(0.2, 0.6, 1.0), -160.0_f32),
        DemoState::B => (Color::srgb(1.0, 0.4, 0.2), 160.0_f32),
    };

    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::splat(128.0)),
            ..default()
        },
        Transform::from_xyz(x, 0.0, 0.0),
        // Scales in on spawn, scales out and despawns when state changes.
        AnimDuring(state.get().clone()),
    ));
}

// ── Input / label ─────────────────────────────────────────────────────────────

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<DemoState>>,
    mut next: ResMut<NextState<DemoState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
    if keyboard.just_pressed(KeyCode::Space) {
        let next_state = match state.get() {
            DemoState::A => DemoState::B,
            DemoState::B => DemoState::A,
        };
        next.set(next_state);
    }
}

fn update_label(state: Res<State<DemoState>>, mut query: Query<&mut Text, With<HintText>>) {
    if !state.is_changed() {
        return;
    }
    let Ok(mut text) = query.single_mut() else {
        return;
    };
    **text = format!(
        "SPACE – toggle state   ESC – quit\nActive state: {:?}",
        state.get()
    );
}
