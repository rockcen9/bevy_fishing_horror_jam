#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_render::globals::Globals

@group(0) @binding(1) var<uniform> globals: Globals;

@group(2) @binding(0) var bg_texture: texture_2d<f32>;
@group(2) @binding(1) var bg_sampler: sampler;

struct LakeParams {
    start_y: f32,
};
@group(2) @binding(2) var<uniform> lake_params: LakeParams;

// Bubble break effect parameters.
// Each active source emits bubbles that rise to the surface and pop.
// Set count = 0 to disable the effect entirely.
// sources[i].x  = UV-x position of the bubble column (0.0 – 1.0)
// sources[i].yzw = unused padding
struct BubbleParams {
    sources: array<vec4<f32>, 8>,
    count:   u32,
};
@group(2) @binding(3) var<uniform> bubble_params: BubbleParams;

// ── Constants ─────────────────────────────────────────────────────────────────

const MAX_SOURCES:      u32 = 8u;
const SLOTS_PER_SOURCE: u32 = 3u;   // simultaneous bubbles per column
const BUBBLE_PERIOD:    f32 = 2.8;  // seconds per slot cycle
const BUBBLE_RISE_UV:   f32 = 0.065;// depth below surface where bubbles spawn (UV)
const BUBBLE_RADIUS:    f32 = 0.003; // rising dot radius (UV)
const ASPECT:           f32 = 1920.0 / 1080.0;

// ── Helpers ───────────────────────────────────────────────────────────────────

// Returns a pseudo-random float in [0, 1) for a given 2D float coordinate.
fn hash21(p: vec2<f32>) -> f32 {
    var q = fract(p * vec2<f32>(0.1031, 0.1030));
    let d = dot(q, q.yx + 33.33);
    q += vec2<f32>(d, d);
    return fract((q.x + q.y) * q.x);
}

// Additive glow [0, 1] from one bubble slot.
// `seed`    – unique float per (source, slot) pair for independent timing.
// `src_x`   – UV-x of the bubble column.
// `lake_y`  – UV-y of the water surface.
fn bubble_glow(uv: vec2<f32>, t: f32, src_x: f32, lake_y: f32, seed: f32) -> f32 {
    // Each slot gets its own phase offset so bubbles in the same column
    // are staggered and don't all pop at the same time.
    let phase   = hash21(vec2<f32>(src_x * 17.3, seed * 31.7)) * BUBBLE_PERIOD;
    let local_t = fract((t + phase) / BUBBLE_PERIOD);  // [0, 1) within one cycle

    // Slight lateral wobble as the bubble rises.
    let wobble = sin(t * 2.3 + seed * 5.1) * 0.004;
    let bx = src_x + wobble;

    // Bubble travels upward from BUBBLE_RISE_UV below the surface and fades out at the top.
    let by = lake_y + BUBBLE_RISE_UV * (1.0 - local_t);

    let dx = (uv.x - bx) * ASPECT;
    let dy = uv.y - by;
    let d  = sqrt(dx * dx + dy * dy);

    let circle = smoothstep(BUBBLE_RADIUS, BUBBLE_RADIUS * 0.3, d);
    // Fade in quickly, fade out as it nears the surface.
    let fade = sin(local_t * 3.14159);
    return circle * fade;
}

// ── Fragment ──────────────────────────────────────────────────────────────────

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv           = mesh.uv;
    let lake_start_y = lake_params.start_y;

    // Above the lake — draw sky with twinkling stars.
    if uv.y < lake_start_y {
        let color = textureSampleLevel(bg_texture, bg_sampler, uv, 0.0);
        let luma  = dot(color.rgb, vec3<f32>(0.299, 0.587, 0.114));

        if luma > 0.28 {
            let pixel   = floor(uv * vec2<f32>(1920.0, 1080.0));
            let phase   = hash21(pixel) * 6.28318;
            let freq    = 1.5 + hash21(pixel + vec2<f32>(17.0, 31.0)) * 3.0;
            let twinkle = 0.1 + 1.2 * abs(sin(globals.time * freq + phase));
            let bloom   = max(0.0, twinkle - 1.0) * 0.4;
            return vec4<f32>(color.rgb * twinkle + vec3<f32>(bloom), color.a);
        }

        return color;
    }

    let t     = globals.time;
    let depth = (uv.y - lake_start_y) / (1.0 - lake_start_y);
    let amp   = 0.0012 + depth * 0.0008;

    let w1 = sin(uv.x * 55.0 + t * 1.3) * amp;
    let w2 = sin(uv.x * 28.0 - t * 0.85 + uv.y * 10.0) * amp * 0.55;
    let w3 = sin(uv.x * 90.0 + t * 2.2  + uv.y * 6.0)  * amp * 0.28;
    let dx = sin(uv.y * 18.0 + t * 0.65) * amp * 0.35;

    let distorted = vec2<f32>(
        clamp(uv.x + dx,           0.0, 1.0),
        clamp(uv.y + w1 + w2 + w3, lake_start_y, 1.0),
    );

    var color = textureSampleLevel(bg_texture, bg_sampler, distorted, 0.0);

    // Bubble break effect — skipped entirely when count == 0.
    if bubble_params.count > 0u {
        var glow   = 0.0;
        let num_sources = min(bubble_params.count, MAX_SOURCES);
        for (var s = 0u; s < num_sources; s++) {
            let src_x = bubble_params.sources[s].x;
            for (var slot = 0u; slot < SLOTS_PER_SOURCE; slot++) {
                let seed = f32(s * SLOTS_PER_SOURCE + slot);
                glow += bubble_glow(uv, t, src_x, lake_start_y, seed);
            }
        }
        glow = clamp(glow, 0.0, 1.0);
        // Eerie blue-white shimmer suits the horror atmosphere.
        let bubble_tint = vec3<f32>(0.55, 0.88, 1.0);
        color = vec4<f32>(color.rgb + bubble_tint * glow * 0.85, color.a);
    }

    return color;
}
