#import bevy_sprite::mesh2d_vertex_output::VertexOutput

// ── Uniforms ───────────────────────────────────────────────────────────────
// params.x = progress [0.0, 1.0] — filled clockwise from 12 o'clock
@group(2) @binding(0) var<uniform> params: vec4<f32>;
// color_fill / color_bg driven by ColorPalette (ivory / grime) from Rust
@group(2) @binding(1) var<uniform> color_fill: vec4<f32>;
@group(2) @binding(2) var<uniform> color_bg: vec4<f32>;

// ── Constants ──────────────────────────────────────────────────────────────
const TWO_PI: f32 = 6.28318530717958647692;

// Ring radii in UV space (quad center at (0.5, 0.5), half-extent = 0.5).
const OUTER_RADIUS: f32 = 0.45;
const INNER_RADIUS: f32 = 0.35;

// Anti-alias half-width in UV units.
const AA: f32 = 0.008;

// ── Fragment ───────────────────────────────────────────────────────────────
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {

    // ── Step 1: Center UV at (0, 0) ────────────────────────────────────────
    // Bevy rectangle UV: (0,0) top-left, (1,1) bottom-right.
    let uv = in.uv - vec2<f32>(0.5, 0.5);

    // ── Step 2: Radial ring mask with smooth AA ─────────────────────────────
    let dist = length(uv);
    let ring_mask =
        smoothstep(OUTER_RADIUS + AA, OUTER_RADIUS - AA, dist)
      * smoothstep(INNER_RADIUS - AA, INNER_RADIUS + AA, dist);

    if ring_mask < 0.001 {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }

    // ── Step 3: Clockwise angle from 12 o'clock ─────────────────────────────
    // atan2(x, -y): 0 at top, 0.25 at right, 0.5 at bottom, 0.75 at left.
    //   Top    (0, -0.5) → atan2( 0,  0.5) =  0       → 0.00
    //   Right  (0.5,  0) → atan2(0.5, 0  ) =  π/2     → 0.25
    //   Bottom (0,  0.5) → atan2( 0, -0.5) =  π       → 0.50
    //   Left  (-0.5,  0) → atan2(-0.5, 0 ) = -π/2     → fract = 0.75
    let raw_angle = atan2(uv.x, -uv.y);
    let angle = fract(raw_angle / TWO_PI); // normalize to [0, 1)

    // ── Step 4: Color based on progress ────────────────────────────────────
    let progress = params.x;

    var color: vec4<f32>;
    if angle < progress {
        color = color_fill;
    } else {
        color = color_bg;
    }

    return vec4<f32>(color.rgb, color.a * ring_mask);
}
