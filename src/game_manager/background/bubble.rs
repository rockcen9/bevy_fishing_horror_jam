use crate::prelude::*;
use bevy::ecs::observer::On;
use bevy::math::Vec4;

use super::{BackgroundMaterialHandle, LakeWaterMaterial};

pub(crate) fn plugin(app: &mut App) {
    app.add_observer(handle_start_bubble);
    app.add_observer(handle_stop_bubble);
}

// ── Events ────────────────────────────────────────────────────────────────────

/// Send to enable bubble columns on the lake surface.
/// `positions` holds UV-x coordinates (0.0–1.0), up to 8 entries.
#[derive(Event)]
pub(crate) struct StartBubbleEvent {
    pub positions: Vec<f32>,
}

/// Send to disable the bubble effect entirely.
#[derive(Event)]
pub(crate) struct StopBubbleEvent;

// ── Observers ─────────────────────────────────────────────────────────────────

fn handle_start_bubble(
    trigger: On<StartBubbleEvent>,
    handle: Option<Res<BackgroundMaterialHandle>>,
    mut materials: ResMut<Assets<LakeWaterMaterial>>,
) {
    let Some(handle) = handle else { return };
    let Some(mut mat) = materials.get_mut(&handle.0) else { return };

    let ev = trigger.event();
    let count = ev.positions.len().min(8) as u32;
    for (i, &x) in ev.positions.iter().take(8).enumerate() {
        mat.bubble_params.sources[i] = Vec4::new(x, 0.0, 0.0, 0.0);
    }
    mat.bubble_params.count = count;
}

fn handle_stop_bubble(
    _trigger: On<StopBubbleEvent>,
    handle: Option<Res<BackgroundMaterialHandle>>,
    mut materials: ResMut<Assets<LakeWaterMaterial>>,
) {
    let Some(handle) = handle else { return };
    let Some(mut mat) = materials.get_mut(&handle.0) else { return };
    mat.bubble_params.count = 0;
}
