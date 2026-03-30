#import bevy_sprite::{
    mesh2d_functions as mesh_functions,
    mesh2d_vertex_output::VertexOutput,
}

@group(2) @binding(0) var base_color_texture: texture_2d<f32>;
@group(2) @binding(1) var base_color_sampler: sampler;
@group(2) @binding(2) var<uniform> bend_params: vec4<f32>;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@vertex
fn vertex(v: Vertex) -> VertexOutput {
    var out: VertexOutput;

    // t = 0 at handle (bottom, uv.y = 1), t = 1 at tip (top, uv.y = 0).
    // t^4 keeps the lower 2/3 of the rod nearly straight and concentrates the
    // bend at the tip — matching a tapered Fast-Action rod under load.
    let t = 1.0 - v.uv.y;
    let t2 = t * t;
    let deflection = t2 * t2;

    var local_pos = v.position;
    local_pos.x += bend_params.x * deflection;
    // Length compensation tuned for the steeper t^4 curve.
    local_pos.y -= abs(bend_params.x) * deflection * 0.25;

    let world_from_local = mesh_functions::get_world_from_local(v.instance_index);
    let world_pos = mesh_functions::mesh2d_position_local_to_world(
        world_from_local,
        vec4<f32>(local_pos, 1.0),
    );

    out.world_position = world_pos;
    out.position = mesh_functions::mesh2d_position_world_to_clip(world_pos);
    out.world_normal = mesh_functions::mesh2d_normal_local_to_world(v.normal, v.instance_index);
    out.uv = v.uv;
    return out;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(base_color_texture, base_color_sampler, mesh.uv);
}
