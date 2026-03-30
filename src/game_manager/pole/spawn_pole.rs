use bevy::{
    asset::RenderAssetUsages,
    render::{
        mesh::Indices,
        render_resource::{AsBindGroup, PrimitiveTopology},
    },
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

use crate::prelude::*;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct PoleMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
    /// x = signed horizontal bend in local pixels (positive = right, negative = left).
    /// Displacement grows quadratically from handle (0) to tip (max).
    #[uniform(2)]
    pub bend_params: Vec4,
}

impl Material2d for PoleMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/pole.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/pole.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

#[derive(Component, Reflect)]
pub struct Pole;

/// Marker for the tip of the fishing pole (world-space top).
#[derive(Component, Reflect)]
pub struct PoleTip;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<PoleMaterial>::default());
    app.add_systems(OnEnter(Screen::Gameplay), spawn_pole);
    #[cfg(feature = "dev")]
    app.add_systems(Update, draw_pole_tip_gizmo.in_set(PausableSystems));
}

/// Builds a subdivided rectangular mesh oriented along the Y axis.
///
/// `v_segments` controls how many horizontal slices the pole has.
/// More slices = smoother curve from the vertex shader bend.
/// UV: (0,0) at top-left (tip), (1,1) at bottom-right (handle) — matches Bevy's convention
/// so that `t = 1 - uv.y` is 0 at the handle and 1 at the tip.
fn build_subdivided_pole_mesh(width: f32, height: f32, v_segments: u32) -> Mesh {
    let half_w = width / 2.0;
    let half_h = height / 2.0;

    let vertex_rows = v_segments + 1;
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity((vertex_rows * 2) as usize);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity((vertex_rows * 2) as usize);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity((vertex_rows * 2) as usize);
    let mut indices: Vec<u32> = Vec::with_capacity((v_segments * 6) as usize);

    for row in 0..=v_segments {
        let t = row as f32 / v_segments as f32;
        let y = -half_h + t * height;
        // UV.y = 1 at bottom (handle, row 0), 0 at top (tip, last row).
        let uv_y = 1.0 - t;

        positions.push([-half_w, y, 0.0]); // left
        positions.push([half_w, y, 0.0]); // right
        normals.push([0.0, 0.0, 1.0]);
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([0.0, uv_y]);
        uvs.push([1.0, uv_y]);
    }

    // Two CCW triangles per quad (viewed from +Z).
    for row in 0..v_segments {
        let bl = row * 2; // bottom-left
        let br = row * 2 + 1; // bottom-right
        let tl = row * 2 + 2; // top-left
        let tr = row * 2 + 3; // top-right
        indices.extend_from_slice(&[bl, tl, tr, bl, tr, br]);
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}

fn spawn_pole(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PoleMaterial>>,
) {
    let texture = asset_server.load("textures/pole.png");
    // pole texture is 124x1936 pixels; mesh is half-res: 61x966.
    // 20 Y-segments = 21 rows of vertices so the vertex shader can produce a smooth curve.
    let mesh = meshes.add(build_subdivided_pole_mesh(61., 966., 20));
    let material = materials.add(PoleMaterial {
        texture,
        bend_params: Vec4::ZERO,
    });

    // Root entity sits at the handle (bottom). Rotating it pivots around the handle.
    commands
        .spawn((
            Name::new("Pole"),
            Pole,
            SpriteLayer::Pole,
            Transform::from_xyz(0.0, -583.0, 1.0),
            Visibility::Inherited,
        ))
        .with_child((
            Name::new("PoleMesh"),
            Mesh2d(mesh),
            MeshMaterial2d(material),
            // Offset the centered mesh up by half its height so its bottom aligns with the root.
            Transform::from_xyz(0.0, 483.0, 0.0),
        ))
        .with_child((
            Name::new("PoleTip"),
            PoleTip,
            // Pole mesh is 966 tall; tip is at half-height (483) above its center (483) = 966.
            Transform::from_xyz(0.0, 966.0, 0.0),
        ));
}
#[cfg(feature = "dev")]
fn draw_pole_tip_gizmo(tip_top_query: Query<&GlobalTransform, With<PoleTip>>, mut gizmos: Gizmos) {
    for global_transform in tip_top_query.iter() {
        let pos = global_transform.translation().truncate();
        gizmos.circle_2d(pos, 10.0, bevy::color::Color::srgb(1.0, 0.0, 1.0));
        gizmos.cross_2d(pos, 12.0, bevy::color::Color::srgb(1.0, 1.0, 0.0));
    }
}
