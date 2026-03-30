use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use serde::Deserialize;
use smol_str::SmolStr;

#[derive(Deserialize, Asset, TypePath, Debug)]
#[serde(transparent)]
pub struct RawItemDataList(pub Vec<ItemData>);

#[derive(Deserialize, Debug, Clone, Reflect)]
pub struct ItemData {
    pub id: String,
    pub random_color: bool,
    pub file_path: String,
}

#[derive(Resource)]
pub struct ItemsJsonHandle(Handle<RawItemDataList>);

#[derive(Resource, Debug, Deref, Reflect)]
pub struct ItemDataBalance(pub Vec<ItemData>);

impl ItemDataBalance {
    pub fn get_by_id(&self, id: &SmolStr) -> Option<&ItemData> {
        self.0.iter().find(|i| i.id == id.as_str())
    }
}

pub fn plugin(app: &mut App) {
    app.add_plugins(JsonAssetPlugin::<RawItemDataList>::new(&["items.json"]));
    app.add_systems(Startup, start_loading_items_json);
    app.add_systems(
        Update,
        promote_items_json_to_resource.run_if(not(resource_exists::<ItemDataBalance>)),
    );
}

fn start_loading_items_json(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<RawItemDataList> = asset_server.load("balance/items.json");
    commands.insert_resource(ItemsJsonHandle(handle));
}

fn promote_items_json_to_resource(
    mut commands: Commands,
    handle: Option<Res<ItemsJsonHandle>>,
    assets: Res<Assets<RawItemDataList>>,
) {
    let Some(handle) = handle else { return };
    let Some(list) = assets.get(&handle.0) else {
        return;
    };
    commands.insert_resource(ItemDataBalance(list.0.clone()));
    commands.remove_resource::<ItemsJsonHandle>();
    info!("Loaded {} item data entries from items.json", list.0.len());
}
