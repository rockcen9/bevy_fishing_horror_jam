use crate::prelude::*;
use super::{BitingSet, detect::PullUpInput};
use bevy_tweening::{lens::TransformRotateZLens, *};
use std::time::Duration;
use super::qte::QteState;

/// How long the snap-back stroke takes (fast).
const PULL_FORWARD_DURATION_MS: u64 = 120;
/// How long the return stroke takes (slow ease out).
const PULL_RETURN_DURATION_MS: u64 = 500;
/// Extra backward rotation on pull (20°).
const PULL_ANGLE_RAD: f32 = 20.0 * std::f32::consts::PI / 180.0;
/// Extra bend boost (local pixels) injected into the shader at peak pull.
pub(super) const PULL_BEND_BOOST_PX: f32 = 150.0;
/// How fast the boost decays to zero (units/sec).
const PULL_BOOST_DECAY_RATE_PX_PER_SEC: f32 = 240.0;

#[derive(Component)]
struct PullForwardTween;

#[derive(Component)]
struct PullReturnTween;

#[derive(Resource)]
struct PullForwardTimer(Timer);

#[derive(Resource)]
struct PullReturnTimer(Timer);

/// Tracks the extra bend magnitude to add on top of the fish-pull bend.
/// Created on pull-input, decays to zero, then removed.
#[derive(Resource)]
pub(super) struct PullBendBoost(pub(super) f32);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        on_pull_gesture_detected
            .in_set(BitingSet::PullInput)
            .run_if(in_state(GameState::Biting))
            .run_if(in_state(Pause(false))),
    );
    app.add_systems(
        Update,
        (tick_pull_timer, tick_pull_return_timer, decay_pull_bend_boost)
            .run_if(in_state(GameState::Biting))
            .run_if(in_state(Pause(false))),
    );
    app.add_systems(OnExit(GameState::Biting), despawn_pull_up_tweens);
}

fn on_pull_gesture_detected(
    pull_input: Res<PullUpInput>,
    q_pole: Query<(Entity, &Transform), With<Pole>>,
    pull_timer: Option<Res<PullForwardTimer>>,
    return_timer: Option<Res<PullReturnTimer>>,
    qte: Res<QteState>,
    mut commands: Commands,
) {
    // Block re-entry while a pull or return is in progress, or after committing.
    if pull_timer.is_some() || return_timer.is_some() || qte.result.is_some() {
        return;
    }
    if !pull_input.triggered {
        return;
    }

    let Ok((pole_entity, transform)) = q_pole.single() else {
        return;
    };

    let current = transform.rotation.to_euler(EulerRot::XYZ).2;
    let tween = Tween::new(
        EaseFunction::SineIn,
        Duration::from_millis(PULL_FORWARD_DURATION_MS),
        TransformRotateZLens {
            start: current,
            // Rotate clockwise (backward) — negative delta.
            end: current - PULL_ANGLE_RAD,
        },
    );
    commands.spawn((
        PullForwardTween,
        TweenAnim::new(tween),
        AnimTarget::component::<Transform>(pole_entity),
    ));
    commands.insert_resource(PullForwardTimer(Timer::from_seconds(
        PULL_FORWARD_DURATION_MS as f32 / 1000.0,
        TimerMode::Once,
    )));
    commands.insert_resource(PullBendBoost(PULL_BEND_BOOST_PX));
}

fn tick_pull_timer(
    time: Res<Time>,
    pull_timer: Option<ResMut<PullForwardTimer>>,
    q_pole: Query<(Entity, &Transform), With<Pole>>,
    q_pull_tween: Query<Entity, With<PullForwardTween>>,
    mut commands: Commands,
) {
    let Some(mut timer) = pull_timer else {
        return;
    };
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    for e in q_pull_tween.iter() {
        commands.entity(e).despawn();
    }
    commands.remove_resource::<PullForwardTimer>();

    let Ok((pole_entity, transform)) = q_pole.single() else {
        return;
    };
    let current = transform.rotation.to_euler(EulerRot::XYZ).2;
    let tween = Tween::new(
        EaseFunction::SineOut,
        Duration::from_millis(PULL_RETURN_DURATION_MS),
        TransformRotateZLens {
            start: current,
            end: current + PULL_ANGLE_RAD,
        },
    );
    commands.spawn((
        PullReturnTween,
        TweenAnim::new(tween),
        AnimTarget::component::<Transform>(pole_entity),
    ));
    commands.insert_resource(PullReturnTimer(Timer::from_seconds(
        PULL_RETURN_DURATION_MS as f32 / 1000.0,
        TimerMode::Once,
    )));
}

fn tick_pull_return_timer(
    time: Res<Time>,
    return_timer: Option<ResMut<PullReturnTimer>>,
    q_return_tween: Query<Entity, With<PullReturnTween>>,
    qte: Res<QteState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    let Some(mut timer) = return_timer else {
        return;
    };
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    for e in q_return_tween.iter() {
        commands.entity(e).despawn();
    }
    commands.remove_resource::<PullReturnTimer>();

    // Transition based on QTE result captured at Space press.
    match qte.result {
        Some(true) => next_state.set(GameState::Succeeding),
        _ => next_state.set(GameState::Failing),
    }
}

/// Linearly decays the bend boost toward zero each frame.
fn decay_pull_bend_boost(
    time: Res<Time>,
    boost: Option<ResMut<PullBendBoost>>,
    mut commands: Commands,
) {
    let Some(mut boost) = boost else {
        return;
    };
    boost.0 = (boost.0 - time.delta_secs() * PULL_BOOST_DECAY_RATE_PX_PER_SEC).max(0.0);
    if boost.0 <= 0.0 {
        commands.remove_resource::<PullBendBoost>();
    }
}

fn despawn_pull_up_tweens(
    q_pull: Query<Entity, With<PullForwardTween>>,
    q_return: Query<Entity, With<PullReturnTween>>,
    mut commands: Commands,
) {
    for e in q_pull.iter() {
        commands.entity(e).despawn();
    }
    for e in q_return.iter() {
        commands.entity(e).despawn();
    }
    commands.remove_resource::<PullForwardTimer>();
    commands.remove_resource::<PullReturnTimer>();
    commands.remove_resource::<PullBendBoost>();
}
