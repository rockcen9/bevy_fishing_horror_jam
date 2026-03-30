use crate::game_manager::player::PlayerHealth;
use crate::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, *};
// use camera_effects::CameraShakeEvent;
use kira_ext::SFXEvent;
use std::time::Duration;

// ── Constants ───────────────────────────────────────────────────────────────

const MONSTER2_DELAY_SECS: f32 = 0.4;
const MONSTER2_LINGER_SECS: f32 = 2.0;
const MONSTER2_FALL_SECS: f32 = 0.4;
const MONSTER2_FALL_END_Y: f32 = -1200.0;

// ── Resources ───────────────────────────────────────────────────────────────

#[derive(Resource)]
struct AttackSide {
    /// X position on screen where the attack sprites are spawned.
    x: f32,
}

#[derive(Resource)]
struct Monster2SpawnTimer(Timer);

#[derive(Resource)]
struct Monster2LingerTimer(Timer);

// ── Components ──────────────────────────────────────────────────────────────

#[derive(Component)]
struct AttackSprite;

#[derive(Component)]
struct Monster1Sprite;

// ── Plugin ──────────────────────────────────────────────────────────────────

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MonsterState::Attack), on_enter);
    app.add_systems(OnExit(MonsterState::Attack), on_exit);
    app.add_systems(
        Update,
        (tick_monster2_spawn_timer, tick_monster2_linger_timer)
            .run_if(in_state(MonsterState::Attack)),
    );
}

// ── Systems ─────────────────────────────────────────────────────────────────

fn on_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    shadow: Query<&Transform, With<super::monster_shadow::MonsterShadow>>,
) {
    let x = shadow.single().map(|t| t.translation.x).unwrap_or(0.0);

    commands.insert_resource(AttackSide { x });
    commands.insert_resource(Monster2SpawnTimer(Timer::from_seconds(
        MONSTER2_DELAY_SECS,
        TimerMode::Once,
    )));

    commands.trigger(SFXEvent::sfx("pull_up_succeed"));
    commands.spawn((
        Name::new("Monster Attack Sprite"),
        AttackSprite,
        Monster1Sprite,
        SpriteLayer::Monster,
        Sprite::from_image(asset_server.load("textures/monster.png")),
        Transform::from_xyz(x, 0.0, 0.0).with_scale(Vec3::splat(1.0)),
    ));
}

fn tick_monster2_spawn_timer(
    time: Res<Time>,
    mut timer: ResMut<Monster2SpawnTimer>,
    side: Res<AttackSide>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut health: ResMut<PlayerHealth>,
    monster1: Query<Entity, With<Monster1Sprite>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    if let Ok(entity) = monster1.single() {
        commands.entity(entity).despawn();
    }
    commands.trigger(SFXEvent::sfx("sq"));
    let monster2_entity = commands
        .spawn((
            Name::new("Monster Attack Sprite 2"),
            AttackSprite,
            SpriteLayer::Monster,
            Sprite::from_image(asset_server.load("textures/monster2.png")),
            Transform::from_xyz(side.x, 0.0, 0.0).with_scale(Vec3::splat(1.5)),
        ))
        .id();

    let fall_tween = Tween::new(
        EaseFunction::QuadraticIn,
        Duration::from_secs_f32(MONSTER2_FALL_SECS),
        TransformPositionLens {
            start: Vec3::new(side.x, 0.0, 0.0),
            end: Vec3::new(side.x, MONSTER2_FALL_END_Y, 0.0),
        },
    );
    commands.spawn((
        AttackSprite,
        TweenAnim::new(fall_tween),
        AnimTarget::component::<Transform>(monster2_entity),
    ));

    let new_health = health.value - 10.0;
    health.set(new_health);

    commands.insert_resource(Monster2LingerTimer(Timer::from_seconds(
        MONSTER2_LINGER_SECS,
        TimerMode::Once,
    )));
}

fn tick_monster2_linger_timer(
    time: Res<Time>,
    timer: Option<ResMut<Monster2LingerTimer>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some(mut timer) = timer else { return };
    if timer.0.tick(time.delta()).just_finished() {
        next_state.set(GameState::Idle);
    }
}

fn on_exit(mut commands: Commands, sprites: Query<Entity, With<AttackSprite>>) {
    commands.remove_resource::<AttackSide>();
    commands.remove_resource::<Monster2SpawnTimer>();
    commands.remove_resource::<Monster2LingerTimer>();
    for entity in &sprites {
        commands.entity(entity).despawn();
    }
}
