use crate::prelude::*;
use bevy_tweening::{lens::TransformRotationLens, *};
use std::time::Duration;

use super::{Backpack, last_item::LastCaughtDescription};

const SHAKE_DEGREES: f32 = 30.0;
/// Total shake duration is 0.2s split across 5 phases: 0→L→R→L→R→0
const SHAKE_PHASE_MS: u64 = 40;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        shake_backpack_on_fish_arrived
            .run_if(resource_changed::<LastCaughtDescription>)
            .in_set(PausableSystems),
    );
}

fn shake_backpack_on_fish_arrived(
    description: Res<LastCaughtDescription>,
    backpack: Query<Entity, With<Backpack>>,
    mut commands: Commands,
) {
    if description.image_path.is_empty() {
        return;
    }
    let Ok(entity) = backpack.single() else {
        return;
    };

    let center = Quat::IDENTITY;
    let left = Quat::from_rotation_z(SHAKE_DEGREES.to_radians());
    let right = Quat::from_rotation_z(-SHAKE_DEGREES.to_radians());

    let phase = Duration::from_millis(SHAKE_PHASE_MS);

    let seq = Tween::new(EaseFunction::Linear, phase, TransformRotationLens { start: center, end: left })
        .then(Tween::new(EaseFunction::Linear, phase, TransformRotationLens { start: left, end: right }))
        .then(Tween::new(EaseFunction::Linear, phase, TransformRotationLens { start: right, end: left }))
        .then(Tween::new(EaseFunction::Linear, phase, TransformRotationLens { start: left, end: right }))
        .then(Tween::new(EaseFunction::Linear, phase, TransformRotationLens { start: right, end: center }));

    commands.spawn((
        TweenAnim::new(seq),
        AnimTarget::component::<Transform>(entity),
    ));
}
