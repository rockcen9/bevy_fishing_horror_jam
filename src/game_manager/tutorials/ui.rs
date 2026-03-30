use crate::loading_bar::{LOADING_BAR_RING_SIZE_PX, LoadingBarMaterial};
use crate::prelude::*;
use bevy_intl::I18n;
use vleue_kinetoscope::AnimatedImageController;

// ── Z Layers ────────────────────────────────────────────────────────────────
const OVERLAY_Z: f32 = 33.0;
const PANEL_Z: f32 = 33.5;
const GIF_FRAME_Z: f32 = 33.8;
const DIVIDER_Z: f32 = 33.9;
const GIF_Z: f32 = 34.0;
const TEXT_Z: f32 = 35.0;
const CLOSE_BTN_Z: f32 = 36.0;
const CLOSE_LB_Z: f32 = 45.0; // SpriteLayer::LoadingBar — above hands

// ── Panel ───────────────────────────────────────────────────────────────────
const PANEL_W: f32 = 1720.0;
const PANEL_H: f32 = 920.0;

// ── GIFs ────────────────────────────────────────────────────────────────────
const GIF_SIZE: Vec2 = Vec2::new(260.0, 260.0);
const GIF_FRAME_SIZE: f32 = 290.0;
const GIF_Y: f32 = 40.0;
const LABEL_Y: f32 = -175.0;
const GIF_POSITIONS: [f32; 4] = [-585.0, -195.0, 195.0, 585.0];
const GIF_LABELS: [&str; 4] = [
    "Cast the line",
    "Pull up the line",
    "Open Backpack",
    "Close Backpack",
];
const GIF_PATHS: [&str; 4] = [
    "tutorials/forward.gif",
    "tutorials/pull.gif",
    "tutorials/open.gif",
    "tutorials/close.gif",
];

// ── Layout ──────────────────────────────────────────────────────────────────
const TITLE_Y: f32 = 390.0;
const DIVIDER_Y: f32 = 320.0;
const CLOSE_BTN_POS: Vec2 = Vec2::new(PANEL_W / 2.0 - 60.0, PANEL_H / 2.0 - 60.0);
const CLOSE_BTN_SIZE: f32 = 80.0;
const CLOSE_HOVER_DURATION_SECS: f32 = 1.0;

// ── Colors ───────────────────────────────────────────────────────────────────
fn color_overlay() -> Color {
    Color::srgba(0.125, 0.066, 0.078, 0.92) // dark_plum #201114 at 92%
}
fn color_panel() -> Color {
    Color::srgb(0.125, 0.066, 0.078) // dark_plum #201114
}
fn color_gif_frame() -> Color {
    Color::srgb(0.188, 0.090, 0.078) // deep_maroon #301714
}
fn color_divider() -> Color {
    Color::srgba(0.365, 0.337, 0.337, 0.6) // grime #5D5656 at 60%
}
fn color_ivory() -> Color {
    Color::srgb(0.937, 0.929, 0.914) // ivory #EFEDE9
}
fn color_bone() -> Color {
    Color::srgb(0.871, 0.839, 0.765) // bone #DED6C3
}

// ── Components / Resources ───────────────────────────────────────────────────
#[derive(Component)]
struct TutorialCloseButton;

#[derive(Component)]
struct TutorialCloseLoadingBar;

#[derive(Resource, Default)]
struct CloseHoverState {
    elapsed: f32,
    active: bool,
}

// ── Plugin ────────────────────────────────────────────────────────────────────
pub(super) fn plugin(app: &mut bevy::app::App) {
    app.init_resource::<CloseHoverState>();
    app.add_systems(OnEnter(GameState::Tutorial), spawn_ui);
    app.add_systems(
        Update,
        (tick_close_hover, sync_close_loading_bar)
            .chain()
            .run_if(in_state(GameState::Tutorial)),
    );
}

// ── Spawn ─────────────────────────────────────────────────────────────────────
fn spawn_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    font_handle: Res<FontHandle>,
    i18n: Res<I18n>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut lb_materials: ResMut<Assets<LoadingBarMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let font = font_handle.get(&i18n);

    // Full-screen dim overlay
    commands.spawn((
        SpriteLayer::Tutorial,
        Sprite {
            color: color_overlay(),
            custom_size: Some(Vec2::new(1920.0, 1080.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, OVERLAY_Z),
        AnimDuring(GameState::Tutorial),
    ));

    // Dark plum panel background
    commands.spawn((
        SpriteLayer::Tutorial,
        Mesh2d(meshes.add(Rectangle::new(PANEL_W, PANEL_H))),
        MeshMaterial2d(color_materials.add(ColorMaterial::from_color(color_panel()))),
        Transform::from_xyz(0.0, 0.0, PANEL_Z),
        AnimDuring(GameState::Tutorial),
    ));

    // Title
    commands.spawn((
        Text2d::new("Tutorial"),
        TextFont {
            font: font.clone(),
            font_size: FontSize::Px(64.0),
            ..default()
        },
        TextColor(color_ivory()),
        Transform::from_xyz(0.0, TITLE_Y, TEXT_Z),
        AnimDuring(GameState::Tutorial),
    ));

    // Horizontal divider beneath the title
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(PANEL_W - 80.0, 2.0))),
        MeshMaterial2d(color_materials.add(ColorMaterial::from_color(color_divider()))),
        Transform::from_xyz(0.0, DIVIDER_Y, DIVIDER_Z),
        AnimDuring(GameState::Tutorial),
    ));

    // GIF columns
    for i in 0..4 {
        let x = GIF_POSITIONS[i];

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(GIF_FRAME_SIZE, GIF_FRAME_SIZE))),
            MeshMaterial2d(color_materials.add(ColorMaterial::from_color(color_gif_frame()))),
            Transform::from_xyz(x, GIF_Y, GIF_FRAME_Z),
            AnimDuring(GameState::Tutorial),
        ));

        commands.spawn((
            AnimatedImageController::play(asset_server.load(GIF_PATHS[i])),
            Sprite {
                custom_size: Some(GIF_SIZE),
                ..default()
            },
            Transform::from_xyz(x, GIF_Y, GIF_Z),
            AnimDuring(GameState::Tutorial),
        ));

        commands.spawn((
            Text2d::new(GIF_LABELS[i]),
            TextFont {
                font: font.clone(),
                font_size: FontSize::Px(32.0),
                ..default()
            },
            TextColor(color_bone()),
            Transform::from_xyz(x, LABEL_Y, TEXT_Z),
            AnimDuring(GameState::Tutorial),
        ));
    }

    // Close button
    commands.spawn((
        TutorialCloseButton,
        Sprite {
            image: asset_server.load("textures/close.png"),
            custom_size: Some(Vec2::splat(CLOSE_BTN_SIZE)),
            ..default()
        },
        Transform::from_xyz(CLOSE_BTN_POS.x, CLOSE_BTN_POS.y, CLOSE_BTN_Z),
        AnimDuring(GameState::Tutorial),
    ));

    // Loading bar ring for close hover
    commands.spawn((
        TutorialCloseLoadingBar,
        SpriteLayer::LoadingBar,
        Mesh2d(meshes.add(Rectangle::new(
            LOADING_BAR_RING_SIZE_PX,
            LOADING_BAR_RING_SIZE_PX,
        ))),
        MeshMaterial2d(lb_materials.add(LoadingBarMaterial::default())),
        Transform::from_xyz(CLOSE_BTN_POS.x, CLOSE_BTN_POS.y, CLOSE_LB_Z),
        Visibility::Hidden,
        AnimDuring(GameState::Tutorial),
    ));
}

// ── Systems ───────────────────────────────────────────────────────────────────
fn tick_close_hover(
    hands: Query<(&Transform, &Visibility), Or<(With<LeftHand>, With<RightHand>)>>,
    time: Res<Time>,
    mut state: ResMut<CloseHoverState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let half = CLOSE_BTN_SIZE / 2.0;

    let is_hovering = hands.iter().any(|(transform, visibility)| {
        if *visibility == Visibility::Hidden {
            return false;
        }
        let hand_pos = transform.translation.truncate();
        (hand_pos.x - CLOSE_BTN_POS.x).abs() <= half && (hand_pos.y - CLOSE_BTN_POS.y).abs() <= half
    });

    if is_hovering {
        if !state.active {
            state.active = true;
            state.elapsed = 0.0;
        }
        state.elapsed += time.delta_secs();
        if state.elapsed >= CLOSE_HOVER_DURATION_SECS {
            *state = CloseHoverState::default();
            next_state.set(GameState::Idle);
        }
    } else {
        state.active = false;
        state.elapsed = 0.0;
    }
}

fn sync_close_loading_bar(
    state: Res<CloseHoverState>,
    loading_bar: Query<
        (&MeshMaterial2d<LoadingBarMaterial>, Entity),
        With<TutorialCloseLoadingBar>,
    >,
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
            mat.params.x = (state.elapsed / CLOSE_HOVER_DURATION_SECS).clamp(0.0, 1.0);
        }
    } else {
        vis.set_if_neq(Visibility::Hidden);
    }
}
