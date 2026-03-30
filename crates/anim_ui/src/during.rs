use bevy::prelude::*;

use crate::{AnimDespawnOnExit, AnimSpawnOn};

/// Attach to an entity to automatically play a scale-in entrance animation
/// and a scale-out despawn animation when the active state no longer matches.
///
/// Requires [`AnimUiPlugin::with_state::<S>()`](crate::AnimUiPlugin) to be
/// registered for the relevant state type.
///
/// # Example
/// ```rust
/// commands.spawn((my_sprite, AnimDuring(GameState::Playing)));
/// ```
#[derive(Component)]
pub struct AnimDuring<S: States>(pub S);

pub(super) fn register<S: States>(app: &mut App) {
    app.add_systems(Update, inject_anim_during::<S>);
}

fn inject_anim_during<S: States>(
    mut commands: Commands,
    query: Query<(Entity, &AnimDuring<S>), Added<AnimDuring<S>>>,
) {
    for (entity, during) in &query {
        commands.entity(entity).insert((
            AnimSpawnOn,
            AnimDespawnOnExit(during.0.clone()),
        ));
    }
}
