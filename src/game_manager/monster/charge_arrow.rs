use bevy::{
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

use crate::prelude::*;

// ── Constants ──────────────────────────────────────────────────────────────

/// Side length of the quad mesh in world pixels.
const CHARGE_ARROW_SIZE: f32 = 280.0;

/// How long it takes to fully charge, in seconds.
const CHARGE_DURATION: f32 = 2.0;

// ── Material ───────────────────────────────────────────────────────────────

/// Custom 2D material driving `shaders/charge_arrow.wgsl`.
///
/// Binding layout (group 2):
///   0 — params     : vec4  (x = progress [0..1], y = flip)
///   1 — color_fill : vec4  (filled arrow color — ColorPalette::blood_bright, linear)
///   2 — color_bg   : vec4  (unfilled arrow color — ColorPalette::abyss_red, linear)
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ChargeArrowMaterial {
    #[uniform(0)]
    pub params: Vec4,
    #[uniform(1)]
    pub color_fill: Vec4,
    #[uniform(2)]
    pub color_bg: Vec4,
}

impl Material2d for ChargeArrowMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/charge_arrow.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

// ── Marker ─────────────────────────────────────────────────────────────────

/// Marks the charge-arrow mesh entity.
#[derive(Component)]
struct ChargeArrow;

/// Which corner the charge arrow is currently in (`true` = right, `false` = left).
#[derive(Resource)]
pub(super) struct ChargeArrowSide(pub bool);

/// Remembers the last two side picks to prevent 3 consecutive same-side picks.
#[derive(Resource, Default)]
pub(super) struct ArrowSideHistory([Option<bool>; 2]);

impl ArrowSideHistory {
    /// Returns true if both recorded picks are `side`, forcing the opposite.
    fn is_forced_opposite(&self, side: bool) -> bool {
        self.0[0] == Some(side) && self.0[1] == Some(side)
    }

    fn record(&mut self, side: bool) {
        self.0[0] = self.0[1];
        self.0[1] = Some(side);
    }
}

/// Tracks accumulated charge while `MonsterState::Roaming` is active.
#[derive(Resource, Default)]
pub(super) struct ChargeProgress {
    /// Current fill level in [0.0, 1.0].
    pub value: f32,
    /// Whether the charge has reached 1.0 this cycle.
    pub triggered: bool,
}

// ── Plugin ─────────────────────────────────────────────────────────────────

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<ChargeArrowMaterial>::default());
    app.init_resource::<ChargeProgress>();
    app.init_resource::<ArrowSideHistory>();
    app.add_systems(OnEnter(MonsterState::PrepareAttack), spawn_charge_arrow);
    app.add_systems(OnExit(MonsterState::PrepareAttack), despawn_charge_arrow);
    app.add_systems(
        Update,
        tick_charge_progress.run_if(in_state(MonsterState::PrepareAttack)),
    );
}

// ── Systems ────────────────────────────────────────────────────────────────

fn color_to_linear_vec4(color: Color) -> Vec4 {
    let c = color.to_linear();
    Vec4::new(c.red, c.green, c.blue, c.alpha)
}

pub(super) fn spawn_charge_arrow(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ChargeArrowMaterial>>,
    mut charge: ResMut<ChargeProgress>,
    mut history: ResMut<ArrowSideHistory>,
    palette: Res<crate::theme::palette::ColorPalette>,
) {
    *charge = ChargeProgress::default();

    let mut use_right_side = getrandom::u32().unwrap_or(0) % 2 == 0;
    // If random pick would make 3 consecutive same-side, flip it.
    if history.is_forced_opposite(use_right_side) {
        use_right_side = !use_right_side;
    }
    history.record(use_right_side);
    commands.insert_resource(ChargeArrowSide(use_right_side));
    let flip = if use_right_side { 1.0_f32 } else { 0.0_f32 };

    let mat = materials.add(ChargeArrowMaterial {
        params: Vec4::new(0.0, flip, 0.0, 0.0),
        color_fill: color_to_linear_vec4(palette.blood_bright),
        color_bg:   color_to_linear_vec4(palette.abyss_red.with_alpha(0.55)),
    });

    let half = CHARGE_ARROW_SIZE / 2.0;
    let half_w = crate::GAME_WIDTH / 2.0;

    // Left arrow: left edge center. Right arrow: right edge center.
    let x = if use_right_side {
        half_w - half //  820
    } else {
        -half_w + half // -820
    };
    let y = 0.0;

    commands.spawn((
        Name::new("ChargeArrow"),
        ChargeArrow,
        SpriteLayer::ChargeArrow,
        Mesh2d(meshes.add(Rectangle::new(CHARGE_ARROW_SIZE, CHARGE_ARROW_SIZE))),
        MeshMaterial2d(mat),
        Transform::from_xyz(x, y, 0.0),
    ));
}

fn despawn_charge_arrow(mut commands: Commands, query: Query<Entity, With<ChargeArrow>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// Advances the charge every frame and syncs progress to the shader.
fn tick_charge_progress(
    time: Res<Time>,
    mut charge: ResMut<ChargeProgress>,
    arrow_query: Query<&MeshMaterial2d<ChargeArrowMaterial>, With<ChargeArrow>>,
    mut materials: ResMut<Assets<ChargeArrowMaterial>>,
) {
    if charge.triggered {
        return;
    }

    charge.value = (charge.value + time.delta_secs() / CHARGE_DURATION).min(1.0);

    if charge.value >= 1.0 {
        charge.triggered = true;
    }

    let Ok(mat_handle) = arrow_query.single() else {
        return;
    };
    let Some(mut mat) = materials.get_mut(&mat_handle.0) else {
        return;
    };
    mat.params.x = charge.value;
}
