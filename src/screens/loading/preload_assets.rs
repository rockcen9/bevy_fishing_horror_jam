//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::prelude::*;

use crate::prelude::*;
use crate::{asset_tracking::ResourceHandles, theme::prelude::*};

use super::LoadingScreen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(LoadingScreen::Assets),
        spawn_asset_loading_screen_or_advance,
    );

    //todo
    app.add_systems(
        Update,
        (
            sync_loading_assets_label_text,
            advance_to_level_loading_screen.run_if(is_all_assets_loaded.and_then(in_state(LoadingScreen::Assets))),
        ),
    );
}
fn advance_to_level_loading_screen(mut next_screen: ResMut<NextState<LoadingScreen>>) {
    next_screen.set(LoadingScreen::Level);
}

fn spawn_asset_loading_screen_or_advance(
    mut commands: Commands,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<LoadingScreen>>,
    palette: Res<ColorPalette>,
    asset_server: Res<AssetServer>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(LoadingScreen::Level);
        return;
    }
    commands.spawn((
        widget::ui_root("Loading Screen"),
        BackgroundColor(palette.get(UiColorName::ScreenBackground)),
        DespawnOnExit(LoadingScreen::Assets),
        children![(
            widget::label("Loading Assets", &palette, &asset_server),
            LoadingAssetsLabel
        )],
    ));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct LoadingAssetsLabel;

fn sync_loading_assets_label_text(
    mut query: Query<&mut Text, With<LoadingAssetsLabel>>,
    resource_handles: Res<ResourceHandles>,
) {
    for mut text in query.iter_mut() {
        text.0 = format!(
            "Loading Assets: {} / {}",
            resource_handles.finished_count(),
            resource_handles.total_count()
        );
    }
}
fn is_all_assets_loaded(resource_handles: Res<ResourceHandles>) -> bool {
    resource_handles.is_all_done()
}
