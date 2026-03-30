use crate::prelude::*;

/// How far back in time to look for the start of a casting gesture (seconds).
const CAST_WINDOW_SECS: f32 = 0.5;
/// Minimum downward drop (world units) within the window to count as a cast.
const CAST_MIN_DROP_PX: f32 = 300.0;

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<CastGestureTracker>()
        .add_systems(OnEnter(GameState::Idle), reset_mock_cast_gesture_tracker)
        .add_systems(
            Update,
            detect_cast_gesture_from_mouse.run_if(in_state(GameState::Idle)),
        );
}

#[derive(Resource, Default)]
struct CastGestureTracker {
    samples: Vec<(f32, f32)>,
    cooldown: f32,
}

fn reset_mock_cast_gesture_tracker(mut tracker: ResMut<CastGestureTracker>) {
    *tracker = CastGestureTracker::default();
}

fn detect_cast_gesture_from_mouse(
    right_hand: Query<(&Transform, &Visibility), With<RightHand>>,
    time: Res<Time>,
    mut tracker: ResMut<CastGestureTracker>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let now = time.elapsed_secs();

    if tracker.cooldown > 0.0 {
        tracker.cooldown -= time.delta_secs();
        return;
    }

    let Ok((transform, visibility)) = right_hand.single() else {
        tracker.samples.clear();
        return;
    };

    if *visibility == Visibility::Hidden {
        tracker.samples.clear();
        return;
    }

    let y = transform.translation.y;
    tracker.samples.push((now, y));
    tracker.samples.retain(|(t, _)| now - t <= CAST_WINDOW_SECS);

    if tracker.samples.len() < 2 {
        return;
    }

    let max_y = tracker
        .samples
        .iter()
        .map(|(_, y)| *y)
        .fold(f32::NEG_INFINITY, f32::max);

    // Gesture: hand peaked in the top half, then dropped past center by at least CAST_MIN_DROP_PX.
    if max_y > 0.0 && y < 0.0 && (max_y - y) > CAST_MIN_DROP_PX {
        next_state.set(GameState::Casting);
        tracker.cooldown = 1.0;
        tracker.samples.clear();
    }
}
