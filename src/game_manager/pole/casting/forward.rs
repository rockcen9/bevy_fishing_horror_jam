use crate::prelude::*;
use bevy_tweening::{lens::TransformRotateZLens, *};
use kira_ext::SFXEvent;
use std::time::Duration;

const CAST_ROTATE_DURATION_MS: u64 = 200;
const CAST_JITTER_DURATION_SECS: f32 = 0.3;
const CAST_RETURN_DURATION_MS: u64 = 600;
/// ~60 degrees forward (left) lean for the cast.
const CAST_ANGLE_RAD: f32 = std::f32::consts::FRAC_PI_3;
/// How far to rotate back after the jitter (degrees).
const CAST_RETURN_AMOUNT_DEG: f32 = 40.0;
const CAST_RETURN_AMOUNT_RAD: f32 = CAST_RETURN_AMOUNT_DEG * std::f32::consts::PI / 180.0;
/// Jitter oscillation speed (radians/sec).
const CAST_JITTER_FREQ_RAD_PER_SEC: f32 = 30.0;
/// Peak jitter amplitude (~5 degrees) — decays to zero as line extends.
const CAST_JITTER_AMP_RAD: f32 = 0.087;

#[derive(Component)]
struct CastForwardTween;

#[derive(Component)]
struct CastReturnTween;

#[derive(Resource)]
struct CastRotateTimer(Timer);

#[derive(Resource)]
pub(crate) struct CastJitterTimer(pub(crate) Timer);

#[derive(Resource)]
pub(crate) struct CastReturnTimer(pub(crate) Timer);

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Casting), begin_cast_animation);
    app.add_systems(
        Update,
        (advance_cast_to_jitter_phase, tick_cast_jitter_phase, advance_cast_to_waiting_state)
            .run_if(in_state(GameState::Casting))
            .run_if(in_state(Pause(false))),
    );
}

fn begin_cast_animation(q_pole: Query<(Entity, &Transform), With<Pole>>, mut commands: Commands) {
    let Ok((pole_entity, transform)) = q_pole.single() else {
        return;
    };

    let current = transform.rotation.to_euler(EulerRot::XYZ).2;
    let tween = Tween::new(
        EaseFunction::SineIn,
        Duration::from_millis(CAST_ROTATE_DURATION_MS),
        TransformRotateZLens {
            start: current,
            end: CAST_ANGLE_RAD,
        },
    );

    commands.spawn((
        CastForwardTween,
        TweenAnim::new(tween),
        AnimTarget::component::<Transform>(pole_entity),
    ));

    commands.insert_resource(CastRotateTimer(Timer::from_seconds(
        CAST_ROTATE_DURATION_MS as f32 / 1000.0,
        TimerMode::Once,
    )));
    commands.trigger(SFXEvent::sfx("casting"));
}

fn advance_cast_to_jitter_phase(
    time: Res<Time>,
    rotate_timer: Option<ResMut<CastRotateTimer>>,
    q_cast_tween: Query<Entity, With<CastForwardTween>>,
    mut commands: Commands,
) {
    let Some(mut timer) = rotate_timer else {
        return;
    };

    if timer.0.tick(time.delta()).just_finished() {
        for e in q_cast_tween.iter() {
            commands.entity(e).despawn();
        }
        commands.remove_resource::<CastRotateTimer>();
        commands.insert_resource(CastJitterTimer(Timer::from_seconds(
            CAST_JITTER_DURATION_SECS,
            TimerMode::Once,
        )));
    }
}

fn tick_cast_jitter_phase(
    time: Res<Time>,
    jitter_timer: Option<ResMut<CastJitterTimer>>,
    mut q_pole: Query<(&mut Transform, Entity), With<Pole>>,
    mut commands: Commands,
) {
    let Some(mut timer) = jitter_timer else {
        return;
    };

    timer.0.tick(time.delta());
    let elapsed = timer.0.elapsed_secs();
    let decay = 1.0 - (elapsed / CAST_JITTER_DURATION_SECS);
    let jitter = (elapsed * CAST_JITTER_FREQ_RAD_PER_SEC).sin() * CAST_JITTER_AMP_RAD * decay;

    if let Ok((mut transform, _)) = q_pole.single_mut() {
        transform.rotation = Quat::from_rotation_z(CAST_ANGLE_RAD + jitter);
    }

    if timer.0.just_finished() {
        commands.remove_resource::<CastJitterTimer>();

        let Ok((_, pole_entity)) = q_pole.single() else {
            return;
        };
        let tween = Tween::new(
            EaseFunction::SineOut,
            Duration::from_millis(CAST_RETURN_DURATION_MS),
            TransformRotateZLens {
                start: CAST_ANGLE_RAD,
                end: CAST_ANGLE_RAD - CAST_RETURN_AMOUNT_RAD,
            },
        );
        commands.spawn((
            CastReturnTween,
            TweenAnim::new(tween),
            AnimTarget::component::<Transform>(pole_entity),
        ));
        commands.insert_resource(CastReturnTimer(Timer::from_seconds(
            CAST_RETURN_DURATION_MS as f32 / 1000.0,
            TimerMode::Once,
        )));
    }
}

fn advance_cast_to_waiting_state(
    time: Res<Time>,
    return_timer: Option<ResMut<CastReturnTimer>>,
    q_return_tween: Query<Entity, With<CastReturnTween>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    let Some(mut timer) = return_timer else {
        return;
    };

    if timer.0.tick(time.delta()).just_finished() {
        for e in q_return_tween.iter() {
            commands.entity(e).despawn();
        }
        commands.remove_resource::<CastReturnTimer>();
        next_state.set(GameState::Waiting);
    }
}
