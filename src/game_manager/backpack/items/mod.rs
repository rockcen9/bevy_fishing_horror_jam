#[cfg(feature = "dev")]
mod debug_buttons;
mod random_color_material;
pub use random_color_material::HueShiftFishMaterial;

use crate::game_manager::balance::ItemDataBalance;
use crate::prelude::*;
/// Marker component shared by all backpack items.
#[derive(Component)]
#[require(SpriteLayer::Item)]
pub struct Item;

#[derive(Component)]
pub struct PrefabId(pub smol_str::SmolStr);

#[derive(Debug, strum_macros::Display, strum_macros::EnumString)]
#[strum(serialize_all = "snake_case")]

pub enum PrefabList {
    Journal,
    BitBass,
    CodeE,
    FinTech,
    SideSwimmer,
    DeepLearn,
    MissEel,
    DepthCarp,
    ReefEr,
    // #[serde(rename = "howitz-perch")]
    HowitzPerch,
    Target1,
    Target2,
    Target3,
    Kai,
}

impl PrefabList {
    pub fn prefab_id(&self) -> PrefabId {
        PrefabId(self.to_string().into())
    }
}

const ITEM_SIZE: f32 = 128.0;

pub(crate) fn plugin(app: &mut bevy::prelude::App) {
    #[cfg(feature = "dev")]
    debug_buttons::plugin(app);
    random_color_material::plugin(app);
    app.add_systems(
        Update,
        insert_mesh_onto_prefab_items.run_if(resource_exists::<ItemDataBalance>),
    );
}

pub(crate) fn hue_shift_from_entity(entity: Entity) -> f32 {
    // Splitmix64 hash — scrambles entity bits into a uniform float in [0, 1).
    let mut x = entity.to_bits() ^ 0x9e3779b97f4a7c15;
    x = x.wrapping_mul(0xbf58476d1ce4e5b9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94d049bb133111eb);
    x ^= x >> 31;
    (x >> 11) as f32 / (1u64 << 53) as f32
}

fn insert_mesh_onto_prefab_items(
    items: Query<(Entity, &PrefabId), Without<Mesh2d>>,
    balance: Res<ItemDataBalance>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut random_color_materials: ResMut<Assets<HueShiftFishMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, prefab_id) in &items {
        let Some(item_data) = balance.get_by_id(&prefab_id.0) else {
            warn!("No item data found for id: {}", prefab_id.0);
            continue;
        };
        let (item_w, item_h) = (ITEM_SIZE, ITEM_SIZE);
        let mesh = Mesh2d(meshes.add(Rectangle::new(item_w, item_h)));
        let texture: Handle<Image> = asset_server.load(item_data.file_path.clone());

        if item_data.random_color {
            let material = random_color_materials.add(HueShiftFishMaterial {
                texture,
                hue_shift: hue_shift_from_entity(entity),
            });
            commands
                .entity(entity)
                .insert((mesh, MeshMaterial2d(material)));
        } else {
            let material = materials.add(ColorMaterial {
                texture: Some(texture),
                ..default()
            });
            commands
                .entity(entity)
                .insert((mesh, MeshMaterial2d(material)));
        }
    }
}
