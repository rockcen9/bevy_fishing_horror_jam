use yolo::PlayerDetections;

use crate::prelude::*;
use super::BitingSet;

/// How far back in time to look for the start of a pull-up gesture (seconds).
const PULL_GESTURE_WINDOW_SECS: f32 = 0.5;
/// Minimum upward rise (world units) within the window to count as a pull-up.
/// The hand must have been in the bottom half and risen past center by this much.
const PULL_GESTURE_MIN_RISE_PX: f32 = 300.0;

/// Set to `true` for exactly one frame when the pull-up gesture (or debug key) fires.
/// Consumed by `pull_up` and `qte` systems in the same frame.
#[derive(Resource, Default)]
pub(super) struct PullUpInput {
    pub triggered: bool,
}

#[derive(Resource, Default)]
struct PullGestureTracker {
    /// Ring of (elapsed_secs, world_y) palm samples within the detection window.
    samples: Vec<(f32, f32)>,
    /// Prevents re-triggering immediately after a gesture fires.
    cooldown: f32,
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<PullUpInput>();
    app.init_resource::<PullGestureTracker>();
    app.add_systems(OnEnter(GameState::Biting), reset_pull_gesture_tracker);
    app.add_systems(
        Update,
        detect_pull_gesture
            .in_set(BitingSet::GestureDetect)
            .run_if(in_state(GameState::Biting))
            .run_if(in_state(Pause(false))),
    );
}

fn reset_pull_gesture_tracker(mut tracker: ResMut<PullGestureTracker>, mut input: ResMut<PullUpInput>) {
    *tracker = PullGestureTracker::default();
    input.triggered = false;
}

fn detect_pull_gesture(
    detections: Res<PlayerDetections>,
    right_hand: Query<(&Transform, &Visibility), With<RightHand>>,
    time: Res<Time>,
    mut tracker: ResMut<PullGestureTracker>,
    mut input: ResMut<PullUpInput>,
) {
    // Reset each frame before evaluating.
    input.triggered = false;

    let now = time.elapsed_secs();

    if tracker.cooldown > 0.0 {
        tracker.cooldown -= time.delta_secs();
        return;
    }

    // Prefer real palm detection; fall back to RightHand (mouse in mock mode).
    let y = if let Some(wrist) = detections.left_wrist.as_ref().or(detections.right_wrist.as_ref()) {
        wrist.center().y
    } else if let Ok((transform, visibility)) = right_hand.single() {
        if *visibility == Visibility::Hidden {
            tracker.samples.clear();
            return;
        }
        transform.translation.y
    } else {
        tracker.samples.clear();
        return;
    };

    tracker.samples.push((now, y));
    tracker.samples.retain(|(t, _)| now - t <= PULL_GESTURE_WINDOW_SECS);

    if tracker.samples.len() < 2 {
        return;
    }

    let min_y = tracker
        .samples
        .iter()
        .map(|(_, y)| *y)
        .fold(f32::INFINITY, f32::min);

    // Gesture: hand was in the bottom half, then rose past center by at least PULL_GESTURE_MIN_RISE_PX.
    if min_y < 0.0 && y > 0.0 && (y - min_y) > PULL_GESTURE_MIN_RISE_PX {
        input.triggered = true;
        tracker.cooldown = 1.0;
        tracker.samples.clear();
    }
}
