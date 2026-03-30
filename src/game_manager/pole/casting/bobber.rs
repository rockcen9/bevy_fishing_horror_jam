use super::forward::CastReturnTimer;
use crate::prelude::*;

#[derive(Component)]
pub struct Bobber;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, spawn_bobber_at_lake_point.run_if(resource_added::<CastReturnTimer>).run_if(in_state(Pause(false))));
    app.add_systems(OnEnter(GameState::Idle), despawn_bobber);
}

fn spawn_bobber_at_lake_point(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    lake_point_query: Query<&GlobalTransform, With<LakePoint1>>,
) {
    let Some(lake_transform) = lake_point_query.iter().next() else {
        return;
    };

    let pos = lake_transform.translation();

    commands.spawn((
        Name::new("Bobber"),
        Bobber,
        SpriteLayer::Bobber,
        Sprite {
            image: asset_server.load("textures/bobber.png"),
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        Transform::from_xyz(pos.x, pos.y, 5.0),
    ));
}

fn despawn_bobber(bobber_query: Query<Entity, With<Bobber>>, mut commands: Commands) {
    for entity in bobber_query.iter() {
        commands.entity(entity).despawn();
    }
}
