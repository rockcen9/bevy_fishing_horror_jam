use bevy::{
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

use crate::prelude::*;

/// Side length of the quad mesh used to render the ring.
/// At OUTER_RADIUS = 0.45 the ring fills 90 % of the quad diameter.
pub const LOADING_BAR_RING_SIZE_PX: f32 = 300.0;

// ── Marker ─────────────────────────────────────────────────────────────────

/// Marker component placed on every circular loading-bar entity.
#[derive(Component)]
pub struct LoadingBar;

// ── Material ───────────────────────────────────────────────────────────────

/// Custom 2D material that drives `shaders/loading_bar.wgsl`.
///
/// Binding layout (group 2):
///   0 — params     : vec4  (x = progress 0..1 clockwise from 12 o'clock, y/z/w unused)
///   1 — color_fill : vec4  (filled arc color — ColorPalette::ivory, linear)
///   2 — color_bg   : vec4  (unfilled ring color — ColorPalette::grime, linear)
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct LoadingBarMaterial {
    /// x = progress [0.0, 1.0] clockwise from 12 o'clock.
    #[uniform(0)]
    pub params: Vec4,
    #[uniform(1)]
    pub color_fill: Vec4,
    #[uniform(2)]
    pub color_bg: Vec4,
}

impl Default for LoadingBarMaterial {
    fn default() -> Self {
        // ivory = #efede9, grime = #5d5656 (both from ColorPalette), converted to linear
        let ivory = Color::srgb_u8(0xef, 0xed, 0xe9).to_linear();
        let grime = Color::srgb_u8(0x5d, 0x56, 0x56).to_linear();
        Self {
            params:     Vec4::ZERO,
            color_fill: Vec4::new(ivory.red, ivory.green, ivory.blue, 1.0),
            color_bg:   Vec4::new(grime.red, grime.green, grime.blue, 0.55),
        }
    }
}

impl Material2d for LoadingBarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/loading_bar.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

// ── Plugin ─────────────────────────────────────────────────────────────────

/// Registers the `LoadingBarMaterial` so it can be used anywhere in the app.
pub fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<LoadingBarMaterial>::default());
}
