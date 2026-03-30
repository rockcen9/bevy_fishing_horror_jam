pub(crate) mod bubble;

use crate::prelude::*;
use bevy::{
    math::Vec4,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin, MeshMaterial2d},
};

pub(crate) use bubble::{StartBubbleEvent, StopBubbleEvent};

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<LakeWaterMaterial>::default());
    app.add_plugins(bubble::plugin);
    app.add_systems(OnEnter(Screen::Gameplay), spawn_background_sprite);
    app.add_systems(OnExit(Screen::Gameplay), cleanup_material_handle);
}

// ── Material ──────────────────────────────────────────────────────────────────

/// The UV-Y coordinate where the lake begins (0 = top, 1 = bottom).
const LAKE_START_Y: f32 = 0.57;

/// Positions for up to 8 bubble columns on the lake surface.
/// Set `count` to 0 to disable the bubble effect entirely.
/// Each active entry's `x` component is the UV-x of the bubble column (0.0–1.0).
#[derive(ShaderType, Debug, Clone)]
pub(crate) struct BubbleParams {
    /// UV-x positions; only `sources[0..count]` are used. `.yzw` is padding.
    pub sources: [Vec4; 8],
    /// Number of active bubble columns. 0 = effect disabled.
    pub count: u32,
}

impl Default for BubbleParams {
    fn default() -> Self {
        Self {
            sources: [Vec4::new(0.5, 0.0, 0.0, 0.0); 8],
            count: 0,
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub(crate) struct LakeWaterMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
    #[uniform(2)]
    pub lake_start_y: f32,
    #[uniform(3)]
    pub bubble_params: BubbleParams,
}

impl Material2d for LakeWaterMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/background.wgsl".into()
    }
}

// ── Internal resource ─────────────────────────────────────────────────────────

#[derive(Resource)]
pub(super) struct BackgroundMaterialHandle(pub Handle<LakeWaterMaterial>);

// ── Spawn / Cleanup ───────────────────────────────────────────────────────────

fn spawn_background_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LakeWaterMaterial>>,
) {
    let texture = asset_server.load("background/night.png");
    let handle = materials.add(LakeWaterMaterial {
        texture,
        lake_start_y: LAKE_START_Y,
        bubble_params: BubbleParams::default(),
    });
    let mesh = meshes.add(Rectangle::new(crate::GAME_WIDTH, crate::GAME_HEIGHT));

    commands.insert_resource(BackgroundMaterialHandle(handle.clone()));

    commands.spawn((
        Name::new("Background"),
        SpriteLayer::Background,
        Mesh2d(mesh),
        MeshMaterial2d(handle),
        Transform::from_xyz(0., 0., -100.),
        DespawnOnExit(Screen::Gameplay),
    ));
}

fn cleanup_material_handle(mut commands: Commands) {
    commands.remove_resource::<BackgroundMaterialHandle>();
}
