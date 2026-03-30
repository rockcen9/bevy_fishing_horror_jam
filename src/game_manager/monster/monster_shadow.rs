use super::bubble::MonsterShadowSpawnPoint;
use crate::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, lens::TransformScaleLens, *};
use std::time::Duration;

// ── Constants ───────────────────────────────────────────────────────────────

/// Vertical float amplitude in pixels.
const FLOAT_AMPLITUDE: f32 = 8.0;
/// Duration of one half-cycle (up or down) in seconds.
const FLOAT_HALF_PERIOD_SECS: f32 = 1.5;
/// Size of the monster shadow image in pixels.
const SHADOW_SIZE: Vec2 = Vec2::new(330.0, 443.0);
/// Uniform scale applied to the monster shadow.
const SHADOW_SCALE: f32 = 0.6;
/// Duration of the despawn scale-out animation in seconds.
const DESPAWN_ANIM_SECS: f32 = 0.2;

// ── Components ──────────────────────────────────────────────────────────────

#[derive(Component, Reflect)]
pub struct MonsterShadow;

#[derive(Resource)]
struct MonsterShadowDespawnTimer(Timer);

// ── Plugin ──────────────────────────────────────────────────────────────────

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MonsterState::Shadow), spawn_monster_shadow);
    app.add_systems(OnExit(MonsterState::Shadow), begin_shadow_despawn);
    app.add_systems(Update, tick_shadow_despawn);
    #[cfg(feature = "dev")]
    app.add_systems(Update, draw_monster_gizmos);
}

// ── Systems ─────────────────────────────────────────────────────────────────

fn spawn_monster_shadow(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    spawn_point: Res<MonsterShadowSpawnPoint>,
) {
    let x = spawn_point.x;
    let y = spawn_point.y;

    let texture = asset_server.load("textures/monster_shadow.png");
    let material = materials.add(ColorMaterial {
        texture: Some(texture),
        ..default()
    });
    let mesh = meshes.add(Rectangle::new(SHADOW_SIZE.x, SHADOW_SIZE.y));

    let shadow_entity = commands
        .spawn((
            Name::new("Monster Shadow"),
            MonsterShadow,
            SpriteLayer::Monster,
            Mesh2d(mesh),
            MeshMaterial2d(material),
            Transform::from_xyz(x, y, 0.0).with_scale(Vec3::splat(SHADOW_SCALE)),
        ))
        .id();

    let float_tween = Tween::new(
        EaseFunction::SineInOut,
        Duration::from_secs_f32(FLOAT_HALF_PERIOD_SECS),
        TransformPositionLens {
            start: Vec3::new(x, y - FLOAT_AMPLITUDE, 0.0),
            end: Vec3::new(x, y + FLOAT_AMPLITUDE, 0.0),
        },
    )
    .with_repeat(RepeatCount::Infinite, RepeatStrategy::MirroredRepeat);

    commands.spawn((
        DespawnOnExit(MonsterState::Shadow),
        TweenAnim::new(float_tween),
        AnimTarget::component::<Transform>(shadow_entity),
    ));
}

fn begin_shadow_despawn(shadow: Query<Entity, With<MonsterShadow>>, mut commands: Commands) {
    let Ok(entity) = shadow.single() else {
        return;
    };

    let tween = Tween::new(
        EaseFunction::QuadraticIn,
        Duration::from_secs_f32(DESPAWN_ANIM_SECS),
        TransformScaleLens {
            start: Vec3::splat(SHADOW_SCALE),
            end: Vec3::ZERO,
        },
    );

    commands.spawn((
        TweenAnim::new(tween),
        AnimTarget::component::<Transform>(entity),
    ));

    commands.insert_resource(MonsterShadowDespawnTimer(Timer::from_seconds(
        DESPAWN_ANIM_SECS,
        TimerMode::Once,
    )));
}

fn tick_shadow_despawn(
    time: Res<Time>,
    timer: Option<ResMut<MonsterShadowDespawnTimer>>,
    shadow: Query<Entity, With<MonsterShadow>>,
    mut commands: Commands,
) {
    let Some(mut timer) = timer else {
        return;
    };

    if timer.0.tick(time.delta()).just_finished() {
        commands.remove_resource::<MonsterShadowDespawnTimer>();
        for entity in shadow.iter() {
            commands.entity(entity).despawn();
        }
    }
}

#[cfg(feature = "dev")]
fn draw_monster_gizmos(query: Query<&GlobalTransform, With<MonsterShadow>>, mut gizmos: Gizmos) {
    for gt in &query {
        let pos = gt.translation().truncate();
        gizmos.circle_2d(pos, 50.0, Color::srgb(1.0, 0.0, 0.0));
        gizmos.cross_2d(pos, 60.0, Color::srgb(1.0, 1.0, 0.0));
    }
}

