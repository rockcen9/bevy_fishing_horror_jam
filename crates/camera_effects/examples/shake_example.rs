use bevy::prelude::*;
use camera_effects::CameraShakeEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Camera Shake Example".to_string(),
                resolution: (1280_u32, 720_u32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(camera_effects::plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Grid of colored squares so the shake is clearly visible.
    for col in -4..=4 {
        for row in -3..=3 {
            let hue = (col + 4) as f32 * 20.0;
            commands.spawn((
                Sprite {
                    color: Color::hsl(hue, 0.7, 0.5),
                    custom_size: Some(Vec2::splat(70.0)),
                    ..default()
                },
                Transform::from_xyz(col as f32 * 130.0, row as f32 * 110.0, 0.0),
            ));
        }
    }

    commands.spawn((
        Text::new("Camera Shake\n\nSPACE  – trigger shake\nESC    – exit"),
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
}

fn handle_input(mut commands: Commands, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
    if keyboard.just_pressed(KeyCode::Space) {
        commands.trigger(CameraShakeEvent);
    }
}
