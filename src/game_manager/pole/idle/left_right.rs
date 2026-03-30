use bevy_tweening::{lens::TransformPositionLens, *};
use std::time::Duration;

use crate::prelude::*;

const POLE_IDLE_SWAY_PX: f32 = 10.0;
const POLE_IDLE_SWAY_DURATION_MS: u64 = 2500;

#[derive(Component)]
struct PoleIdleTween;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Idle), spawn_pole_idle_sway_tween);
    app.add_systems(OnExit(GameState::Idle), despawn_pole_idle_sway_tween);
    app.add_systems(OnEnter(GameState::Waiting), spawn_pole_idle_sway_tween);
    app.add_systems(OnExit(GameState::Waiting), despawn_pole_idle_sway_tween);
}

fn spawn_pole_idle_sway_tween(mut commands: Commands, q_pole: Query<(Entity, &Transform), With<Pole>>) {
    let Ok((pole_entity, transform)) = q_pole.single() else {
        return;
    };

    let base = transform.translation;
    let tween = Tween::new(
        EaseFunction::SineInOut,
        Duration::from_millis(POLE_IDLE_SWAY_DURATION_MS),
        TransformPositionLens {
            start: base + Vec3::new(-POLE_IDLE_SWAY_PX, 0.0, 0.0),
            end: base + Vec3::new(POLE_IDLE_SWAY_PX, 0.0, 0.0),
        },
    )
    .with_repeat(RepeatCount::Infinite, RepeatStrategy::MirroredRepeat);

    commands.spawn((
        PoleIdleTween,
        TweenAnim::new(tween),
        AnimTarget::component::<Transform>(pole_entity),
    ));
}

fn despawn_pole_idle_sway_tween(mut commands: Commands, q: Query<Entity, With<PoleIdleTween>>) {
    for entity in q.iter() {
        commands.entity(entity).despawn();
    }
}
