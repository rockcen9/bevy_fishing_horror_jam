//! Validates that all assets are preloaded before the game starts.
//! Observers fire whenever a component is added to an entity and warn
//! if the referenced asset was not already loaded.

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(validate_sprite);
}

fn validate_sprite(add: On<Add, Sprite>, q: Query<&Sprite>, assets: Res<AssetServer>) {
    let sprite = q.get(add.entity).unwrap();
    validate_asset(&sprite.image, &assets, "Sprite");
}

fn validate_asset<T: Asset>(handle: &Handle<T>, assets: &AssetServer, type_name: &str) {
    let Some(path) = handle.path() else {
        return;
    };
    if !assets.is_loaded_with_dependencies(handle) {
        warn!("{type_name} at path \"{path}\" was not preloaded and will load during gameplay.");
    }
}
