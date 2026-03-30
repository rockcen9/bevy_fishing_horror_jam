use crate::prelude::*;

use super::{BACKPACK_HEIGHT, BACKPACK_WIDTH, Backpack, BackpackOpenEvent, LOADING_BAR_HOVER_DURATION_SECS};
use crate::loading_bar::{LoadingBar, LoadingBarMaterial};

// ── Resource ───────────────────────────────────────────────────────────────

#[derive(Resource, Default)]
struct BackpackOpenHoverState {
    elapsed: f32,
    active: bool,
}

// ── Plugin ─────────────────────────────────────────────────────────────────

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<BackpackOpenHoverState>().add_systems(
        Update,
        (tick_backpack_open_hover, sync_backpack_hover_loading_bar)
            .chain()
            .in_set(PausableSystems),
    );
}

// ── Systems ────────────────────────────────────────────────────────────────

/// Detects hand-over-backpack overlap, accumulates hover time, and fires
/// "open backpack" when the full LOADING_BAR_HOVER_DURATION_SECS has elapsed.
fn tick_backpack_open_hover(
    hands: Query<(&Transform, &Visibility), Or<(With<LeftHand>, With<RightHand>)>>,
    backpack: Query<&Transform, With<Backpack>>,
    time: Res<Time>,
    mut state: ResMut<BackpackOpenHoverState>,
    mut commands: Commands,
    monster_state: Option<Res<State<GameState>>>,
) {
    let Some(monster_state) = monster_state else {
        debug!("backpack_hover: no GameState resource, skipping");
        return;
    };
    if *monster_state != GameState::Monster
        && *monster_state != GameState::Idle
        && *monster_state != GameState::Waiting
    {
        state.active = false;
        state.elapsed = 0.0;
        debug!("backpack_hover: wrong state ({:?}), skipping", *monster_state);
        return;
    }
    let Ok(bp_transform) = backpack.single() else {
        debug!("backpack_hover: no Backpack entity found");
        return;
    };

    let bp_pos = bp_transform.translation.truncate();
    let half_w = BACKPACK_WIDTH / 2.0;
    let half_h = BACKPACK_HEIGHT / 2.0;

    let is_hovering = hands.iter().any(|(transform, visibility)| {
        let hand_pos = transform.translation.truncate();
        debug!(
            "backpack_hover: hand vis={:?} pos={:?} bp_pos={:?} dx={:.1} dy={:.1} half_w={} half_h={}",
            visibility, hand_pos, bp_pos,
            (hand_pos.x - bp_pos.x).abs(),
            (hand_pos.y - bp_pos.y).abs(),
            half_w, half_h
        );
        if *visibility == Visibility::Hidden {
            return false;
        }
        (hand_pos.x - bp_pos.x).abs() <= half_w && (hand_pos.y - bp_pos.y).abs() <= half_h
    });

    debug!("backpack_hover: is_hovering={} elapsed={:.2}", is_hovering, state.elapsed);

    if is_hovering {
        if !state.active {
            state.active = true;
            state.elapsed = 0.0;
        }
        state.elapsed += time.delta_secs();
        if state.elapsed >= LOADING_BAR_HOVER_DURATION_SECS {
            debug!("Open backpack!");
            commands.trigger(BackpackOpenEvent);
            state.active = false;
            state.elapsed = 0.0;
        }
    } else {
        state.active = false;
        state.elapsed = 0.0;
    }
}

/// Shows/hides the loading bar and syncs its progress uniform to the hover timer.
fn sync_backpack_hover_loading_bar(
    state: Res<BackpackOpenHoverState>,
    loading_bar: Query<(&MeshMaterial2d<LoadingBarMaterial>, Entity), With<LoadingBar>>,
    mut materials: ResMut<Assets<LoadingBarMaterial>>,
    mut visibility: Query<&mut Visibility>,
) {
    let Ok((mat_handle, entity)) = loading_bar.single() else {
        return;
    };
    let Ok(mut vis) = visibility.get_mut(entity) else {
        return;
    };

    if state.active {
        vis.set_if_neq(Visibility::Visible);
        if let Some(mut mat) = materials.get_mut(&mat_handle.0) {
            mat.params.x = (state.elapsed / LOADING_BAR_HOVER_DURATION_SECS).clamp(0.0, 1.0);
        }
    } else {
        vis.set_if_neq(Visibility::Hidden);
    }
}
