// use anim_ui::DespawnOnExit;

use crate::loading_bar::{LOADING_BAR_RING_SIZE_PX, LoadingBarMaterial};
use crate::prelude::*;
use kira_ext::SFXEvent;

// Top-left corner: 1920×1080 world, origin at center → top-left is (-960, 540)
const BTN_SIZE: f32 = 80.0;
const BTN_MARGIN: f32 = 50.0;
const BTN_X: f32 = -960.0 + BTN_MARGIN + BTN_SIZE / 2.0;
const BTN_Y: f32 = 540.0 - BTN_MARGIN - BTN_SIZE / 2.0;
const BTN_Z: f32 = 35.0;
const BTN_LB_Z: f32 = 45.0; // matches SpriteLayer::LoadingBar
const HOVER_DURATION_SECS: f32 = 1.0;

// ── Markers / Resources ──────────────────────────────────────────────────────

#[derive(Component)]
struct TutorialButton;

#[derive(Component)]
struct TutorialButtonLoadingBar;

#[derive(Resource, Default)]
struct TutorialButtonHoverState {
    elapsed: f32,
    active: bool,
}

// ── Plugin ────────────────────────────────────────────────────────────────────

pub(super) fn plugin(app: &mut bevy::app::App) {
    app.init_resource::<TutorialButtonHoverState>();
    app.add_systems(OnEnter(Screen::Gameplay), spawn_tutorial_button);
    app.add_systems(
        Update,
        (tick_tutorial_button_hover, sync_tutorial_button_loading_bar)
            .chain()
            .in_set(PausableSystems),
    );
}

// ── Spawn ─────────────────────────────────────────────────────────────────────

fn spawn_tutorial_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut lb_materials: ResMut<Assets<LoadingBarMaterial>>,
) {
    commands.spawn((
        Name::new("TutorialButton"),
        TutorialButton,
        Sprite {
            image: asset_server.load("textures/tutorial.png"),
            custom_size: Some(Vec2::splat(BTN_SIZE)),
            ..default()
        },
        Transform::from_xyz(BTN_X, BTN_Y, BTN_Z),
        DespawnOnExit(Screen::Gameplay),
    ));

    commands.spawn((
        Name::new("TutorialButtonLoadingBar"),
        TutorialButtonLoadingBar,
        SpriteLayer::LoadingBar,
        Mesh2d(meshes.add(Rectangle::new(
            LOADING_BAR_RING_SIZE_PX,
            LOADING_BAR_RING_SIZE_PX,
        ))),
        MeshMaterial2d(lb_materials.add(LoadingBarMaterial::default())),
        Transform::from_xyz(BTN_X, BTN_Y, BTN_LB_Z),
        Visibility::Hidden,
        DespawnOnExit(Screen::Gameplay),
    ));
}

// ── Systems ───────────────────────────────────────────────────────────────────

fn tick_tutorial_button_hover(
    hands: Query<(&Transform, &Visibility), Or<(With<LeftHand>, With<RightHand>)>>,
    game_state: Option<Res<State<GameState>>>,
    time: Res<Time>,
    mut state: ResMut<TutorialButtonHoverState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    // Disable while Tutorial is already open
    if game_state.is_some_and(|s| *s == GameState::Tutorial) {
        state.active = false;
        state.elapsed = 0.0;
        return;
    }

    let half = BTN_SIZE / 2.0;

    let is_hovering = hands.iter().any(|(transform, visibility)| {
        if *visibility == Visibility::Hidden {
            return false;
        }
        let hand_pos = transform.translation.truncate();
        (hand_pos.x - BTN_X).abs() <= half && (hand_pos.y - BTN_Y).abs() <= half
    });

    if is_hovering {
        if !state.active {
            state.active = true;
            state.elapsed = 0.0;
        }
        state.elapsed += time.delta_secs();
        if state.elapsed >= HOVER_DURATION_SECS {
            *state = TutorialButtonHoverState::default();
            // commands.trigger(SFXEvent::ui("pick"));
            next_state.set(GameState::Tutorial);
        }
    } else {
        state.active = false;
        state.elapsed = 0.0;
    }
}

fn sync_tutorial_button_loading_bar(
    state: Res<TutorialButtonHoverState>,
    loading_bar: Query<
        (&MeshMaterial2d<LoadingBarMaterial>, Entity),
        With<TutorialButtonLoadingBar>,
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
            mat.params.x = (state.elapsed / HOVER_DURATION_SECS).clamp(0.0, 1.0);
        }
    } else {
        vis.set_if_neq(Visibility::Hidden);
    }
}
