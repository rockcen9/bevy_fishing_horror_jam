use bevy::prelude::*;
use camera_effects::{FadeInEvent, FadeOutEvent, FadePlugin};

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum DemoState {
    #[default]
    Running,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Screen Fade Example".to_string(),
                resolution: (1280_u32, 720_u32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(camera_effects::plugin)
        .add_plugins(FadePlugin {
            exit_state: DemoState::Running,
        })
        .init_state::<DemoState>()
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Colorful background so the fade overlay is clearly visible.
    for (i, color) in [
        Color::srgb(0.8, 0.2, 0.2),
        Color::srgb(0.2, 0.7, 0.3),
        Color::srgb(0.2, 0.4, 0.9),
        Color::srgb(0.9, 0.7, 0.1),
    ]
    .iter()
    .enumerate()
    {
        commands.spawn((
            Sprite {
                color: *color,
                custom_size: Some(Vec2::new(300.0, 300.0)),
                ..default()
            },
            Transform::from_xyz((i as f32 - 1.5) * 320.0, 0.0, 0.0),
        ));
    }

    commands.spawn((
        Text::new(
            "Screen Fade\n\nI  – fade in  (black → visible)\nO  – fade out (visible → black)\nESC – exit",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
        TextColor(Color::WHITE),
        TextFont {
            font_size: FontSize::Px(22.0),
            ..default()
        },
    ));

    // Start faded-in so the scene is revealed on launch.
    commands.trigger(FadeInEvent::default());
}

fn handle_input(mut commands: Commands, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
    if keyboard.just_pressed(KeyCode::KeyI) {
        commands.trigger(FadeInEvent::default());
    }
    if keyboard.just_pressed(KeyCode::KeyO) {
        commands.trigger(FadeOutEvent::default());
    }
}
