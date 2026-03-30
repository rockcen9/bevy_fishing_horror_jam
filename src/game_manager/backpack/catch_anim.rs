use crate::game_manager::balance::ItemDataBalance;
use crate::game_manager::pole::{Bobber, HookedFish};
use crate::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, *};
use std::time::Duration;

use super::{Backpack, BACKPACK_HEIGHT, BACKPACK_WIDTH, PADDING};

const CATCH_ANIM_DURATION_MS: u64 = 200;
const FISH_SPRITE_SIZE: f32 = 64.0;
/// Extra time after animation ends before despawning.
const DESPAWN_DELAY_MS: u64 = 50;

#[derive(Component)]
struct CatchAnimationFish;

#[derive(Resource)]
struct CatchAnimationDespawnTimer(Timer);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Succeeding), spawn_fish_catch_animation);
    app.add_systems(
        Update,
        tick_catch_animation_despawn_timer.run_if(in_state(GameState::Succeeding)),
    );
    app.add_systems(OnExit(GameState::Succeeding), despawn_catch_animation);
}

fn spawn_fish_catch_animation(
    bobber: Query<(&Transform, &HookedFish), With<Bobber>>,
    backpack: Query<&Transform, With<Backpack>>,
    balance: Res<ItemDataBalance>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let Ok((bobber_transform, hooked_fish)) = bobber.single() else {
        return;
    };
    let Some(item_data) = balance.get_by_id(&hooked_fish.0 .0) else {
        return;
    };

    let start = Vec3::new(
        bobber_transform.translation.x,
        bobber_transform.translation.y,
        1000.0,
    );
    let backpack_pos = backpack
        .single()
        .map(|t| t.translation)
        .unwrap_or_else(|_| Vec3::new(-960.0 + BACKPACK_WIDTH / 2.0 + PADDING, -540.0 + BACKPACK_HEIGHT / 2.0 + PADDING, 0.0));
    let end = Vec3::new(backpack_pos.x, backpack_pos.y, 1000.0);

    let texture: Handle<Image> = asset_server.load(item_data.file_path.clone());
    let fish_entity = commands
        .spawn((
            Name::new("CatchAnimFish"),
            CatchAnimationFish,
            Sprite {
                image: texture,
                custom_size: Some(Vec2::splat(FISH_SPRITE_SIZE)),
                ..default()
            },
            Transform::from_translation(start),
        ))
        .id();

    let tween = Tween::new(
        EaseFunction::QuadraticIn,
        Duration::from_millis(CATCH_ANIM_DURATION_MS),
        TransformPositionLens { start, end },
    );
    commands.spawn((TweenAnim::new(tween), AnimTarget::component::<Transform>(fish_entity)));

    commands.insert_resource(CatchAnimationDespawnTimer(Timer::from_seconds(
        (CATCH_ANIM_DURATION_MS + DESPAWN_DELAY_MS) as f32 / 1000.0,
        TimerMode::Once,
    )));
}

fn tick_catch_animation_despawn_timer(
    time: Res<Time>,
    timer: Option<ResMut<CatchAnimationDespawnTimer>>,
    q_fish: Query<Entity, With<CatchAnimationFish>>,
    mut commands: Commands,
) {
    let Some(mut timer) = timer else {
        return;
    };
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    commands.remove_resource::<CatchAnimationDespawnTimer>();
    for e in q_fish.iter() {
        commands.entity(e).despawn();
    }
}

fn despawn_catch_animation(
    q_fish: Query<Entity, With<CatchAnimationFish>>,
    mut commands: Commands,
) {
    for e in q_fish.iter() {
        commands.entity(e).despawn();
    }
    commands.remove_resource::<CatchAnimationDespawnTimer>();
}
