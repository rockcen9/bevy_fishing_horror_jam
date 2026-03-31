use crate::loading_bar::{LoadingBar, LoadingBarMaterial, LOADING_BAR_RING_SIZE_PX};
use crate::prelude::*;

use super::last_item::{DISPLAY_GAP, DISPLAY_SIZE};
use super::{BACKPACK_HEIGHT, BACKPACK_WIDTH, PADDING};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_backpack_hover_loading_bar);
}

fn spawn_backpack_hover_loading_bar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LoadingBarMaterial>>,
) {
    let x = -960.0 + BACKPACK_WIDTH / 2.0 + PADDING;
    let y = -250.0 + DISPLAY_SIZE / 2.0 + DISPLAY_GAP + BACKPACK_HEIGHT / 2.0;

    commands.spawn((
        Name::new("BackpackLoadingBar"),
        LoadingBar,
        SpriteLayer::LoadingBar,
        Mesh2d(meshes.add(Rectangle::new(LOADING_BAR_RING_SIZE_PX, LOADING_BAR_RING_SIZE_PX))),
        MeshMaterial2d(materials.add(LoadingBarMaterial::default())),
        Transform::from_xyz(x, y, 45.0),
        Visibility::Hidden,
    ));
}
