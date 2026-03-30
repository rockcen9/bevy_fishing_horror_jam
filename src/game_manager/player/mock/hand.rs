use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game_manager::player::RightHand;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, sync_right_hand_to_mouse);
}

/// Keeps the RightHand entity visible and positioned at the mouse cursor
/// (world-space). Runs every frame so it overrides any hide_hands calls.
fn sync_right_hand_to_mouse(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut right_hand: Query<(&mut Transform, &mut Visibility), With<RightHand>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((camera, cam_transform)) = camera_q.single() else {
        return;
    };
    let Ok((mut transform, mut vis)) = right_hand.single_mut() else {
        return;
    };

    vis.set_if_neq(Visibility::Inherited);

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(cam_transform, cursor_pos) else {
        return;
    };

    transform.translation.x = world_pos.x;
    transform.translation.y = world_pos.y;
}
