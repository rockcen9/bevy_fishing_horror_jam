use bevy::{
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

use crate::prelude::*;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct HueShiftFishMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
    #[uniform(2)]
    pub hue_shift: f32,
}

impl Material2d for HueShiftFishMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/random_color_fish.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<HueShiftFishMaterial>::default());
}
