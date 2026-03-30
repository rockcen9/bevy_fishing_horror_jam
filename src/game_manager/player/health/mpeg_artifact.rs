use crate::prelude::*;
use bevy::{
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin, MeshMaterial2d},
};
use camera_effects::CameraShakeEvent;
use kira_ext::SFXEvent;

use super::PlayerHealth;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<MpegArtifactMaterial>::default());
    app.add_systems(
        Update,
        (spawn_mpeg_artifact_on_damage, tick_mpeg_artifact_fade),
    );
}

// ── Material ─────────────────────────────────────────────────────────────────

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct MpegArtifactMaterial {
    #[uniform(0)]
    pub intensity: f32,
    #[uniform(0)]
    pub alpha: f32,
}

impl Material2d for MpegArtifactMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/mpeg_artifact.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

// ── Overlay entity ────────────────────────────────────────────────────────────

const MPEG_ARTIFACT_FADE_DURATION_SECS: f32 = 0.2;

#[derive(Component)]
struct MpegArtifactOverlay {
    elapsed: f32,
}

fn spawn_mpeg_artifact_on_damage(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MpegArtifactMaterial>>,
    health: Res<PlayerHealth>,
    mut prev_health: Local<f32>,
) {
    let current = health.value;
    let previous = *prev_health;
    *prev_health = current;

    if current >= previous {
        return;
    }

    commands.trigger(SFXEvent::sfx("glitch"));

    let mesh = meshes.add(Rectangle::new(crate::GAME_WIDTH, crate::GAME_HEIGHT));
    let material = materials.add(MpegArtifactMaterial {
        intensity: 1.0,
        alpha: 1.0,
    });
    commands.spawn((
        Name::new("MpegArtifactOverlay"),
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, 0.0, 999.0),
        MpegArtifactOverlay { elapsed: 0.0 },
    ));
    commands.trigger(CameraShakeEvent);
}

fn tick_mpeg_artifact_fade(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut MpegArtifactOverlay,
        &MeshMaterial2d<MpegArtifactMaterial>,
    )>,
    mut materials: ResMut<Assets<MpegArtifactMaterial>>,
    time: Res<Time>,
) {
    for (entity, mut overlay, mat_handle) in &mut query {
        overlay.elapsed += time.delta_secs();

        if let Some(mut mat) = materials.get_mut(&mat_handle.0) {
            mat.alpha = (1.0 - overlay.elapsed / MPEG_ARTIFACT_FADE_DURATION_SECS).max(0.0);
        }

        if overlay.elapsed >= MPEG_ARTIFACT_FADE_DURATION_SECS {
            commands.entity(entity).despawn();
        }
    }
}
