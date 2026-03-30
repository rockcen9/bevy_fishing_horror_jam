use bevy::prelude::*;
use bevy_tweening::{AnimTarget, TweenAnim, lens::TransformScaleLens, *};
use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_add_anim_spawn_on);
}

/// Attach to an entity at spawn time to automatically play a 200 ms
/// scale-from-zero enter animation. No other setup required.
///
/// # Example
/// ```rust
/// commands.spawn((
///     my_sprite_bundle,
///     AnimSpawnOn,
///     AnimDespawnOnExit(GameState::Idle),
/// ));
/// ```
#[derive(Component)]
pub struct AnimSpawnOn;

fn on_add_anim_spawn_on(
    trigger: On<Add, AnimSpawnOn>,
    mut transforms: Query<&mut Transform>,
    mut commands: Commands,
) {
    let entity = trigger.event_target();

    if let Ok(mut transform) = transforms.get_mut(entity) {
        transform.scale = Vec3::ZERO;
    }

    let tween = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_millis(200),
        TransformScaleLens {
            start: Vec3::ZERO,
            end: Vec3::ONE,
        },
    );

    commands.spawn((TweenAnim::new(tween), AnimTarget::component::<Transform>(entity)));
}
