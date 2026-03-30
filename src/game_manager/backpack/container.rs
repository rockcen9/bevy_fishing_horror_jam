use crate::loading_bar::{LOADING_BAR_RING_SIZE_PX, LoadingBarMaterial};
use crate::prelude::*;
use bevy_intl::I18n;

use super::{
    BackpackCloseEvent, BackpackIsOpen, BackpackOpenEvent, FishCaughtEvent,
    LOADING_BAR_HOVER_DURATION_SECS, PanelCloseGoesToDead, PanelCloseGoesToEnd, RestartGameEvent,
    items::{Item, PrefabId, PrefabList, hue_shift_from_entity},
    last_item::{DeferredLastItemDescription, LastCaughtDescription},
};

// ── Layout constants ────────────────────────────────────────────────────────

const CONTAINER_W: f32 = 960.0;
const CONTAINER_H: f32 = 640.0;

// Side-by-side layout: container sits to the left of the description panel.
const SIDE_PANEL_GAP: f32 = 16.0;
const DESC_PANEL_W: f32 = 1920.0 * 0.33; // must match PANEL_W in description_panel.rs
const CONTAINER_WORLD_X: f32 = -(DESC_PANEL_W + SIDE_PANEL_GAP) / 2.0;

use LOADING_BAR_HOVER_DURATION_SECS as ITEM_HOVER_REVEAL_DURATION_SECS;
const SLOT_SIZE: f32 = 160.0;
const SLOT_GAP: f32 = 16.0;
const GRID_COLS: u32 = 4;
const GRID_ROWS: u32 = 3;

const CLOSE_BTN_SIZE: f32 = 80.0;
const CLOSE_BTN_MARGIN: f32 = 10.0;
const CLOSE_LOADING_BAR_RING_SIZE_PX: f32 = 160.0;
use LOADING_BAR_HOVER_DURATION_SECS as CONTAINER_CLOSE_HOVER_DURATION_SECS;

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Returns the world-space XY position for a given backpack slot index.
fn slot_world_pos(slot: u32) -> Vec2 {
    let total_w = GRID_COLS as f32 * SLOT_SIZE + (GRID_COLS - 1) as f32 * SLOT_GAP;
    let total_h = GRID_ROWS as f32 * SLOT_SIZE + (GRID_ROWS - 1) as f32 * SLOT_GAP;
    let origin_x = -total_w / 2.0 + SLOT_SIZE / 2.0;
    let origin_y = total_h / 2.0 - SLOT_SIZE / 2.0;
    let row = slot / GRID_COLS;
    let col = slot % GRID_COLS;
    Vec2::new(
        CONTAINER_WORLD_X + origin_x + col as f32 * (SLOT_SIZE + SLOT_GAP),
        origin_y - row as f32 * (SLOT_SIZE + SLOT_GAP),
    )
}

// ── Markers ─────────────────────────────────────────────────────────────────

#[derive(Component)]
pub struct Container;

#[derive(Component)]
struct CloseButton;

#[derive(Component)]
struct CloseLoadingBar;

#[derive(Component)]
struct ItemHoverLoadingBar;

#[derive(Component)]
struct ScreenOverlay;

// ── Resources ───────────────────────────────────────────────────────────────

#[derive(Resource, Default)]
struct ContainerCloseHoverState {
    elapsed: f32,
    active: bool,
}

#[derive(Resource, Default)]
struct BackpackItemHoverState {
    elapsed: f32,
    active: bool,
    hovered_item: Option<Entity>,
    hovered_world_pos: Vec2,
}

/// Tracks which grid slot the next caught item will occupy.
/// Slot 0 is always the journal; fish start at slot 1.
#[derive(Resource)]
struct BackpackSlotTracker {
    next_item_slot: u32,
}

impl Default for BackpackSlotTracker {
    fn default() -> Self {
        Self { next_item_slot: 1 }
    }
}

// ── Plugin ──────────────────────────────────────────────────────────────────

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ContainerCloseHoverState>()
        .init_resource::<BackpackSlotTracker>()
        .init_resource::<BackpackItemHoverState>()
        .add_systems(OnEnter(Screen::Gameplay), spawn_journal_item)
        .add_systems(
            Update,
            (
                (tick_close_button_hover, sync_close_loading_bar_to_hover).chain(),
                (tick_item_hover, sync_item_hover_loading_bar).chain(),
            )
                .run_if(in_state(Pause(true))),
        )
        .add_systems(
            Update,
            initialize_journal_slot_description.run_if(resource_added::<ItemDataBalance>),
        )
        .add_observer(on_backpack_open)
        .add_observer(on_backpack_close)
        .add_observer(on_fish_caught)
        // .add_observer(on_show_description_panel)
        // .add_observer(on_hide_description_panel)
        .add_observer(on_restart_game);
}

// ── Observers ───────────────────────────────────────────────────────────────

fn on_backpack_open(
    _trigger: On<BackpackOpenEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut lb_materials: ResMut<Assets<LoadingBarMaterial>>,
    asset_server: Res<AssetServer>,
    palette: Res<crate::theme::palette::ColorPalette>,
    mut next_pause: ResMut<NextState<Pause>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut backpack_is_open: ResMut<BackpackIsOpen>,
    mut items: Query<&mut Visibility, With<Item>>,
) {
    spawn_backpack_ui(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut lb_materials,
        &asset_server,
        &palette,
    );
    for mut vis in &mut items {
        vis.set_if_neq(Visibility::Visible);
    }
    next_pause.set(Pause(true));
    next_game_state.set(GameState::UIOpened);
    backpack_is_open.0 = true;
}

fn on_backpack_close(
    _trigger: On<BackpackCloseEvent>,
    container: Query<Entity, With<Container>>,
    close_bar: Query<Entity, With<CloseLoadingBar>>,
    overlay: Query<Entity, With<ScreenOverlay>>,
    item_hover_bar: Query<Entity, With<ItemHoverLoadingBar>>,
    mut next_pause: ResMut<NextState<Pause>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut backpack_is_open: ResMut<BackpackIsOpen>,
    mut close_state: ResMut<ContainerCloseHoverState>,
    mut items: Query<&mut Visibility, With<Item>>,
    mut commands: Commands,
) {
    for e in &container {
        commands.entity(e).insert(AnimDespawn);
    }
    for e in &close_bar {
        commands.entity(e).despawn();
    }
    for e in &overlay {
        commands.entity(e).despawn();
    }
    for e in &item_hover_bar {
        commands.entity(e).despawn();
    }
    for mut vis in &mut items {
        vis.set_if_neq(Visibility::Hidden);
    }
    next_pause.set(Pause(false));
    next_game_state.set(GameState::Idle);
    backpack_is_open.0 = false;
    *close_state = ContainerCloseHoverState::default();
    commands.trigger(DespawnDescriptionPanelEvent);
}

// ── Systems ─────────────────────────────────────────────────────────────────

fn tick_close_button_hover(
    hands: Query<(&Transform, &Visibility), Or<(With<LeftHand>, With<RightHand>)>>,
    close_btn: Query<&GlobalTransform, With<CloseButton>>,
    time: Res<Time>,
    mut state: ResMut<ContainerCloseHoverState>,
    mut commands: Commands,
) {
    let Ok(btn_gt) = close_btn.single() else {
        return;
    };
    let btn_pos = btn_gt.translation().truncate();
    let half = CLOSE_BTN_SIZE / 2.0;

    let is_hovering = hands.iter().any(|(transform, visibility)| {
        if *visibility == Visibility::Hidden {
            return false;
        }
        let hand_pos = transform.translation.truncate();
        (hand_pos.x - btn_pos.x).abs() <= half && (hand_pos.y - btn_pos.y).abs() <= half
    });

    if is_hovering {
        if !state.active {
            state.active = true;
            state.elapsed = 0.0;
        }
        state.elapsed += time.delta_secs();
        if state.elapsed >= CONTAINER_CLOSE_HOVER_DURATION_SECS {
            state.active = false;
            state.elapsed = 0.0;
            commands.trigger(BackpackCloseEvent);
        }
    } else {
        state.active = false;
        state.elapsed = 0.0;
    }
}

fn sync_close_loading_bar_to_hover(
    state: Res<ContainerCloseHoverState>,
    close_bar: Query<(&MeshMaterial2d<LoadingBarMaterial>, Entity), With<CloseLoadingBar>>,
    mut materials: ResMut<Assets<LoadingBarMaterial>>,
    mut visibility: Query<&mut Visibility>,
) {
    let Ok((mat_handle, entity)) = close_bar.single() else {
        return;
    };
    let Ok(mut vis) = visibility.get_mut(entity) else {
        return;
    };

    if state.active {
        vis.set_if_neq(Visibility::Visible);
        if let Some(mut mat) = materials.get_mut(&mat_handle.0) {
            mat.params.x = (state.elapsed / CONTAINER_CLOSE_HOVER_DURATION_SECS).clamp(0.0, 1.0);
        }
    } else {
        vis.set_if_neq(Visibility::Hidden);
    }
}

fn tick_item_hover(
    hands: Query<(&Transform, &Visibility), Or<(With<LeftHand>, With<RightHand>)>>,
    items: Query<(Entity, &GlobalTransform, &PrefabId), With<Item>>,
    container: Query<(), With<Container>>,
    time: Res<Time>,
    mut state: ResMut<BackpackItemHoverState>,
    balance: Option<Res<ItemDataBalance>>,
    i18n: Res<I18n>,
    mut commands: Commands,
) {
    if container.is_empty() {
        return;
    }
    let half = SLOT_SIZE / 2.0;

    let hovered = items.iter().find(|(_, gt, _)| {
        let item_pos = gt.translation().truncate();
        hands.iter().any(|(t, vis)| {
            if *vis == Visibility::Hidden {
                return false;
            }
            let hand_pos = t.translation.truncate();
            (hand_pos.x - item_pos.x).abs() <= half && (hand_pos.y - item_pos.y).abs() <= half
        })
    });

    if let Some((entity, gt, prefab_id)) = hovered {
        let world_pos = gt.translation().truncate();

        if state.hovered_item != Some(entity) {
            state.hovered_item = Some(entity);
            state.elapsed = 0.0;
            state.active = true;
            state.hovered_world_pos = world_pos;
        }

        state.elapsed += time.delta_secs();
        if state.elapsed >= ITEM_HOVER_REVEAL_DURATION_SECS {
            state.active = false;
            state.elapsed = 0.0;
            state.hovered_item = None;

            if let Some(balance) = balance {
                if let Some(item_data) = balance.get_by_id(&prefab_id.0) {
                    let t = i18n.translation("items");
                    let hue_shift = if item_data.random_color {
                        hue_shift_from_entity(entity)
                    } else {
                        0.0
                    };
                    commands.trigger(SpawnDescriptionPanelEvent {
                        name: t.t_with_gender(&item_data.id, "name"),
                        image_path: item_data.file_path.clone(),
                        text: t.t_with_gender(&item_data.id, "desc"),
                        log: t.t_with_gender(&item_data.id, "log"),
                        hue_shift,
                    });
                }
            }
        }
    } else {
        state.active = false;
        state.elapsed = 0.0;
        state.hovered_item = None;
    }
}

fn sync_item_hover_loading_bar(
    state: Res<BackpackItemHoverState>,
    mut loading_bar: Query<
        (&MeshMaterial2d<LoadingBarMaterial>, Entity, &mut Transform),
        With<ItemHoverLoadingBar>,
    >,
    mut materials: ResMut<Assets<LoadingBarMaterial>>,
    mut visibility: Query<&mut Visibility>,
) {
    let Ok((mat_handle, entity, mut lb_transform)) = loading_bar.single_mut() else {
        return;
    };
    let Ok(mut vis) = visibility.get_mut(entity) else {
        return;
    };

    if state.active {
        vis.set_if_neq(Visibility::Visible);
        lb_transform.translation = state.hovered_world_pos.extend(45.0);
        if let Some(mut mat) = materials.get_mut(&mat_handle.0) {
            mat.params.x = (state.elapsed / ITEM_HOVER_REVEAL_DURATION_SECS).clamp(0.0, 1.0);
        }
    } else {
        vis.set_if_neq(Visibility::Hidden);
    }
}

fn initialize_journal_slot_description(
    balance: Res<ItemDataBalance>,
    i18n: Res<I18n>,
    mut description: ResMut<LastCaughtDescription>,
) {
    if let Some(item_data) = balance.get_by_id(&PrefabList::Journal.prefab_id().0) {
        let t = i18n.translation("items");
        description.name = t.t_with_gender(&item_data.id, "name");
        description.image_path = item_data.file_path.clone();
        description.text = t.t_with_gender(&item_data.id, "desc");
        description.log = t.t_with_gender(&item_data.id, "log");
        description.hue_shift = 0.0;
    }
}

fn on_fish_caught(
    trigger: On<FishCaughtEvent>,
    mut inventory: ResMut<BackpackSlotTracker>,
    mut commands: Commands,
    balance: Option<Res<ItemDataBalance>>,
    i18n: Res<I18n>,
    mut pending: ResMut<DeferredLastItemDescription>,
) {
    let slot = inventory.next_item_slot;
    if slot >= GRID_COLS * GRID_ROWS {
        return; // backpack full
    }

    let pos = slot_world_pos(slot);
    let row = slot / GRID_COLS;
    let col = slot % GRID_COLS;

    let fish_entity = commands
        .spawn((
            Name::new(format!("FishItem({row},{col})")),
            Item,
            PrefabId(trigger.event().prefab_id.clone()),
            Transform::from_xyz(pos.x, pos.y, 2.0),
            Visibility::Hidden,
        ))
        .id();

    inventory.next_item_slot += 1;

    if let Some(balance) = balance {
        if let Some(item_data) = balance.get_by_id(&trigger.event().prefab_id) {
            let t = i18n.translation("items");
            pending.name = t.t_with_gender(&item_data.id, "name");
            pending.image_path = item_data.file_path.clone();
            pending.text = t.t_with_gender(&item_data.id, "desc");
            pending.log = t.t_with_gender(&item_data.id, "log");
            pending.hue_shift = if item_data.random_color {
                hue_shift_from_entity(fish_entity)
            } else {
                0.0
            };
            let id = trigger.event().prefab_id.as_str();
            pending.auto_open = id == PrefabList::Target3.prefab_id().0.as_str()
                || id == PrefabList::Kai.prefab_id().0.as_str();
            pending.goes_to_end = id == PrefabList::Target3.prefab_id().0.as_str();
            pending.goes_to_dead = id == PrefabList::Kai.prefab_id().0.as_str();
            pending.timer = Some(Timer::from_seconds(0.2, TimerMode::Once));
        }
    }
}

fn on_restart_game(
    _trigger: On<RestartGameEvent>,
    items: Query<(Entity, &PrefabId), With<Item>>,
    container: Query<Entity, With<Container>>,
    close_bar: Query<Entity, With<CloseLoadingBar>>,
    overlay: Query<Entity, With<ScreenOverlay>>,
    item_hover_bar: Query<Entity, With<ItemHoverLoadingBar>>,
    mut inventory: ResMut<BackpackSlotTracker>,
    mut backpack_is_open: ResMut<BackpackIsOpen>,
    mut panel_close_goes_to_end: ResMut<PanelCloseGoesToEnd>,
    mut panel_close_goes_to_dead: ResMut<PanelCloseGoesToDead>,
    balance: Option<Res<ItemDataBalance>>,
    i18n: Res<bevy_intl::I18n>,
    mut description: ResMut<LastCaughtDescription>,
    mut commands: Commands,
) {
    let journal_id = PrefabList::Journal.prefab_id().0;
    for (entity, prefab_id) in &items {
        if prefab_id.0 != journal_id {
            commands.entity(entity).despawn();
        }
    }
    for e in &container {
        commands.entity(e).despawn();
    }
    for e in &close_bar {
        commands.entity(e).despawn();
    }
    for e in &overlay {
        commands.entity(e).despawn();
    }
    for e in &item_hover_bar {
        commands.entity(e).despawn();
    }
    inventory.next_item_slot = 1;
    backpack_is_open.0 = false;
    panel_close_goes_to_end.0 = false;
    panel_close_goes_to_dead.0 = false;

    if let Some(balance) = balance {
        if let Some(item_data) = balance.get_by_id(&journal_id) {
            let t = i18n.translation("items");
            description.name = t.t_with_gender(&item_data.id, "name");
            description.image_path = item_data.file_path.clone();
            description.text = t.t_with_gender(&item_data.id, "desc");
            description.log = t.t_with_gender(&item_data.id, "log");
            description.hue_shift = 0.0;
        }
    }
}

// ── Spawn ────────────────────────────────────────────────────────────────────

fn spawn_journal_item(mut commands: Commands) {
    let pos = slot_world_pos(0);
    commands.spawn((
        Name::new("JournalItem"),
        Item,
        PrefabList::Journal.prefab_id(),
        Transform::from_xyz(pos.x, pos.y, 2.0),
        Visibility::Hidden,
    ));
}

fn spawn_backpack_ui(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    lb_materials: &mut Assets<LoadingBarMaterial>,
    asset_server: &AssetServer,
    palette: &crate::theme::palette::ColorPalette,
) {
    let bg_mat = materials.add(ColorMaterial {
        color: palette.panel_bg.with_alpha(0.5),
        ..default()
    });
    let slot_mat = materials.add(ColorMaterial {
        color: palette.backpack_slot,
        ..default()
    });
    let close_mat = materials.add(ColorMaterial {
        texture: Some(asset_server.load("textures/close.png")),
        ..default()
    });
    let overlay_mat = materials.add(ColorMaterial {
        color: palette.panel_bg.with_alpha(0.5),
        ..default()
    });
    let close_lb_mat = lb_materials.add(LoadingBarMaterial::default());

    let slot_mesh = meshes.add(Rectangle::new(SLOT_SIZE, SLOT_SIZE));

    // Grid origin (top-left slot center, relative to container center)
    let total_w = GRID_COLS as f32 * SLOT_SIZE + (GRID_COLS - 1) as f32 * SLOT_GAP;
    let total_h = GRID_ROWS as f32 * SLOT_SIZE + (GRID_ROWS - 1) as f32 * SLOT_GAP;
    let origin_x = -total_w / 2.0 + SLOT_SIZE / 2.0;
    let origin_y = total_h / 2.0 - SLOT_SIZE / 2.0;

    // Close button: top-right corner of the panel
    let close_x = CONTAINER_W / 2.0 - CLOSE_BTN_SIZE / 2.0 - CLOSE_BTN_MARGIN;
    let close_y = CONTAINER_H / 2.0 - CLOSE_BTN_SIZE / 2.0 - CLOSE_BTN_MARGIN;

    commands
        .spawn((
            Name::new("BackpackContainer"),
            Container,
            Mesh2d(meshes.add(Rectangle::new(CONTAINER_W, CONTAINER_H))),
            MeshMaterial2d(bg_mat),
            Transform::from_xyz(CONTAINER_WORLD_X, 0.0, 20.0),
            AnimSpawnOn,
        ))
        .with_children(|parent| {
            // Item grid slots (visual backgrounds only)
            for row in 0..GRID_ROWS {
                for col in 0..GRID_COLS {
                    let x = origin_x + col as f32 * (SLOT_SIZE + SLOT_GAP);
                    let y = origin_y - row as f32 * (SLOT_SIZE + SLOT_GAP);

                    parent.spawn((
                        Name::new(format!("Slot({row},{col})")),
                        Mesh2d(slot_mesh.clone()),
                        MeshMaterial2d(slot_mat.clone()),
                        Transform::from_xyz(x, y, 1.0),
                    ));
                }
            }

            // Close button (top-right corner)
            parent.spawn((
                Name::new("CloseButton"),
                CloseButton,
                Mesh2d(meshes.add(Rectangle::new(CLOSE_BTN_SIZE, CLOSE_BTN_SIZE))),
                MeshMaterial2d(close_mat),
                Transform::from_xyz(close_x, close_y, 1.0),
            ));
        });

    // Fullscreen overlay: root entity centered at world origin so it covers the entire screen.
    commands.spawn((
        Name::new("BackpackScreenOverlay"),
        ScreenOverlay,
        Mesh2d(meshes.add(Rectangle::new(crate::GAME_WIDTH, crate::GAME_HEIGHT))),
        MeshMaterial2d(overlay_mat),
        Transform::from_xyz(0.0, 0.0, 15.0),
    ));

    // Close loading bar: starts hidden, becomes visible on hover.
    commands.spawn((
        Name::new("CloseLoadingBar"),
        CloseLoadingBar,
        SpriteLayer::LoadingBar,
        Mesh2d(meshes.add(Rectangle::new(
            CLOSE_LOADING_BAR_RING_SIZE_PX,
            CLOSE_LOADING_BAR_RING_SIZE_PX,
        ))),
        MeshMaterial2d(close_lb_mat),
        Transform::from_xyz(CONTAINER_WORLD_X + close_x, close_y, 45.0),
        Visibility::Hidden,
    ));

    // Item hover loading bar: starts hidden, position updated each frame.
    commands.spawn((
        Name::new("ItemHoverLoadingBar"),
        ItemHoverLoadingBar,
        SpriteLayer::LoadingBar,
        Mesh2d(meshes.add(Rectangle::new(
            LOADING_BAR_RING_SIZE_PX,
            LOADING_BAR_RING_SIZE_PX,
        ))),
        MeshMaterial2d(lb_materials.add(LoadingBarMaterial::default())),
        Transform::from_xyz(CONTAINER_WORLD_X, 0.0, 45.0),
        Visibility::Hidden,
    ));
}
