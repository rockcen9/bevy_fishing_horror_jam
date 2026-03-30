use crate::prelude::*;

#[derive(Resource, Default, Debug, Reflect)]
pub struct RightHandScreenPosition {
    pub right_hand: Option<ScreenHalf>,
}

#[derive(Debug, Reflect, PartialEq, Eq, Clone, Copy)]
pub enum ScreenHalf {
    Top,
    Bottom,
}

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<RightHandScreenPosition>();

    #[cfg(feature = "backend")]
    app.add_systems(Update, update_hand_screen_from_detections);

    #[cfg(not(feature = "backend"))]
    app.add_systems(Update, update_hand_screen_from_mouse);
}

/// Deadzone around y=0 to prevent rapid Top/Bottom flipping when cursor hovers near center.
const HAND_SCREEN_HYSTERESIS_PX: f32 = 80.0;

#[cfg(not(feature = "backend"))]
fn update_hand_screen_from_mouse(
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut hand_screen: ResMut<RightHandScreenPosition>,
) {
    let Ok(window) = windows.single() else {
        hand_screen.right_hand = None;
        return;
    };
    let Ok((camera, cam_transform)) = camera_q.single() else {
        hand_screen.right_hand = None;
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok(world_pos) = camera.viewport_to_world_2d(cam_transform, cursor_pos) else {
        return;
    };

    debug!("hand world_y={:.1}", world_pos.y);

    // Hysteresis: only transition out of current state when the cursor clearly crosses
    // the threshold, preventing oscillation when hovering near the boundary.
    let position = match hand_screen.right_hand {
        Some(ScreenHalf::Top) => {
            if world_pos.y < -HAND_SCREEN_HYSTERESIS_PX {
                ScreenHalf::Bottom
            } else {
                ScreenHalf::Top
            }
        }
        _ => {
            if world_pos.y >= HAND_SCREEN_HYSTERESIS_PX {
                ScreenHalf::Top
            } else {
                ScreenHalf::Bottom
            }
        }
    };
    let next = Some(position);
    if hand_screen.right_hand != next {
        hand_screen.right_hand = next;
    }
}

#[cfg(feature = "backend")]
fn update_hand_screen_from_detections(
    detections: Res<yolo::PlayerDetections>,
    mut hand_screen: ResMut<RightHandScreenPosition>,
) {
    let Some(wrist) = detections.right_wrist.as_ref() else {
        hand_screen.right_hand = None;
        return;
    };

    let position = if wrist.center().y >= 0.0 {
        ScreenHalf::Top
    } else {
        ScreenHalf::Bottom
    };

    hand_screen.right_hand = Some(position);
}
