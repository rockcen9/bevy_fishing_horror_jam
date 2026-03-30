use bevy::{
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};
use bevy_tweening::{AnimTarget, Delay, Lens, Tween, TweenAnim};
use std::time::Duration;

use super::{BitingSet, detect::PullUpInput, monster_threat::MonsterThreat};
use crate::{game_manager::player::PlayerHealth, prelude::*};
use kira_ext::SFXEvent;

// ── Constants ──────────────────────────────────────────────────────────────

/// Diameter of the dial quad in world pixels.
const QTE_DIAL_SIZE_PX: f32 = 400.0;

/// How fast the marker travels around the ring (full rotations per second).
const QTE_MARKER_SPEED_RPS: f32 = 0.35;

/// Half-angular width of the moving marker needle [0, 1] normalized.
const QTE_MARKER_HALF_WIDTH_NORMALIZED: f32 = 0.0125;

/// Angular width of the success zone [0, 1] normalized.
const QTE_ZONE_WIDTH_NORMALIZED: f32 = 0.12;

// ── Material ───────────────────────────────────────────────────────────────

/// Custom 2D material that drives the QTE dial shader.
///
/// Binding layout (group 2):
///   0 — params      : vec4  (progress, zone_start, zone_width, marker_half_width)
///   1 — ring_color  : vec4  (background ring RGBA)
///   2 — zone_color  : vec4  (success arc RGBA)
///   3 — marker_color: vec4  (needle RGBA, changed to green/red on hit/miss)
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub(super) struct QteDialMaterial {
    /// x = progress (0..1 clockwise from 12 o'clock)
    /// y = zone_start (0..1)
    /// z = zone_width (0..1)
    /// w = marker half-width (0..1)
    #[uniform(0)]
    pub params: Vec4,

    #[uniform(1)]
    pub ring_color: Vec4,

    #[uniform(2)]
    pub zone_color: Vec4,

    #[uniform(3)]
    pub marker_color: Vec4,
}

impl Material2d for QteDialMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/qte_dial.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

// ── Resource ───────────────────────────────────────────────────────────────

/// Runtime state for the active QTE.
#[derive(Resource, Debug)]
pub(super) struct QteState {
    /// Marker position [0, 1) around the ring.
    pub progress: f32,
    /// Start of the success arc [0, 1).
    pub zone_start: f32,
    /// Set when the player commits (Space press): true = in zone, false = miss.
    /// Pull-up reads this to decide the outcome after the animation finishes.
    pub result: Option<bool>,
}

impl Default for QteState {
    fn default() -> Self {
        // Zone starts at ~10 o'clock position so it is not trivially visible.
        Self {
            progress: 0.0,
            zone_start: 0.18,
            result: None,
        }
    }
}

// ── Alpha lens ─────────────────────────────────────────────────────────────

/// Fades all color uniform alphas on the QTE dial material.
struct QteDialAlphaLens {
    start: f32,
    end: f32,
}

impl Lens<QteDialMaterial> for QteDialAlphaLens {
    fn lerp(&mut self, mut target: Mut<'_, QteDialMaterial>, ratio: f32) {
        let alpha = self.start + (self.end - self.start) * ratio;
        target.ring_color.w = alpha;
        target.zone_color.w = alpha;
        target.marker_color.w = alpha;
    }
}

// ── Marker components ──────────────────────────────────────────────────────

/// Marks the QTE dial mesh entity for easy query / cleanup.
#[derive(Component)]
struct QteDial;

/// Marks the entity holding the dial fade-out tween for cleanup.
#[derive(Component)]
struct QteDialFadeTween;

// ── Plugin ─────────────────────────────────────────────────────────────────

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<QteDialMaterial>::default());
    app.init_resource::<QteState>();
    app.add_systems(OnEnter(GameState::Biting), spawn_qte_dial);
    app.add_systems(OnExit(GameState::Biting), despawn_qte_dial);
    app.add_systems(
        Update,
        tick_qte_marker_progress
            .run_if(in_state(GameState::Biting))
            .run_if(in_state(Pause(false))),
    );
    app.add_systems(
        Update,
        on_pull_gesture_commit_qte
            .in_set(BitingSet::QteInput)
            .run_if(in_state(GameState::Biting))
            .run_if(in_state(Pause(false))),
    );
}

// ── Systems ────────────────────────────────────────────────────────────────

fn color_to_linear_vec4(color: Color) -> Vec4 {
    let c = color.to_linear();
    Vec4::new(c.red, c.green, c.blue, c.alpha)
}

fn spawn_qte_dial(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<QteDialMaterial>>,
    mut qte: ResMut<QteState>,
    palette: Res<crate::theme::palette::ColorPalette>,
) {
    // Use sub-second elapsed time as a cheap varied zone position.
    let zone_start = time.elapsed_secs().fract();
    *qte = QteState {
        zone_start,
        ..QteState::default()
    };

    let material = materials.add(QteDialMaterial {
        params: Vec4::new(
            qte.progress,
            qte.zone_start,
            QTE_ZONE_WIDTH_NORMALIZED,
            QTE_MARKER_HALF_WIDTH_NORMALIZED,
        ),
        // grime — semi-transparent background ring
        ring_color: color_to_linear_vec4(palette.grime.with_alpha(0.75)),
        // amber — success zone arc
        zone_color: color_to_linear_vec4(palette.amber),
        // ivory — needle
        marker_color: color_to_linear_vec4(palette.ivory),
    });

    commands.spawn((
        Name::new("QteDial"),
        QteDial,
        SpriteLayer::QteDial,
        Mesh2d(meshes.add(Rectangle::new(QTE_DIAL_SIZE_PX, QTE_DIAL_SIZE_PX))),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, 0.0, 50.0),
    ));
}

fn despawn_qte_dial(
    mut commands: Commands,
    q_dial: Query<Entity, With<QteDial>>,
    q_fade: Query<Entity, With<QteDialFadeTween>>,
) {
    for entity in &q_dial {
        commands.entity(entity).despawn();
    }
    for entity in &q_fade {
        commands.entity(entity).despawn();
    }
}

/// Advances the marker's angular position every frame and syncs it to the shader.
/// Stops updating once the player has committed (freeze before fade-out).
fn tick_qte_marker_progress(
    time: Res<Time>,
    mut qte: ResMut<QteState>,
    dial_query: Query<&MeshMaterial2d<QteDialMaterial>, With<QteDial>>,
    mut materials: ResMut<Assets<QteDialMaterial>>,
) {
    if qte.result.is_some() {
        return;
    }

    qte.progress = (qte.progress + QTE_MARKER_SPEED_RPS * time.delta_secs()) % 1.0;

    let Ok(mat_handle) = dial_query.single() else {
        return;
    };
    let Some(mut mat) = materials.get_mut(&mat_handle.0) else {
        return;
    };
    mat.params.x = qte.progress;
}

/// Checks for pull-up gesture, stores the hit/miss result, and flashes the needle.
/// State transition happens later, after the pull-up animation finishes.
fn on_pull_gesture_commit_qte(
    pull_input: Res<PullUpInput>,
    mut qte: ResMut<QteState>,
    dial_query: Query<&MeshMaterial2d<QteDialMaterial>, With<QteDial>>,
    mut materials: ResMut<Assets<QteDialMaterial>>,
    mut health: ResMut<PlayerHealth>,
    mut threat: ResMut<MonsterThreat>,
    mut next_monster: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut commands: Commands,
    palette: Res<crate::theme::palette::ColorPalette>,
) {
    // Only evaluate once — ignore subsequent gestures while result is pending.
    if qte.result.is_some() || !pull_input.triggered {
        return;
    }

    let zone_end = (qte.zone_start + QTE_ZONE_WIDTH_NORMALIZED) % 1.0;
    let in_zone = if qte.zone_start <= zone_end {
        qte.progress >= qte.zone_start && qte.progress <= zone_end
    } else {
        // Success zone wraps around the 0/1 boundary.
        qte.progress >= qte.zone_start || qte.progress <= zone_end
    };

    qte.result = Some(in_zone);

    if in_zone {
        commands.trigger(SFXEvent::sfx("pull_up_succeed"));
    } else {
        let new_val = health.value - 10.0;
        health.set(new_val);
    }

    let triggered = threat.record_and_roll(in_zone, time.elapsed_secs());
    if !in_zone && triggered {
        next_monster.set(GameState::Monster);
    }

    // Flash the needle green on success, red on failure.
    // Also spawn a freeze (0.2s) → fade-out (0.2s) tween on the dial material.
    if let Ok(mat_handle) = dial_query.single() {
        if let Some(mut mat) = materials.get_mut(&mat_handle.0) {
            // moss = hit, blood_bright = miss — both from ColorPalette
            mat.marker_color = if in_zone {
                color_to_linear_vec4(palette.moss)
            } else {
                color_to_linear_vec4(palette.blood_bright)
            };
        }

        let delay = Delay::new(Duration::from_millis(500));
        let fade = Tween::new(
            EaseFunction::Linear,
            Duration::from_millis(200),
            QteDialAlphaLens {
                start: 1.0,
                end: 0.0,
            },
        );
        commands.spawn((
            QteDialFadeTween,
            TweenAnim::new(delay.then(fade)),
            AnimTarget::asset(&mat_handle.0),
        ));
    }
}
