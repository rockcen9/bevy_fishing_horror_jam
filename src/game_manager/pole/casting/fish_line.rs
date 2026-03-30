use super::forward::{CastJitterTimer, CastReturnTimer};
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (draw_taut_fishing_line_gizmo, draw_cast_in_flight_line_gizmo).run_if(
            in_state(GameState::Casting)
                .or_else(in_state(GameState::Waiting))
                .or_else(in_state(GameState::Biting))
                .or_else(in_state(GameState::Reeling)),
        ).run_if(in_state(Pause(false))),
    );
}

fn draw_taut_fishing_line_gizmo(
    mut gizmos: Gizmos,
    tiptop_query: Query<&GlobalTransform, With<PoleTip>>,
    bobber_query: Query<&GlobalTransform, With<super::Bobber>>,
    return_timer: Option<Res<CastReturnTimer>>,
    state: Res<State<GameState>>,
) {
    if *state.get() == GameState::Casting && return_timer.is_none() {
        return;
    }
    let Some(tiptop_transform) = tiptop_query.iter().next() else {
        return;
    };
    let Some(bobber_transform) = bobber_query.iter().next() else {
        return;
    };

    let p0 = tiptop_transform.translation().truncate();
    let p2 = bobber_transform.translation().truncate();

    if *state.get() == GameState::Biting {
        gizmos.line_2d(p0, p2, bevy::color::Color::WHITE);
        return;
    }

    let mid_x = (p0.x + p2.x) / 2.0;
    let min_y = p0.y.min(p2.y);
    let sag_amount = 50.0;
    let p1 = Vec2::new(mid_x, min_y - sag_amount);

    let segments = 20;
    let mut prev_point = p0;

    for i in 1..=segments {
        let t = i as f32 / segments as f32;
        let one_minus_t = 1.0 - t;

        let current_point =
            one_minus_t * one_minus_t * p0 + 2.0 * one_minus_t * t * p1 + t * t * p2;

        gizmos.line_2d(prev_point, current_point, bevy::color::Color::WHITE);
        prev_point = current_point;
    }
}

fn draw_cast_in_flight_line_gizmo(
    mut gizmos: Gizmos,
    tiptop_query: Query<&GlobalTransform, With<PoleTip>>,
    bobber_query: Query<&GlobalTransform, With<LakePoint1>>,
    jitter_timer: Option<Res<CastJitterTimer>>,
    return_timer: Option<Res<CastReturnTimer>>,
    state: Res<State<GameState>>,
) {
    if *state.get() != GameState::Casting {
        return;
    }
    let Some(jitter) = jitter_timer else {
        return;
    };
    if return_timer.is_some() {
        return;
    }

    let Some(tiptop_transform) = tiptop_query.iter().next() else {
        return;
    };
    let Some(bobber_transform) = bobber_query.iter().next() else {
        return;
    };

    let tip = tiptop_transform.translation().truncate();
    let target = bobber_transform.translation().truncate();

    let progress = jitter.0.fraction();
    let hook = tip.lerp(target, progress);

    gizmos.line_2d(tip, hook, bevy::color::Color::WHITE);
}
