use crate::prelude::*;
use bevy_tweening::{lens::TransformRotateZLens, *};
use std::time::Duration;

use super::spawn_pole::Pole;

const HIDE_DURATION_SECS: f32 = 0.3;

#[derive(Component)]
struct PoleHideTween;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Monster), rotate_pole_down);
    app.add_systems(OnExit(GameState::Monster), rotate_pole_up);
}

fn rotate_pole_down(q_pole: Query<(Entity, &Transform), With<Pole>>, mut commands: Commands) {
    let Ok((pole_entity, transform)) = q_pole.single() else {
        return;
    };

    let current = transform.rotation.to_euler(EulerRot::XYZ).2;
    let tween = Tween::new(
        EaseFunction::SineInOut,
        Duration::from_secs_f32(HIDE_DURATION_SECS),
        TransformRotateZLens {
            start: current,
            end: current + std::f32::consts::PI,
        },
    );

    commands.spawn((
        PoleHideTween,
        TweenAnim::new(tween),
        AnimTarget::component::<Transform>(pole_entity),
    ));
}

fn rotate_pole_up(
    q_pole: Query<(Entity, &Transform), With<Pole>>,
    q_tween: Query<Entity, With<PoleHideTween>>,
    mut commands: Commands,
) {
    for e in q_tween.iter() {
        commands.entity(e).despawn();
    }

    let Ok((pole_entity, transform)) = q_pole.single() else {
        return;
    };

    let current = transform.rotation.to_euler(EulerRot::XYZ).2;
    let tween = Tween::new(
        EaseFunction::SineInOut,
        Duration::from_secs_f32(HIDE_DURATION_SECS),
        TransformRotateZLens {
            start: current,
            end: 0.0,
        },
    );

    commands.spawn((
        PoleHideTween,
        TweenAnim::new(tween),
        AnimTarget::component::<Transform>(pole_entity),
    ));
}
