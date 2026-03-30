use crate::loading_bar::{LOADING_BAR_RING_SIZE_PX, LoadingBarMaterial};
use crate::prelude::*;
use bevy_tweening::{lens::TransformScaleLens, *};
use std::time::Duration;

use super::items::HueShiftFishMaterial;
use super::{
    BACKPACK_WIDTH, LOADING_BAR_HOVER_DURATION_SECS, PADDING, PanelCloseGoesToDead,
    PanelCloseGoesToEnd, RestartGameEvent,
};

pub(super) const DISPLAY_SIZE: f32 = 128.0;

fn journal_display_size() -> (f32, f32) {
    (DISPLAY_SIZE, DISPLAY_SIZE)
}

fn display_size_for_path(image_path: &str) -> (f32, f32) {
    if image_path.contains("journal") {
        journal_display_size()
    } else {
        (DISPLAY_SIZE, DISPLAY_SIZE)
    }
}
pub(super) const DISPLAY_GAP: f32 = 8.0;
use LOADING_BAR_HOVER_DURATION_SECS as LAST_ITEM_HOVER_DURATION_SECS;

// ── Markers / Resources ─────────────────────────────────────────────────────

#[derive(Component)]
pub(super) struct LastItemDisplay;

#[derive(Component)]
pub(super) struct LastItemLoadingBar;

#[derive(Resource, Default)]
pub(super) struct LastCaughtDescription {
    pub(super) name: String,
    pub(super) image_path: String,
    pub(super) text: String,
    pub(super) log: String,
    pub(super) hue_shift: f32,
}

#[derive(Resource, Default)]
struct LastItemHoverState {
    elapsed: f32,
    active: bool,
}

/// Holds a description update that will be applied to [`LastCaughtDescription`]
/// after a short delay (so the catch animation finishes first).
#[derive(Resource, Default)]
pub(super) struct DeferredLastItemDescription {
    pub(super) name: String,
    pub(super) image_path: String,
    pub(super) text: String,
    pub(super) log: String,
    pub(super) hue_shift: f32,
    pub(super) timer: Option<Timer>,
    /// When true, automatically opens the description panel when the timer fires.
    pub(super) auto_open: bool,
    /// When true, closing the auto-opened panel transitions to `GameState::End`.
    pub(super) goes_to_end: bool,
    /// When true, closing the auto-opened panel transitions to `GameState::Dead`.
    pub(super) goes_to_dead: bool,
}

// ── Plugin ──────────────────────────────────────────────────────────────────

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<LastCaughtDescription>()
        .init_resource::<LastItemHoverState>()
        .init_resource::<DeferredLastItemDescription>()
        .add_systems(OnEnter(Screen::Gameplay), spawn_last_item_display)
        .add_observer(on_restart_game)
        .add_systems(
            Update,
            (
                flush_pending_last_item_description,
                sync_last_item_display_material.run_if(resource_changed::<LastCaughtDescription>),
                tick_last_item_hover,
                sync_last_item_hover_loading_bar,
            )
                .chain()
                .in_set(PausableSystems),
        );
}

// ── Systems ──────────────────────────────────────────────────────────────────

fn flush_pending_last_item_description(
    time: Res<Time>,
    mut pending: ResMut<DeferredLastItemDescription>,
    mut description: ResMut<LastCaughtDescription>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut panel_close_goes_to_end: ResMut<PanelCloseGoesToEnd>,
    mut panel_close_goes_to_dead: ResMut<PanelCloseGoesToDead>,
    mut commands: Commands,
) {
    let Some(ref mut timer) = pending.timer else {
        return;
    };
    if !timer.tick(time.delta()).just_finished() {
        return;
    }
    description.name = pending.name.clone();
    description.image_path = pending.image_path.clone();
    description.text = pending.text.clone();
    description.log = pending.log.clone();
    description.hue_shift = pending.hue_shift;
    if pending.auto_open {
        next_game_state.set(GameState::UIOpened);
        panel_close_goes_to_end.0 = pending.goes_to_end;
        panel_close_goes_to_dead.0 = pending.goes_to_dead;
        commands.trigger(SpawnDescriptionPanelEvent {
            name: pending.name.clone(),
            image_path: pending.image_path.clone(),
            text: pending.text.clone(),
            log: pending.log.clone(),
            hue_shift: pending.hue_shift,
        });
    }
    pending.timer = None;
    pending.auto_open = false;
    pending.goes_to_end = false;
    pending.goes_to_dead = false;
}

fn on_restart_game(
    _trigger: On<RestartGameEvent>,
    mut pending: ResMut<DeferredLastItemDescription>,
    mut hover_state: ResMut<LastItemHoverState>,
) {
    *pending = DeferredLastItemDescription::default();
    *hover_state = LastItemHoverState::default();
}

// ── Spawn ────────────────────────────────────────────────────────────────────

fn spawn_last_item_display(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut rc_materials: ResMut<Assets<HueShiftFishMaterial>>,
    mut lb_materials: ResMut<Assets<LoadingBarMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let x = -960.0 + BACKPACK_WIDTH / 2.0 + PADDING;
    let y = 0.0;

    // Journal texture is 310×443; spawn with correct aspect ratio.
    let (w, h) = journal_display_size();
    commands.spawn((
        Name::new("LastItemDisplay"),
        LastItemDisplay,
        SpriteLayer::Backpack,
        Mesh2d(meshes.add(Rectangle::new(w, h))),
        MeshMaterial2d(rc_materials.add(HueShiftFishMaterial {
            texture: asset_server.load("textures/journal.png"),
            hue_shift: 0.0,
        })),
        Transform::from_xyz(x, y, 10.0),
    ));

    commands.spawn((
        Name::new("LastItemLoadingBar"),
        LastItemLoadingBar,
        SpriteLayer::LoadingBar,
        Mesh2d(meshes.add(Rectangle::new(
            LOADING_BAR_RING_SIZE_PX,
            LOADING_BAR_RING_SIZE_PX,
        ))),
        MeshMaterial2d(lb_materials.add(LoadingBarMaterial::default())),
        Transform::from_xyz(x, y, 45.0),
        Visibility::Hidden,
    ));
}

// ── Systems ──────────────────────────────────────────────────────────────────

fn sync_last_item_display_material(
    description: Res<LastCaughtDescription>,
    display: Query<(Entity, &MeshMaterial2d<HueShiftFishMaterial>, &Mesh2d), With<LastItemDisplay>>,
    mut rc_materials: ResMut<Assets<HueShiftFishMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if description.image_path.is_empty() {
        return;
    }
    let Ok((entity, mat_handle, mesh_handle)) = display.single() else {
        return;
    };
    if let Some(mut mat) = rc_materials.get_mut(&mat_handle.0) {
        mat.texture = asset_server.load(description.image_path.clone());
        mat.hue_shift = description.hue_shift;
    }
    let (w, h) = display_size_for_path(&description.image_path);
    if let Some(mut mesh) = meshes.get_mut(&mesh_handle.0) {
        *mesh = Rectangle::new(w, h).into();
    }

    let tween = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_millis(200),
        TransformScaleLens {
            start: Vec3::ZERO,
            end: Vec3::ONE,
        },
    );
    commands.spawn((
        TweenAnim::new(tween),
        AnimTarget::component::<Transform>(entity),
    ));
}

fn tick_last_item_hover(
    hands: Query<(&Transform, &Visibility), Or<(With<LeftHand>, With<RightHand>)>>,
    display: Query<&Transform, With<LastItemDisplay>>,
    time: Res<Time>,
    mut state: ResMut<LastItemHoverState>,
    description: Res<LastCaughtDescription>,
    mut commands: Commands,
    monster_state: Option<Res<State<GameState>>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let Some(monster_state) = monster_state else {
        return;
    };
    if *monster_state == GameState::Monster || *monster_state == GameState::UIOpened {
        state.active = false;
        state.elapsed = 0.0;
        return;
    }
    let Ok(display_transform) = display.single() else {
        return;
    };

    let display_pos = display_transform.translation.truncate();
    let half = DISPLAY_SIZE / 2.0;

    let is_hovering = hands.iter().any(|(transform, visibility)| {
        if *visibility == Visibility::Hidden {
            return false;
        }
        let hand_pos = transform.translation.truncate();
        (hand_pos.x - display_pos.x).abs() <= half && (hand_pos.y - display_pos.y).abs() <= half
    });

    if is_hovering {
        if !state.active {
            state.active = true;
            state.elapsed = 0.0;
        }
        state.elapsed += time.delta_secs();
        if state.elapsed >= LAST_ITEM_HOVER_DURATION_SECS {
            state.active = false;
            state.elapsed = 0.0;
            if !description.text.is_empty() {
                next_game_state.set(GameState::UIOpened);
                commands.trigger(SpawnDescriptionPanelEvent {
                    name: description.name.clone(),
                    image_path: description.image_path.clone(),
                    text: description.text.clone(),
                    log: description.log.clone(),
                    hue_shift: description.hue_shift,
                });
            }
        }
    } else {
        state.active = false;
        state.elapsed = 0.0;
    }
}

fn sync_last_item_hover_loading_bar(
    state: Res<LastItemHoverState>,
    loading_bar: Query<(&MeshMaterial2d<LoadingBarMaterial>, Entity), With<LastItemLoadingBar>>,
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
            mat.params.x = (state.elapsed / LAST_ITEM_HOVER_DURATION_SECS).clamp(0.0, 1.0);
        }
    } else {
        vis.set_if_neq(Visibility::Hidden);
    }
}
