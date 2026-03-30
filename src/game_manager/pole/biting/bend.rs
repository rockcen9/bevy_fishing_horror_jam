use crate::prelude::*;
use super::pull_up::PullBendBoost;
use super::move_bobber::{BobberSpringMotion, BobberPhase};

/// Max tip displacement in local-space pixels when a fish is biting.
/// The pole mesh is 966px tall; 200px gives ~20% arc at the tip.
const POLE_MAX_BEND_PX: f32 = 200.0;

/// Default TipTop position in Pole root local space (un-bent).
const TIPTOP_DEFAULT_Y: f32 = 966.0;

/// Tremble params per bobber phase: (primary_freq, amp).
const TREMBLE_RESTING:    (f32, f32) = (5.0,  2.0);
const TREMBLE_NIBBLING:   (f32, f32) = (12.0, 6.0);
const TREMBLE_STRUGGLING: (f32, f32) = (22.0, 15.0);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (sync_pole_bend_to_bobber, sync_tiptop_position_from_bend)
            .chain()
            .run_if(in_state(GameState::Biting))
            .run_if(in_state(Pause(false))),
    );
    app.add_systems(OnExit(GameState::Biting), (reset_pole_bend, reset_tiptop_position));
}

fn sync_pole_bend_to_bobber(
    tiptop_query: Query<&GlobalTransform, With<PoleTip>>,
    bobber_query: Query<(&GlobalTransform, Option<&BobberSpringMotion>), With<Bobber>>,
    pole_mesh_query: Query<(&MeshMaterial2d<PoleMaterial>, &GlobalTransform)>,
    mut materials: ResMut<Assets<PoleMaterial>>,
    pull_boost: Option<Res<PullBendBoost>>,
    time: Res<Time>,
) {
    let Some(tip) = tiptop_query.iter().next() else { return };
    let Some((bobber_transform, bobber_motion)) = bobber_query.iter().next() else { return };
    let Some((mat_handle, pole_mesh_transform)) = pole_mesh_query.iter().next() else { return };
    let Some(mut material) = materials.get_mut(&mat_handle.0) else { return };

    let tip_pos = tip.translation().truncate();
    let bobber_pos = bobber_transform.translation().truncate();
    let world_dir = (bobber_pos - tip_pos).normalize_or_zero();

    let (_, rotation, _) = pole_mesh_transform.to_scale_rotation_translation();
    let local_dir = rotation.inverse().mul_vec3(world_dir.extend(0.0));

    let phase = bobber_motion.map(|m| m.phase).unwrap_or(BobberPhase::Resting);
    let (freq, amp) = match phase {
        BobberPhase::Resting    => TREMBLE_RESTING,
        BobberPhase::Nibbling   => TREMBLE_NIBBLING,
        BobberPhase::Struggling => TREMBLE_STRUGGLING,
    };

    let t = time.elapsed_secs();
    let tremble = f32::sin(t * freq) * amp
        + f32::sin(t * (freq * 1.44)) * (amp * 0.5)
        + f32::sin(t * (freq * 0.64)) * (amp * 0.3);

    let boost = pull_boost.map(|b| b.0).unwrap_or(0.0);
    let bend_x = local_dir.x * POLE_MAX_BEND_PX + local_dir.x.signum() * boost + tremble;

    let bend = Vec4::new(bend_x, 0.0, 0.0, 0.0);
    if material.bend_params != bend {
        material.bend_params = bend;
    }
}

fn sync_tiptop_position_from_bend(
    pole_mesh_query: Query<&MeshMaterial2d<PoleMaterial>>,
    materials: Res<Assets<PoleMaterial>>,
    mut tiptop_query: Query<&mut Transform, With<PoleTip>>,
) {
    let Some(mat_handle) = pole_mesh_query.iter().next() else { return };
    let Some(material) = materials.get(&mat_handle.0) else { return };
    let Some(mut tiptop) = tiptop_query.iter_mut().next() else { return };

    let bend_x = material.bend_params.x;
    let new_translation = Vec3::new(bend_x, TIPTOP_DEFAULT_Y - bend_x.abs() * 0.25, 0.0);
    if tiptop.translation != new_translation {
        tiptop.translation = new_translation;
    }
}

fn reset_pole_bend(
    pole_mesh_query: Query<&MeshMaterial2d<PoleMaterial>>,
    mut materials: ResMut<Assets<PoleMaterial>>,
) {
    let Some(mat_handle) = pole_mesh_query.iter().next() else { return };
    let Some(mut material) = materials.get_mut(&mat_handle.0) else { return };
    if material.bend_params != Vec4::ZERO {
        material.bend_params = Vec4::ZERO;
    }
}

fn reset_tiptop_position(mut tiptop_query: Query<&mut Transform, With<PoleTip>>) {
    let original = Vec3::new(0.0, TIPTOP_DEFAULT_Y, 0.0);
    for mut t in tiptop_query.iter_mut() {
        if t.translation != original {
            t.translation = original;
        }
    }
}
