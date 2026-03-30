use bevy::math::AspectRatio;
use bevy::prelude::*;
use bevy_simple_screen_boxing::{CameraBox, CameraBoxingPlugin};

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(CameraBoxingPlugin);
    app.add_systems(PostStartup, add_camera_box);
}

fn add_camera_box(mut commands: Commands, camera_q: Query<Entity, With<Camera>>) {
    for entity in &camera_q {
        commands.entity(entity).insert(CameraBox::StaticAspectRatio {
            aspect_ratio: AspectRatio::try_from_pixels(16, 9).unwrap(),
            position: None,
        });
    }
}
