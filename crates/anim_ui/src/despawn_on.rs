use bevy::prelude::*;
use bevy_tweening::{AnimCompletedEvent, AnimTarget, AnimTargetKind, TweenAnim, lens::TransformScaleLens, *};
use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(despawn_on_anim_complete);
}

/// Attach to an entity at spawn time to declare which state it belongs to.
/// When the active state no longer matches the inner state value, a scale-to-zero
/// exit animation (200 ms) is played and the entity is despawned once finished.
///
/// Add [`AnimUiPlugin<S>`](crate::AnimUiPlugin) to activate monitoring for the
/// relevant `States` type.
///
/// # Example
/// ```rust
/// commands.spawn((
///     my_sprite_bundle,
///     AnimDespawnOnExit(GameState::Idle),
/// ));
/// ```
#[derive(Component)]
pub struct AnimDespawnOnExit<S: States>(pub S);

/// Marker inserted automatically once the exit animation has started.
/// Prevents the exit system from retriggering the animation.
#[derive(Component)]
pub struct IsDespawning;

pub(super) fn register<S: States>(app: &mut App) {
    app.add_systems(Update, trigger_exit_anims::<S>);
}

fn trigger_exit_anims<S: States>(
    mut commands: Commands,
    state: Option<Res<State<S>>>,
    query: Query<(Entity, &AnimDespawnOnExit<S>), Without<IsDespawning>>,
) {
    let Some(state) = state else { return };
    for (entity, scoped) in &query {
        if *state.get() == scoped.0 {
            continue;
        }

        let tween = Tween::new(
            EaseFunction::QuadraticIn,
            Duration::from_millis(200),
            TransformScaleLens {
                start: Vec3::ONE,
                end: Vec3::ZERO,
            },
        );

        commands.spawn((
            TweenAnim::new(tween),
            AnimTarget::component::<Transform>(entity),
        ));
        commands.entity(entity).insert(IsDespawning);
    }
}

fn despawn_on_anim_complete(
    trigger: On<AnimCompletedEvent>,
    despawning: Query<Entity, With<IsDespawning>>,
    mut commands: Commands,
) {
    let AnimTargetKind::Component { entity } = trigger.event().target else {
        return;
    };
    if despawning.contains(entity) {
        commands.entity(entity).despawn();
    }
}
