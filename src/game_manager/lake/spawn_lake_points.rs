use crate::prelude::*;

/// Fixed world positions for the three lake fishing points.
pub const LAKE_POINT_1: Vec2 = Vec2::new(-600.0, -100.0);
pub const LAKE_POINT_2: Vec2 = Vec2::new(-100.0, -100.0);
pub const LAKE_POINT_3: Vec2 = Vec2::new(400.0, -100.0);

/// Marker component for lake fishing point 1.
#[derive(Component, Reflect, Default)]
pub struct LakePoint1;

/// Marker component for lake fishing point 2.
#[derive(Component, Reflect, Default)]
pub struct LakePoint2;

/// Marker component for lake fishing point 3.
#[derive(Component, Reflect, Default)]
pub struct LakePoint3;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_lake_points);
    #[cfg(feature = "dev")]
    app.add_systems(Update, draw_lake_points_gizmos);
}

fn spawn_lake_points(mut commands: Commands) {
    commands.spawn((
        Name::new("LakePoint1"),
        LakePoint1,
        Transform::from_xyz(LAKE_POINT_1.x, LAKE_POINT_1.y, 0.0),
        GlobalTransform::default(),
        DespawnOnExit(Screen::Gameplay),
    ));
    commands.spawn((
        Name::new("LakePoint2"),
        LakePoint2,
        Transform::from_xyz(LAKE_POINT_2.x, LAKE_POINT_2.y, 0.0),
        GlobalTransform::default(),
        DespawnOnExit(Screen::Gameplay),
    ));
    commands.spawn((
        Name::new("LakePoint3"),
        LakePoint3,
        Transform::from_xyz(LAKE_POINT_3.x, LAKE_POINT_3.y, 0.0),
        GlobalTransform::default(),
        DespawnOnExit(Screen::Gameplay),
    ));
}

#[cfg(feature = "dev")]
fn draw_lake_points_gizmos(
    mut gizmos: Gizmos,
    q1: Query<&GlobalTransform, With<LakePoint1>>,
    q2: Query<&GlobalTransform, With<LakePoint2>>,
    q3: Query<&GlobalTransform, With<LakePoint3>>,
) {
    let color = bevy::color::Color::srgb(0.0, 0.8, 1.0);
    let radius = 20.0;

    for gt in q1.iter().chain(q2.iter()).chain(q3.iter()) {
        let pos = gt.translation().truncate();
        gizmos.circle_2d(pos, radius, color);
        gizmos.line_2d(
            pos - Vec2::new(radius, 0.0),
            pos + Vec2::new(radius, 0.0),
            color,
        );
        gizmos.line_2d(
            pos - Vec2::new(0.0, radius),
            pos + Vec2::new(0.0, radius),
            color,
        );
    }
}
