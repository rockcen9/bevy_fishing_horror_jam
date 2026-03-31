use crate::prelude::*;
use bevy::picking::prelude::*;

use super::last_item::{DISPLAY_GAP, DISPLAY_SIZE};
use super::{Backpack, BACKPACK_HEIGHT, BACKPACK_WIDTH, PADDING};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_backpack);
}

fn spawn_backpack(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("textures/backpack.png");

    let x = -960.0 + BACKPACK_WIDTH / 2.0 + PADDING;
    let y = -250.0 + DISPLAY_SIZE / 2.0 + DISPLAY_GAP + BACKPACK_HEIGHT / 2.0;

    commands.spawn((
        Name::new("Backpack"),
        Backpack,
        SpriteLayer::Backpack,
        Mesh2d(meshes.add(Rectangle::new(BACKPACK_WIDTH, BACKPACK_HEIGHT))),
        MeshMaterial2d(materials.add(ColorMaterial {
            texture: Some(texture),
            ..default()
        })),
        Transform::from_xyz(x, y, 10.0),
        Pickable::default(),
    ));
}
