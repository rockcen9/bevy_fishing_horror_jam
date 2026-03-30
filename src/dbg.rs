use crate::{prelude::*, screens::Screen};

// pub const DEBUG_PHYSICS_VISUAL: bool = false;

pub const _ACTIVE_SCENARIO: bool = false;
#[allow(dead_code)]
#[allow(dead_code)]
pub const DEBUG_ENABLE_TELEMETRY: bool = true;

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(DebugConfig::default());
    app.add_systems(Startup, print_debug_config);
    app.add_systems(OnEnter(Screen::Splash), skip_splash_if_debug);
    app.add_systems(OnEnter(Screen::Gameplay), skip_to_game_state_if_debug);
}

fn print_debug_config(config: Res<DebugConfig>) {
    info!("[DebugConfig] {config:?}");
}

fn skip_splash_if_debug(config: Res<DebugConfig>, mut next_screen: ResMut<NextState<Screen>>) {
    if config.skip_splash_assets_loading {
        next_screen.set(Screen::Gameplay);
    }
}

fn skip_to_game_state_if_debug(
    config: Res<DebugConfig>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    #[cfg(feature = "dev")]
    if let Some(state) = &config.skip_to {
        next_game_state.set(state.clone());
    }
}
/// Global debug configuration resource for controlling various debug features
#[allow(dead_code)]
#[derive(Resource, Debug)]
pub struct DebugConfig {
    pub skip_splash_assets_loading: bool,
    pub skip_to: Option<GameState>,
    pub side_car_windows: bool,
    pub make_window_bigger: bool,
    pub check_inventory_grid: bool,

    pub detective_objects_inference: bool,
    pub debug_shop: bool,
}

impl Default for DebugConfig {
    fn default() -> Self {
        #[cfg(feature = "dev")]
        {
            Self {
                skip_splash_assets_loading: true,
                skip_to: Some(GameState::Idle),
                side_car_windows: false,
                make_window_bigger: false,
                check_inventory_grid: false,

                detective_objects_inference: std::env::var("DETECTIVE_OBJECTS_INFERENCE").is_ok(),

                debug_shop: false,
            }
        }
        #[cfg(not(feature = "dev"))]
        {
            Self {
                skip_splash_assets_loading: false,
                skip_to: None,
                side_car_windows: false,
                make_window_bigger: false,
                check_inventory_grid: false,

                detective_objects_inference: false,
                debug_shop: false,
            }
        }
    }
}
