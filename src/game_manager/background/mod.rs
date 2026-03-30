use crate::prelude::*;
use bevy::{
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin, MeshMaterial2d},
};

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<LakeWaterMaterial>::default());
    app.add_systems(OnEnter(Screen::Gameplay), spawn_background_sprite);
}

// ── Material ──────────────────────────────────────────────────────────────────

/// The UV-Y coordinate where the lake begins (0 = top, 1 = bottom).
/// Adjust if the shoreline moves after an art update.
const LAKE_START_Y: f32 = 0.57;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct LakeWaterMaterial {
    #[texture(0)]
    #[sampler(1)]
    texture: Handle<Image>,
    #[uniform(2)]
    lake_start_y: f32,
}

impl Material2d for LakeWaterMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/background.wgsl".into()
    }
}

// ── Spawn ─────────────────────────────────────────────────────────────────────

fn spawn_background_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LakeWaterMaterial>>,
) {
    let texture = asset_server.load("background/night.png");
    let material = materials.add(LakeWaterMaterial {
        texture,
        lake_start_y: LAKE_START_Y,
    });
    let mesh = meshes.add(Rectangle::new(crate::GAME_WIDTH, crate::GAME_HEIGHT));

    commands.spawn((
        Name::new("Background"),
        SpriteLayer::Background,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(0., 0., -100.),
        DespawnOnExit(Screen::Gameplay),
    ));
}
