use yolo::PlayerDetections;

use crate::prelude::*;

/// How far back in time to look for the start of a casting gesture (seconds).
const CAST_WINDOW_SECS: f32 = 0.5;
/// Minimum downward drop (world units) within the window to count as a cast.
/// The hand must have been in the top half and dropped this many units.
const CAST_MIN_DROP_PX: f32 = 300.0;

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<CastGestureTracker>()
        .add_systems(OnEnter(GameState::Idle), reset_cast_gesture_tracker)
        .add_systems(
            Update,
            detect_cast_gesture
                .run_if(in_state(GameState::Idle))
                .run_if(in_state(Pause(false))),
        );
}

#[derive(Resource, Default)]
struct CastGestureTracker {
    /// Ring of (elapsed_secs, world_y) palm samples within the detection window.
    samples: Vec<(f32, f32)>,
    /// Prevents re-triggering immediately after a cast.
    cooldown: f32,
}

fn reset_cast_gesture_tracker(mut tracker: ResMut<CastGestureTracker>) {
    *tracker = CastGestureTracker::default();
}

fn detect_cast_gesture(
    detections: Res<PlayerDetections>,
    time: Res<Time>,
    mut tracker: ResMut<CastGestureTracker>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let now = time.elapsed_secs();

    if tracker.cooldown > 0.0 {
        tracker.cooldown -= time.delta_secs();
        return;
    }

    let Some(wrist) = detections
        .left_wrist
        .as_ref()
        .or(detections.right_wrist.as_ref())
    else {
        tracker.samples.clear();
        return;
    };

    let y = wrist.center().y;
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
    let travel = max_y - y;
    debug!(
        "[cast gesture] max_y={:.1} cur_y={:.1} travel={:.1}/{:.1} | max_ok={} cur_ok={}",
        max_y,
        y,
        travel,
        CAST_MIN_DROP_PX,
        max_y > 0.0,
        y < 0.0
    );
    if max_y > 0.0 && y < 0.0 && travel > CAST_MIN_DROP_PX {
        next_state.set(GameState::Casting);
        tracker.cooldown = 1.0;
        tracker.samples.clear();
    }
}
