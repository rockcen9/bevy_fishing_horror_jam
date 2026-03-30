#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::globals::Globals

@group(0) @binding(1) var<uniform> globals: Globals;

@group(2) @binding(0) var bg_texture: texture_2d<f32>;
@group(2) @binding(1) var bg_sampler: sampler;

// FIX 1: Wrap the scalar in a struct
struct LakeParams {
    start_y: f32,
};
@group(2) @binding(2) var<uniform> lake_params: LakeParams;

// Returns a pseudo-random float in [0, 1) for a given 2D integer coordinate.
fn hash21(p: vec2<f32>) -> f32 {
    var q = fract(p * vec2<f32>(0.1031, 0.1030));
    // FIX 3: Explicitly cast scalar to vec2 for addition
    let d = dot(q, q.yx + 33.33);
    q += vec2<f32>(d, d);
    return fract((q.x + q.y) * q.x);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;
    let lake_start_y = lake_params.start_y;

    // Above the lake — draw sky with twinkling stars.
    if uv.y < lake_start_y {
        // FIX 2: Use textureSampleLevel inside non-uniform control flow
        let color = textureSampleLevel(bg_texture, bg_sampler, uv, 0.0);
        let luma = dot(color.rgb, vec3<f32>(0.299, 0.587, 0.114));

        // Stars are bright pixels that stand out from the dark sky.
        if luma > 0.28 {
            let pixel = floor(uv * vec2<f32>(1920.0, 1080.0));
            let phase = hash21(pixel) * 6.28318;
            let freq  = 1.5 + hash21(pixel + vec2<f32>(17.0, 31.0)) * 3.0;
            // Dramatic twinkle: oscillates between 10 % and 130 % brightness.
            let twinkle = 0.1 + 1.2 * abs(sin(globals.time * freq + phase));
            // Also add a slight white bloom at peak brightness.
            let bloom = max(0.0, twinkle - 1.0) * 0.4;
            return vec4<f32>(color.rgb * twinkle + vec3<f32>(bloom), color.a);
        }

        return color;
    }

    let t = globals.time;

    // 0 = shoreline, 1 = bottom of image.
    let depth = (uv.y - lake_start_y) / (1.0 - lake_start_y);

    // Wave amplitude grows slightly toward the bottom (perspective).
    let amp = 0.0012 + depth * 0.0008;

    // Three overlapping sine waves scrolling at different speeds/frequencies.
    let w1 = sin(uv.x * 55.0 + t * 1.3) * amp;
    let w2 = sin(uv.x * 28.0 - t * 0.85 + uv.y * 10.0) * amp * 0.55;
    let w3 = sin(uv.x * 90.0 + t * 2.2 + uv.y * 6.0) * amp * 0.28;

    // Small lateral drift adds a bit of choppiness.
    let dx = sin(uv.y * 18.0 + t * 0.65) * amp * 0.35;

    let distorted = vec2<f32>(
        clamp(uv.x + dx, 0.0, 1.0),
        clamp(uv.y + w1 + w2 + w3, lake_start_y, 1.0),
    );

    // Using textureSampleLevel here as well is generally safer and slightly faster for 2D meshes.
    return textureSampleLevel(bg_texture, bg_sampler, distorted, 0.0);
}