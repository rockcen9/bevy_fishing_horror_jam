use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game_manager::player::PlayerHeadPosition;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, sync_head_to_mouse);
}

/// Keeps PlayerHeadPosition updated to the mouse cursor (world-space).
fn sync_head_to_mouse(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut head: ResMut<PlayerHeadPosition>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((camera, cam_transform)) = camera_q.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(cam_transform, cursor_pos) else {
        return;
    };

    head.position = world_pos;
}
