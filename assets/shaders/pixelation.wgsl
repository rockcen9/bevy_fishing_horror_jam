#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var base_color_texture: texture_2d<f32>;
@group(2) @binding(1) var base_color_sampler: sampler;
@group(2) @binding(2) var<uniform> pixels_per_block: f32;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let texture_size = vec2<f32>(textureDimensions(base_color_texture));
    let pixel_coords = mesh.uv * texture_size;
    let quantized_coords = floor(pixel_coords / pixels_per_block) * pixels_per_block + (pixels_per_block / 2.0);
    let quantized_uv = quantized_coords / texture_size;
    return textureSample(base_color_texture, base_color_sampler, quantized_uv);
}
