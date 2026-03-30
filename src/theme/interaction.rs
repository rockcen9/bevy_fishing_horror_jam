use bevy::prelude::*;
use bevy_tweening::{lens::UiTransformTranslationPxLens, *};
use std::time::Duration;

use crate::PostPhysicsAppSystems;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (trigger_on_press, apply_interaction_palette).in_set(PostPhysicsAppSystems::ChangeUi),
    );
}

/// Palette for widget interactions. Add this to an entity that supports
/// [`Interaction`]s, such as a button, to change its [`BackgroundColor`] based
/// on the current interaction state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub(crate) struct InteractionPalette {
    pub(crate) none: Color,
    pub(crate) hovered: Color,
    pub(crate) pressed: Color,
}

/// Event triggered on a UI entity when the [`Interaction`] component on the same entity changes to
/// [`Interaction::Pressed`]. Observe this event to detect e.g. button presses.
#[derive(EntityEvent)]
pub(crate) struct OnPress {
    pub(crate) entity: Entity,
}

fn trigger_on_press(
    interaction_query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (entity, interaction) in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            commands.trigger(OnPress { entity });
        }
    }
}

fn apply_interaction_palette(
    mut palette_query: Query<
        (&Interaction, &InteractionPalette, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, palette, mut background) in &mut palette_query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}

/// Apply hover lift effect: buttons float up slightly when hovered
/// NOTE: This is currently disabled in favor of per-menu hover systems.
/// See menus/main.rs for an example of how to add hover lift to specific buttons.
#[allow(dead_code)]
fn apply_hover_lift(
    mut commands: Commands,
    button_query: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<InteractionPalette>),
    >,
) {
    const LIFT_DISTANCE: f32 = -4.0; // Negative Y means up in UI coordinates
    const ANIMATION_DURATION_MS: u64 = 150;

    for (entity, interaction) in &button_query {
        match *interaction {
            Interaction::Hovered => {
                let tween = Tween::new(
                    EaseFunction::QuadraticOut,
                    Duration::from_millis(ANIMATION_DURATION_MS),
                    UiTransformTranslationPxLens {
                        start: Vec2::ZERO,
                        end: Vec2::new(0.0, LIFT_DISTANCE),
                    },
                );
                commands.spawn((
                    TweenAnim::new(tween),
                    AnimTarget::component::<UiTransform>(entity),
                ));
            }
            Interaction::None | Interaction::Pressed => {
                let tween = Tween::new(
                    EaseFunction::QuadraticIn,
                    Duration::from_millis(ANIMATION_DURATION_MS),
                    UiTransformTranslationPxLens {
                        start: Vec2::new(0.0, LIFT_DISTANCE),
                        end: Vec2::ZERO,
                    },
                );
                commands.spawn((
                    TweenAnim::new(tween),
                    AnimTarget::component::<UiTransform>(entity),
                ));
            }
        }
    }
}

