use kira_ext::SFXEvent;

use super::charge_arrow::{ChargeArrowSide, ChargeProgress};
use crate::prelude::*;

// ── Constants ───────────────────────────────────────────────────────────────

const PEACE_DELAY_SECS: f32 = 1.0;

// ── Resources ───────────────────────────────────────────────────────────────

/// Tracks whether the escape check has already resolved this roaming cycle.
#[derive(Resource, Default)]
pub(super) struct PlayerEscapeResult {
    pub resolved: bool,
}

#[derive(Resource)]
struct AttackToPeaceDelayTimer(Timer);

// ── Plugin ──────────────────────────────────────────────────────────────────

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MonsterState::PrepareAttack), insert_escape_result);
    app.add_systems(OnExit(MonsterState::PrepareAttack), remove_escape_resources);
    app.add_systems(
        Update,
        (evaluate_escape_on_charge_complete, tick_peace_delay)
            .run_if(in_state(MonsterState::PrepareAttack)),
    );
    #[cfg(feature = "dev")]
    app.add_systems(Update, draw_corner_gizmos);
}

// ── Systems ─────────────────────────────────────────────────────────────────

fn insert_escape_result(mut commands: Commands) {
    commands.insert_resource(PlayerEscapeResult::default());
    debug!("roaming started — hide in the corner before the arrow fully charges");
}

fn remove_escape_resources(mut commands: Commands) {
    commands.remove_resource::<PlayerEscapeResult>();
    commands.remove_resource::<AttackToPeaceDelayTimer>();
}

#[cfg(feature = "dev")]
fn draw_corner_gizmos(head: Res<PlayerHeadPosition>, mut gizmos: Gizmos) {
    let half_w = crate::GAME_WIDTH / 2.0;
    let half_h = crate::GAME_HEIGHT / 2.0;
    // Left half and right half safe zones.
    let left_safe = head.position.x <= 0.0;
    let right_safe = head.position.x >= 0.0;
    for (center_x, inside) in [(-half_w / 2.0, left_safe), (half_w / 2.0, right_safe)] {
        let color = if inside {
            Color::srgba(0.0, 1.0, 0.0, 0.3)
        } else {
            Color::srgba(1.0, 1.0, 0.0, 0.15)
        };
        gizmos.rect_2d(
            Vec2::new(center_x, 0.0),
            Vec2::new(half_w, half_h * 2.0),
            color,
        );
    }
}

fn tick_peace_delay(
    time: Res<Time>,
    mut timer: Option<ResMut<AttackToPeaceDelayTimer>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some(ref mut t) = timer else { return };
    if t.0.tick(time.delta()).just_finished() {
        next_state.set(GameState::Idle);
    }
}

fn evaluate_escape_on_charge_complete(
    mut commands: Commands,
    charge: Res<ChargeProgress>,
    mut result: ResMut<PlayerEscapeResult>,
    head: Res<PlayerHeadPosition>,
    arrow_side: Option<Res<ChargeArrowSide>>,
    mut last_triggered: Local<bool>,
    mut next_monster_state: ResMut<NextState<MonsterState>>,
) {
    // Detect the rising edge of charge.triggered.
    let just_triggered = charge.triggered && !*last_triggered;
    *last_triggered = charge.triggered;

    if !just_triggered || result.resolved {
        return;
    }

    result.resolved = true;

    // Arrow right → safe on right half (x >= 0); arrow left → safe on left half (x <= 0).
    let arrow_right = arrow_side.as_deref().map(|s| s.0).unwrap_or(false);
    let safe = if arrow_right {
        head.position.x >= 0.0
    } else {
        head.position.x <= 0.0
    };
    debug!(
        "charge full — head={:?}  arrow_right={arrow_right}  safe={safe}",
        head.position
    );

    if safe {
        info!("safe!");
        commands.insert_resource(AttackToPeaceDelayTimer(Timer::from_seconds(
            PEACE_DELAY_SECS,
            TimerMode::Once,
        )));
        commands.trigger(SFXEvent::sfx("heart_beat"));
    } else {
        next_monster_state.set(MonsterState::AttackAnimation);
    }
}
