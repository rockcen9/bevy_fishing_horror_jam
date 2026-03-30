use bevy::sprite::Anchor;
use bevy::text::TextBounds;

use crate::game_manager::backpack::{
    BackpackIsOpen, HueShiftFishMaterial, LOADING_BAR_HOVER_DURATION_SECS, PanelCloseGoesToDead,
    PanelCloseGoesToEnd,
};
use crate::loading_bar::LoadingBarMaterial;
use crate::prelude::*;

// ── Layout ───────────────────────────────────────────────────────────────────

const DESCRIPTION_PANEL_W: f32 = 1920.0 * 0.33;
const DESCRIPTION_PANEL_H: f32 = 1080.0 * 0.60;
const DESCRIPTION_PANEL_PADDING_PX: f32 = 36.0;

// Side-by-side layout: panel sits to the right of the backpack container.
const BACKPACK_DESCRIPTION_GAP_PX: f32 = 16.0;
const BACKPACK_CONTAINER_W: f32 = 960.0; // must match CONTAINER_W in container.rs
pub(crate) const DESCRIPTION_PANEL_WORLD_X: f32 =
    (BACKPACK_CONTAINER_W + BACKPACK_DESCRIPTION_GAP_PX) / 2.0;

const DESCRIPTION_IMAGE_SIZE_PX: f32 = 160.0;
const DESCRIPTION_IMAGE_Y: f32 =
    DESCRIPTION_PANEL_H / 2.0 - DESCRIPTION_PANEL_PADDING_PX - DESCRIPTION_IMAGE_SIZE_PX / 2.0;
const DESCRIPTION_IMAGE_BG_SIZE_PX: f32 = DESCRIPTION_IMAGE_SIZE_PX + 16.0;

const DESCRIPTION_NAME_TOP_Y: f32 = DESCRIPTION_IMAGE_Y - DESCRIPTION_IMAGE_SIZE_PX / 2.0 - 12.0;
const DESCRIPTION_DIVIDER_Y: f32 = DESCRIPTION_NAME_TOP_Y - 48.0;
const DESCRIPTION_CONTENT_TOP_Y: f32 = DESCRIPTION_DIVIDER_Y - 20.0;
const DESCRIPTION_CONTENT_W: f32 = DESCRIPTION_PANEL_W - DESCRIPTION_PANEL_PADDING_PX * 2.0;
const DESCRIPTION_DESC_H: f32 = 160.0;
// Log sits above desc: compute log height as total remaining minus desc section.
const DESCRIPTION_LOG_H: f32 = DESCRIPTION_CONTENT_TOP_Y
    - (-DESCRIPTION_PANEL_H / 2.0 + DESCRIPTION_PANEL_PADDING_PX)
    - 8.0
    - DESCRIPTION_DESC_H;
const DESCRIPTION_DESC_TOP_Y: f32 = DESCRIPTION_CONTENT_TOP_Y - DESCRIPTION_LOG_H - 8.0;

const DESCRIPTION_CLOSE_BTN_SIZE_PX: f32 = 80.0;
const DESCRIPTION_CLOSE_BTN_MARGIN_PX: f32 = 10.0;
const DESCRIPTION_CLOSE_LOADING_BAR_SIZE_PX: f32 = 160.0;
use LOADING_BAR_HOVER_DURATION_SECS as DESCRIPTION_CLOSE_HOVER_DURATION_SECS;

// Colors come from ColorPalette resource — see on_show_description_panel

fn description_image_size_for_path(image_path: &str) -> (f32, f32) {
    if image_path.contains("journal") {
        (DESCRIPTION_IMAGE_SIZE_PX, DESCRIPTION_IMAGE_SIZE_PX)
    } else {
        (DESCRIPTION_IMAGE_SIZE_PX, DESCRIPTION_IMAGE_SIZE_PX)
    }
}

// ── Markers ──────────────────────────────────────────────────────────────────

#[derive(Component, Debug, Reflect)]
pub struct DescriptionPanel;
#[derive(Component, Debug, Reflect)]
pub struct DescriptionSubPanel;
#[derive(Component)]
struct DescriptionCloseButton;

#[derive(Component)]
struct DescriptionCloseLoadingBar;

// ── Resources ────────────────────────────────────────────────────────────────

#[derive(Resource, Default)]
struct DescriptionCloseHoverState {
    elapsed: f32,
    active: bool,
}

/// Tracks whether this panel was the one that triggered the pause,
/// so it can safely unpause on close without stomping backpack state.
#[derive(Resource, Default)]
struct DescriptionPanelPauseState {
    triggered_pause: bool,
}

// ── Events ───────────────────────────────────────────────────────────────────

#[derive(Event)]
pub(crate) struct SpawnDescriptionPanelEvent {
    pub(crate) name: String,
    pub(crate) image_path: String,
    pub(crate) text: String,
    pub(crate) log: String,
    pub(crate) hue_shift: f32,
}

#[derive(Event)]
pub(crate) struct DespawnDescriptionPanelEvent;

// ── Plugin ───────────────────────────────────────────────────────────────────

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<DescriptionCloseHoverState>()
        .init_resource::<DescriptionPanelPauseState>()
        .add_systems(
            Update,
            (
                tick_description_panel_close_hover,
                sync_description_close_loading_bar,
            )
                .chain()
                .run_if(in_state(Pause(true))),
        )
        .add_observer(on_show_description_panel)
        .add_observer(on_hide_description_panel);
}

// ── Systems ───────────────────────────────────────────────────────────────────

fn tick_description_panel_close_hover(
    hands: Query<(&Transform, &Visibility), Or<(With<LeftHand>, With<RightHand>)>>,
    close_btn: Query<&GlobalTransform, With<DescriptionCloseButton>>,
    time: Res<Time>,
    mut state: ResMut<DescriptionCloseHoverState>,
    mut commands: Commands,
) {
    let Ok(btn_gt) = close_btn.single() else {
        return;
    };
    let btn_pos = btn_gt.translation().truncate();
    let half = DESCRIPTION_CLOSE_BTN_SIZE_PX / 2.0;

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
        if state.elapsed >= DESCRIPTION_CLOSE_HOVER_DURATION_SECS {
            state.active = false;
            state.elapsed = 0.0;
            commands.trigger(DespawnDescriptionPanelEvent);
        }
    } else {
        state.active = false;
        state.elapsed = 0.0;
    }
}

fn sync_description_close_loading_bar(
    state: Res<DescriptionCloseHoverState>,
    close_bar: Query<
        (&MeshMaterial2d<LoadingBarMaterial>, Entity),
        With<DescriptionCloseLoadingBar>,
    >,
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
            mat.params.x = (state.elapsed / DESCRIPTION_CLOSE_HOVER_DURATION_SECS).clamp(0.0, 1.0);
        }
    } else {
        vis.set_if_neq(Visibility::Hidden);
    }
}

// ── Observers ─────────────────────────────────────────────────────────────────

fn on_show_description_panel(
    trigger: On<SpawnDescriptionPanelEvent>,
    mut commands: Commands,
    existing_panel: Query<Entity, With<DescriptionPanel>>,
    existing_sub: Query<Entity, With<DescriptionSubPanel>>,
    existing_lb: Query<Entity, With<DescriptionCloseLoadingBar>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rc_materials: ResMut<Assets<HueShiftFishMaterial>>,
    mut lb_materials: ResMut<Assets<LoadingBarMaterial>>,
    asset_server: Res<AssetServer>,
    font_handle: Res<FontHandle>,
    i18n: Res<bevy_intl::I18n>,
    palette: Res<crate::theme::palette::ColorPalette>,
    pause_state: Res<State<Pause>>,
    mut panel_state: ResMut<DescriptionPanelPauseState>,
    mut next_pause: ResMut<NextState<Pause>>,
) {
    // Despawn any existing panel instances before spawning a fresh one.
    for e in existing_panel.iter() {
        commands.entity(e).despawn();
    }
    for e in existing_sub.iter() {
        commands.entity(e).despawn();
    }
    for e in existing_lb.iter() {
        commands.entity(e).despawn();
    }

    let event = trigger.event();
    let name = event.name.clone();
    let image_path = event.image_path.clone();
    let text = event.text.clone();
    let log = event.log.clone();
    let hue_shift = event.hue_shift;

    let backpack_open = pause_state.get().0;
    let panel_x = if backpack_open {
        DESCRIPTION_PANEL_WORLD_X
    } else {
        0.0
    };

    let close_x = DESCRIPTION_PANEL_W / 2.0
        - DESCRIPTION_CLOSE_BTN_SIZE_PX / 2.0
        - DESCRIPTION_CLOSE_BTN_MARGIN_PX;
    let close_y = DESCRIPTION_PANEL_H / 2.0
        - DESCRIPTION_CLOSE_BTN_SIZE_PX / 2.0
        - DESCRIPTION_CLOSE_BTN_MARGIN_PX;

    // Hide close button when backpack is open (its close button handles everything).
    let close_btn_vis = if backpack_open {
        Visibility::Hidden
    } else {
        Visibility::Inherited
    };

    let mut fish_mat = HueShiftFishMaterial::default();
    fish_mat.texture = asset_server.load(image_path.clone());
    fish_mat.hue_shift = hue_shift;

    let (img_w, img_h) = description_image_size_for_path(&image_path);

    let text_x = panel_x - DESCRIPTION_PANEL_W / 2.0 + DESCRIPTION_PANEL_PADDING_PX;

    // Panel background
    commands.spawn((
        Name::new("DescriptionPanel"),
        DescriptionPanel,
        Mesh2d(meshes.add(Rectangle::new(DESCRIPTION_PANEL_W, DESCRIPTION_PANEL_H))),
        MeshMaterial2d(materials.add(ColorMaterial {
            color: palette.panel_bg.with_alpha(0.96),
            ..default()
        })),
        Transform::from_xyz(panel_x, 0.0, 25.0),
        AnimSpawnOn,
    ));

    // Image background card
    commands.spawn((
        Name::new("ImageBg"),
        DescriptionSubPanel,
        Mesh2d(meshes.add(Rectangle::new(
            DESCRIPTION_IMAGE_BG_SIZE_PX,
            DESCRIPTION_IMAGE_BG_SIZE_PX,
        ))),
        MeshMaterial2d(materials.add(ColorMaterial {
            color: palette.panel_image_bg,
            ..default()
        })),
        Transform::from_xyz(panel_x, DESCRIPTION_IMAGE_Y, 26.0),
        AnimSpawnOn,
    ));

    // Item image
    commands.spawn((
        Name::new("ItemImage"),
        DescriptionSubPanel,
        Mesh2d(meshes.add(Rectangle::new(img_w, img_h))),
        MeshMaterial2d(rc_materials.add(fish_mat)),
        Transform::from_xyz(panel_x, DESCRIPTION_IMAGE_Y, 27.0),
        AnimSpawnOn,
    ));

    // Item name
    commands.spawn((
        Name::new("ItemName"),
        DescriptionSubPanel,
        Text2d::new(name),
        title_font(&font_handle, &i18n),
        TextColor(palette.panel_content_text),
        TextLayout::new_with_justify(Justify::Center),
        TextBounds::new_horizontal(DESCRIPTION_CONTENT_W),
        Anchor::TOP_CENTER,
        Transform::from_xyz(panel_x, DESCRIPTION_NAME_TOP_Y, 27.0),
        AnimSpawnOn,
    ));

    // Divider
    commands.spawn((
        Name::new("Divider"),
        DescriptionSubPanel,
        Mesh2d(meshes.add(Rectangle::new(DESCRIPTION_CONTENT_W, 2.0))),
        MeshMaterial2d(materials.add(ColorMaterial {
            color: palette.panel_divider.with_alpha(0.6),
            ..default()
        })),
        Transform::from_xyz(panel_x, DESCRIPTION_DIVIDER_Y, 27.0),
        AnimSpawnOn,
    ));

    // Log content (above description)
    commands.spawn((
        Name::new("LogText"),
        DescriptionSubPanel,
        Text2d::new(log),
        panel_font(&font_handle, &i18n),
        TextColor(palette.panel_content_text),
        TextLayout::new_with_justify(Justify::Left),
        TextBounds::new(DESCRIPTION_CONTENT_W, DESCRIPTION_LOG_H),
        Anchor::TOP_LEFT,
        Transform::from_xyz(text_x, DESCRIPTION_CONTENT_TOP_Y, 27.0),
        AnimSpawnOn,
    ));

    // Description content (below log)
    commands.spawn((
        Name::new("ContentText"),
        DescriptionSubPanel,
        Text2d::new(text),
        TextFont {
            font: font_handle.get(&i18n),
            font_size: FontSize::Px(26.0),
            ..default()
        },
        TextColor(palette.panel_dim_text.with_alpha(0.6)),
        TextLayout::new_with_justify(Justify::Left),
        TextBounds::new(DESCRIPTION_CONTENT_W, DESCRIPTION_DESC_H),
        Anchor::TOP_LEFT,
        Transform::from_xyz(text_x, DESCRIPTION_DESC_TOP_Y, 27.0),
        AnimSpawnOn,
    ));

    // Close button
    commands.spawn((
        Name::new("DescriptionCloseButton"),
        DescriptionSubPanel,
        DescriptionCloseButton,
        Mesh2d(meshes.add(Rectangle::new(
            DESCRIPTION_CLOSE_BTN_SIZE_PX,
            DESCRIPTION_CLOSE_BTN_SIZE_PX,
        ))),
        MeshMaterial2d(materials.add(ColorMaterial {
            texture: Some(asset_server.load("textures/close.png")),
            ..default()
        })),
        Transform::from_xyz(panel_x + close_x, close_y, 26.0),
        close_btn_vis,
        AnimSpawnOn,
    ));

    // Close loading bar
    commands.spawn((
        Name::new("DescriptionCloseLoadingBar"),
        DescriptionCloseLoadingBar,
        Mesh2d(meshes.add(Rectangle::new(
            DESCRIPTION_CLOSE_LOADING_BAR_SIZE_PX,
            DESCRIPTION_CLOSE_LOADING_BAR_SIZE_PX,
        ))),
        MeshMaterial2d(lb_materials.add(LoadingBarMaterial::default())),
        Transform::from_xyz(panel_x + close_x, close_y, 45.0),
        Visibility::Hidden,
    ));

    if !backpack_open {
        next_pause.set(Pause(true));
        panel_state.triggered_pause = true;
    }
}

fn on_hide_description_panel(
    _trigger: On<DespawnDescriptionPanelEvent>,
    mut commands: Commands,
    panel: Query<Entity, With<DescriptionPanel>>,
    sub_panels: Query<Entity, With<DescriptionSubPanel>>,
    close_bar: Query<Entity, With<DescriptionCloseLoadingBar>>,
    mut next_pause: ResMut<NextState<Pause>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut close_state: ResMut<DescriptionCloseHoverState>,
    mut panel_state: ResMut<DescriptionPanelPauseState>,
    backpack_is_open: Res<BackpackIsOpen>,
    mut panel_close_goes_to_end: ResMut<PanelCloseGoesToEnd>,
    mut panel_close_goes_to_dead: ResMut<PanelCloseGoesToDead>,
) {
    for e in &panel {
        commands.entity(e).insert(AnimDespawn);
    }
    for e in &sub_panels {
        commands.entity(e).insert(AnimDespawn);
    }
    for e in &close_bar {
        commands.entity(e).despawn();
    }

    if panel_state.triggered_pause {
        next_pause.set(Pause(false));
        panel_state.triggered_pause = false;
    }
    if !backpack_is_open.0 {
        if panel_close_goes_to_end.0 {
            panel_close_goes_to_end.0 = false;
            next_game_state.set(GameState::End);
        } else if panel_close_goes_to_dead.0 {
            panel_close_goes_to_dead.0 = false;
            next_game_state.set(GameState::Dead);
        } else {
            next_game_state.set(GameState::Idle);
        }
    }
    *close_state = DescriptionCloseHoverState::default();
}
