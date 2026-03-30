use crate::loading_bar::LoadingBar;
use crate::prelude::*;
use bevy_tweening::{lens::TransformScaleLens, *};
use std::time::Duration;

use super::Backpack;
use super::last_item::{LastItemDisplay, LastItemLoadingBar};

const BACKPACK_HIDE_ANIM_DURATION_MS: u64 = 200;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Monster), on_monster_attack_hide_backpack)
        .add_systems(OnExit(GameState::Monster), on_monster_peace_show_backpack);
}

fn on_monster_attack_hide_backpack(
    backpack: Query<Entity, With<Backpack>>,
    last_item: Query<Entity, With<LastItemDisplay>>,
    mut backpack_lb: Query<&mut Visibility, (With<LoadingBar>, Without<LastItemLoadingBar>)>,
    mut last_item_lb: Query<&mut Visibility, With<LastItemLoadingBar>>,
    mut commands: Commands,
) {
    for entity in backpack.iter().chain(last_item.iter()) {
        let tween = Tween::new(
            EaseFunction::QuadraticIn,
            Duration::from_millis(BACKPACK_HIDE_ANIM_DURATION_MS),
            TransformScaleLens {
                start: Vec3::ONE,
                end: Vec3::ZERO,
            },
        );
        commands.spawn((
            TweenAnim::new(tween),
            AnimTarget::component::<Transform>(entity),
        ));
    }
    for mut vis in &mut backpack_lb {
        vis.set_if_neq(Visibility::Hidden);
    }
    for mut vis in &mut last_item_lb {
        vis.set_if_neq(Visibility::Hidden);
    }
}

fn on_monster_peace_show_backpack(
    backpack: Query<Entity, With<Backpack>>,
    last_item: Query<Entity, With<LastItemDisplay>>,
    mut commands: Commands,
) {
    for entity in backpack.iter().chain(last_item.iter()) {
        let tween = Tween::new(
            EaseFunction::QuadraticOut,
            Duration::from_millis(BACKPACK_HIDE_ANIM_DURATION_MS),
            TransformScaleLens {
                start: Vec3::ZERO,
                end: Vec3::ONE,
            },
        );
        commands.spawn((
            TweenAnim::new(tween),
            AnimTarget::component::<Transform>(entity),
        ));
    }
}
