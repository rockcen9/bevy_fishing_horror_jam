#import bevy_sprite::mesh2d_vertex_output::VertexOutput

// ── Uniforms ───────────────────────────────────────────────────────────────
// params: x = progress (0..1), y = zone_start (0..1),
//         z = zone_width (0..1),  w = marker_half_width (0..1)
@group(2) @binding(0) var<uniform> params: vec4<f32>;
@group(2) @binding(1) var<uniform> ring_color: vec4<f32>;
@group(2) @binding(2) var<uniform> zone_color: vec4<f32>;
@group(2) @binding(3) var<uniform> marker_color: vec4<f32>;

// ── Constants ──────────────────────────────────────────────────────────────
const PI: f32     = 3.14159265358979323846;
const TWO_PI: f32 = 6.28318530717958647692;

// Ring radii in UV space (quad covers [0,1]×[0,1], center at (0.5, 0.5)).
// outer=0.45 fills nearly the full quad; inner=0.35 gives a 0.10-wide band.
const OUTER_RADIUS: f32 = 0.45;
const INNER_RADIUS: f32 = 0.35;

// ── Helpers ────────────────────────────────────────────────────────────────

// Returns true if `angle` (in [0,1)) lies within the arc [arc_start, arc_end).
// Handles wrap-around (e.g. arc_start=0.9, arc_end=0.1).
fn arc_hit(angle: f32, arc_start: f32, arc_end: f32) -> bool {
    if arc_start <= arc_end {
        return angle >= arc_start && angle <= arc_end;
    }
    // Wrap-around case: arc straddles the 0/1 boundary.
    return angle >= arc_start || angle <= arc_end;
}

// ── Fragment ───────────────────────────────────────────────────────────────
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {

    // ── Step 1: Convert UV [0,1]→[-0.5,0.5] (centered coordinates) ──────
    // The mesh quad has UV (0,0) at the top-left corner and (1,1) at the
    // bottom-right corner (Bevy 2d convention).  Subtracting 0.5 places the
    // origin at the center of the quad.
    let uv = in.uv - vec2<f32>(0.5, 0.5);

    // ── Step 2: Radial distance & ring mask ──────────────────────────────
    // Euclidean distance from center — the "radius" in polar coordinates.
    let dist = length(uv);

    // smoothstep gives AA at both radial edges:
    //   outer edge: 1 → 0 as dist goes from OUTER_RADIUS-aa to OUTER_RADIUS+aa
    //   inner edge: 0 → 1 as dist goes from INNER_RADIUS-aa to INNER_RADIUS+aa
    let aa = 0.005; // anti-alias half-width in UV units
    let ring_mask =
        smoothstep(OUTER_RADIUS + aa, OUTER_RADIUS - aa, dist)
      * smoothstep(INNER_RADIUS - aa, INNER_RADIUS + aa, dist);

    // Discard pixels fully outside the ring band.
    if ring_mask < 0.001 {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }

    // ── Step 3: Angular position (polar θ) ───────────────────────────────
    // We want θ = 0 at 12 o'clock (top of the ring) and increasing clockwise.
    //
    // Screen / UV space: y increases *downward*, so the top of the quad has
    // uv.y = 0, meaning centered_uv.y = -0.5.
    //
    // Using atan2(x, -y) achieves the desired mapping:
    //   • Top    (0, -0.5) → atan2( 0,  0.5) = 0          → angle = 0.00
    //   • Right  (0.5,  0) → atan2(0.5, 0  ) = π/2        → angle = 0.25
    //   • Bottom (0,  0.5) → atan2( 0, -0.5) = π          → angle = 0.50
    //   • Left  (-0.5,  0) → atan2(-0.5, 0 ) = -π/2       → fract(-0.25) = 0.75
    //
    // fract() normalises the full circle to [0, 1) without any branch.
    let raw_angle = atan2(uv.x, -uv.y);
    let angle = fract(raw_angle / TWO_PI);

    // ── Step 4: Unpack uniform params ───────────────────────────────────
    let progress         = params.x; // marker centre position [0, 1)
    let zone_start       = params.y; // success zone start     [0, 1)
    let zone_width       = params.z; // success zone width     (0, 1)
    let marker_half      = params.w; // half-width of marker   (0, 1)

    let zone_end     = fract(zone_start + zone_width);
    let marker_start = fract(progress - marker_half + 1.0); // +1 avoids fract(-tiny)
    let marker_end   = fract(progress + marker_half);

    // ── Step 5: Layer colours ────────────────────────────────────────────
    // Layer order (painter's algorithm, back to front):
    //   background ring → success zone → moving marker
    var color = ring_color;

    if arc_hit(angle, zone_start, zone_end) {
        color = zone_color;
    }

    // Marker always draws on top of the zone so it is visible inside it.
    if arc_hit(angle, marker_start, marker_end) {
        color = marker_color;
    }

    // Blend ring mask into alpha for smooth AA at the radial edges.
    return vec4<f32>(color.rgb, color.a * ring_mask);
}
