use crate::game_manager::background::{StartBubbleEvent, StopBubbleEvent};
use crate::prelude::*;

use super::{charge_arrow::ChargeArrowSide, zone::MonsterZone};

const SPAWN_X_MARGIN: f32 = 400.0;

// ── Resources ─────────────────────────────────────────────────────────────────

/// Pre-decided world-space spawn point for the monster shadow.
/// Chosen on `OnEnter(MonsterState::Bubble)` so the position is fixed
/// before the shadow state begins.
#[derive(Resource)]
pub(super) struct MonsterShadowSpawnPoint {
    pub x: f32,
    pub y: f32,
}

/// Tracks the last two `ChargeArrowSide` picks to prevent three consecutive
/// same-side picks.
#[derive(Resource, Default)]
pub(super) struct ArrowSideHistory([Option<bool>; 2]);

impl ArrowSideHistory {
    fn is_forced_opposite(&self, side: bool) -> bool {
        self.0[0] == Some(side) && self.0[1] == Some(side)
    }

    fn record(&mut self, side: bool) {
        self.0[0] = self.0[1];
        self.0[1] = Some(side);
    }
}

// ── Plugin ────────────────────────────────────────────────────────────────────

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ArrowSideHistory>();
    app.add_systems(OnEnter(MonsterState::Bubble), decide_spawn_point);
    app.add_systems(OnExit(MonsterState::Bubble), stop_bubble);
}

// ── Systems ───────────────────────────────────────────────────────────────────

fn decide_spawn_point(
    mut commands: Commands,
    mut history: ResMut<ArrowSideHistory>,
    zone: Res<MonsterZone>,
) {
    let mut use_right_side = getrandom::u32().unwrap_or(0) % 2 == 0;
    if history.is_forced_opposite(use_right_side) {
        use_right_side = !use_right_side;
    }
    history.record(use_right_side);

    // Arrow on right → shadow on left half, arrow on left → shadow on right half.
    let x = if use_right_side {
        f32_random_range(-crate::GAME_WIDTH / 2.0 + SPAWN_X_MARGIN, 0.0)
    } else {
        f32_random_range(0.0, crate::GAME_WIDTH / 2.0 - SPAWN_X_MARGIN)
    };
    let y = f32_random_range(zone.min_y, zone.max_y);

    commands.insert_resource(ChargeArrowSide(use_right_side));
    commands.insert_resource(MonsterShadowSpawnPoint { x, y });

    let uv_x = (x + crate::GAME_WIDTH / 2.0) / crate::GAME_WIDTH;
    commands.trigger(StartBubbleEvent { positions: vec![uv_x] });
}

fn stop_bubble(mut commands: Commands) {
    commands.trigger(StopBubbleEvent);
}

fn f32_random_range(min: f32, max: f32) -> f32 {
    let raw = getrandom::u32().unwrap_or(0);
    let t = (raw as f32) / (u32::MAX as f32);
    min + t * (max - min)
}
