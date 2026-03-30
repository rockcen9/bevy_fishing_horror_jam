use crate::game_manager::pole::{Bobber, FishCatchSequence, HookedFish};
use crate::prelude::*;

mod catch_anim;
mod container;
pub use container::Container;
mod hand_pointer;
mod hide_ui;
mod items;
mod last_item;
mod loading_bar;
mod spawn;

pub(crate) use items::{HueShiftFishMaterial, PrefabId, PrefabList};

/// Fired when the backpack hover timer completes.
#[derive(Event)]
pub(crate) struct BackpackOpenEvent;

/// Fired when the close button hover timer completes.
#[derive(Event)]
pub(crate) struct BackpackCloseEvent;

/// Fired when the player successfully catches a fish (QTE success).
#[derive(Event)]
pub(crate) struct FishCaughtEvent {
    pub(crate) prefab_id: smol_str::SmolStr,
}

/// Fired when the dead sequence completes; each subsystem resets itself.
#[derive(Event)]
pub(crate) struct RestartGameEvent;

/// Shared hover duration (seconds) for all loading-bar interactions:
/// backpack open, backpack close, item reveal, last-item display, and description panel close.
pub(crate) const LOADING_BAR_HOVER_DURATION_SECS: f32 = 1.0;

/// Tracks whether the backpack container is currently open.
#[derive(Resource, Default)]
pub(crate) struct BackpackIsOpen(pub bool);

/// When true, closing the description panel transitions to `GameState::End`
/// instead of `GameState::Idle`. Set when Target3 is caught.
#[derive(Resource, Default)]
pub(crate) struct PanelCloseGoesToEnd(pub bool);

/// When true, closing the description panel transitions to `GameState::Dead`
/// instead of `GameState::Idle`. Set when Kai is caught.
#[derive(Resource, Default)]
pub(crate) struct PanelCloseGoesToDead(pub bool);

/// Marker component for the backpack entity.
#[derive(Component)]
pub(super) struct Backpack;

pub(super) const BACKPACK_WIDTH: f32 = 180.0;
pub(super) const BACKPACK_HEIGHT: f32 = 190.0;
pub(super) const PADDING: f32 = 20.0;

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<BackpackIsOpen>()
        .init_resource::<PanelCloseGoesToEnd>()
        .init_resource::<PanelCloseGoesToDead>();
    catch_anim::plugin(app);
    items::plugin(app);
    last_item::plugin(app);
    loading_bar::plugin(app);
    hand_pointer::plugin(app);
    hide_ui::plugin(app);
    spawn::plugin(app);
    container::plugin(app);
    app.add_systems(OnEnter(GameState::Succeeding), send_fish_caught_on_succeed);
}

fn send_fish_caught_on_succeed(
    mut commands: Commands,
    mut bucket: ResMut<FishCatchSequence>,
    bobber: Query<&HookedFish, With<Bobber>>,
) {
    let Ok(hooked) = bobber.single() else {
        return;
    };
    commands.trigger(FishCaughtEvent {
        prefab_id: hooked.0.0.clone(),
    });

    bucket.catch_index += 1;
}
