use bevy::prelude::*;
use bevy_tweening::{AnimCompletedEvent, AnimTarget, AnimTargetKind, TweenAnim, lens::TransformScaleLens, *};
use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_add_anim_despawn)
        .add_observer(despawn_on_anim_complete);
}

/// Attach to an entity to immediately play a 200 ms scale-out animation and
/// despawn it once the animation finishes.
///
/// # Example
/// ```rust
/// commands.entity(entity).insert(AnimDespawn);
/// ```
#[derive(Component)]
pub struct AnimDespawn;

fn on_add_anim_despawn(
    trigger: On<Add, AnimDespawn>,
    mut commands: Commands,
) {
    let entity = trigger.event_target();

    let tween = Tween::new(
        EaseFunction::QuadraticIn,
        Duration::from_millis(200),
        TransformScaleLens {
            start: Vec3::ONE,
            end: Vec3::ZERO,
        },
    );

    commands.entity(entity).insert(IsDespawningAnim);
    commands.spawn((TweenAnim::new(tween), AnimTarget::component::<Transform>(entity)));
}

/// Marker inserted automatically when the despawn animation starts.
#[derive(Component)]
struct IsDespawningAnim;

fn despawn_on_anim_complete(
    trigger: On<AnimCompletedEvent>,
    despawning: Query<Entity, With<IsDespawningAnim>>,
    mut commands: Commands,
) {
    let AnimTargetKind::Component { entity } = trigger.event().target else {
        return;
    };
    if despawning.contains(entity) {
        commands.entity(entity).despawn();
    }
}
