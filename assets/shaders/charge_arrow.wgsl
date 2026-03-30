#import bevy_sprite::mesh2d_vertex_output::VertexOutput

// ── Uniforms ───────────────────────────────────────────────────────────────
// params.x = progress [0.0, 1.0]  (0 = empty, 1 = full)
@group(2) @binding(0) var<uniform> params: vec4<f32>;
// color_fill / color_bg driven by ColorPalette (blood_bright / abyss_red) from Rust
@group(2) @binding(1) var<uniform> color_fill: vec4<f32>;
@group(2) @binding(2) var<uniform> color_bg: vec4<f32>;

// ── Constants ──────────────────────────────────────────────────────────────

// Arrow axis: tail = right UV(1,0.5), tip = left UV(0,0.5).
// AXIS points left (-x), PERP points up (+y).
const AXIS: vec2<f32> = vec2<f32>(-1.0, 0.0);
const PERP: vec2<f32> = vec2<f32>(0.0, 1.0);

// Arrow geometry (in centered UV units, usable range ≈ [-0.5, 0.5]).
const T_TAIL: f32    = -0.44;   // axis position where the shaft tail starts
const T_TIP: f32     =  0.44;   // axis position at the arrowhead tip
const SHAFT_W: f32   =  0.07;   // shaft half-width
const HEAD_LEN: f32  =  0.22;   // arrowhead length along the axis
const HEAD_W: f32    =  0.20;   // arrowhead half-width at its base

// Anti-alias half-width in UV units.
const AA: f32 = 0.006;


// ── Fragment ───────────────────────────────────────────────────────────────
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {

    // params.y = 1.0 → flip horizontally (right-side arrow pointing right).
    var raw_uv = in.uv;
    if params.y > 0.5 {
        raw_uv.x = 1.0 - raw_uv.x;
    }

    // Center UV at origin.
    let p = raw_uv - vec2<f32>(0.5, 0.5);

    // Decompose into axis (along arrow) and perpendicular components.
    let t = dot(p, AXIS); // -0.5 (tail/right) → +0.5 (tip/left)
    let s = dot(p, PERP);

    // Reject pixels outside the axis span of the arrow.
    if t < T_TAIL || t > T_TIP {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }

    // Compute allowed half-width at this axial position.
    let t_head_base = T_TIP - HEAD_LEN;
    var arrow_hw: f32;
    if t <= t_head_base {
        // Shaft region — constant width.
        arrow_hw = SHAFT_W;
    } else {
        // Arrowhead region — linearly tapers from HEAD_W to 0 at the tip.
        let head_frac = (T_TIP - t) / HEAD_LEN;
        arrow_hw = HEAD_W * head_frac;
    }

    // Soft edge (anti-aliased) using the perpendicular distance.
    let edge_dist = abs(s) - arrow_hw;
    let inside = smoothstep(AA, -AA, edge_dist);

    if inside < 0.001 {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }

    // Fill from tail (t = T_TAIL) toward tip (t = T_TIP) as progress grows.
    let total_len = T_TIP - T_TAIL;
    let t_fill = T_TAIL + params.x * total_len;

    var color: vec4<f32>;
    if t <= t_fill {
        color = color_fill;
    } else {
        color = color_bg;
    }

    return vec4<f32>(color.rgb, color.a * inside);
}
