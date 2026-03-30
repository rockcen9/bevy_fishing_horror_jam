use bevy::asset::AssetMetaCheck;
use bevy_framepace::{FramepaceSettings, Limiter};
use tracing_subscriber::field::MakeExt;
use vleue_kinetoscope::AnimatedImagePlugin;

pub const GAME_WIDTH: f32 = 1920.;
pub const GAME_HEIGHT: f32 = 1080.;

mod game_manager;
pub mod loading_bar;
pub mod menus;
mod prelude;
use crate::prelude::*;
pub use prelude::*;
mod asset_loading;
mod asset_tracking;
mod dbg;
#[cfg(feature = "dev")]
mod dev_tools;
mod screens;

mod third_party;

mod theme;
mod version;
pub const GAME_VERSION: &str = env!("CARGO_PKG_VERSION");
fn main() -> AppExit {
    #[cfg(feature = "dev")]
    if let Ok(_) = dotenvy::dotenv() {
        println!("Feature dev_mode enabled, .env loaded.");
    }
    let mut app = App::new();
    let default_plugins = DefaultPlugins
        .set(bevy::log::LogPlugin {
            level: bevy::log::Level::INFO,
            filter: format!(
                concat!(
                    "{default},",
                    "calloop::loop_logic=error,",
                    "bevy_game::game_manager::shop::spawn=info,",
                    "yolo::game_manager::detection=warn,",
                    "bevy_game::game_manager::pole::idle::idle_to_backward=warn,",
                    "bevy_game::game_manager::player::hand_screen=warn,",
                    "bevy_game::game_manager::pole::biting::move_bobber=warn,",
                    "bevy_game::game_manager::pole::waiting::hooked_fish=warn,",
                    "bevy_game::game_manager::monster::check_escape=warn,",
                    "bevy_game::game_manager::backpack::hand_pointer=warn,",
                    "bevy_game::game_manager::ui::description_panel=debug,",
                    "bevy_game::game_manager::backpack::container=debug,",
                ),
                default = bevy::log::DEFAULT_FILTER
            ),
            fmt_layer: |_| {
                Some(Box::new(
                    bevy::log::tracing_subscriber::fmt::Layer::default()
                        .without_time()
                        .map_fmt_fields(MakeExt::debug_alt)
                        .with_writer(std::io::stderr),
                ))
            },
            ..default()
        })
        .set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        })
        .set(WindowPlugin {
            primary_window: Window {
                visible: false,
                title: "Symbiotic Hook".to_string(),
                fit_canvas_to_parent: true,
                resolution: bevy::window::WindowResolution::new(
                    GAME_WIDTH as u32,
                    GAME_HEIGHT as u32,
                ),
                ..default()
            }
            .into(),
            ..default()
        });
    #[cfg(feature = "dev")]
    let default_plugins =
        default_plugins.disable::<bevy::dev_tools::render_debug::RenderDebugOverlayPlugin>();
    app.add_plugins(default_plugins);

    app.add_systems(Update, show_window_after_warmup);

    app.add_plugins(AnimatedImagePlugin);
    app.add_plugins(bevy_framepace::FramepacePlugin);
    app.insert_resource(FramepaceSettings {
        limiter: Limiter::from_framerate(60.0),
    });

    asset_loading::plugin(&mut app);
    third_party::plugin(&mut app);
    loading_bar::plugin(&mut app);
    theme::plugin(&mut app);
    game_manager::plugin(&mut app);
    menus::plugin(&mut app);
    yolo::plugin(&mut app);

    // app.add_systems(Startup, |mut command: Commands| {
    //     command.trigger(BGMEvent::new("prepare"))
    // });

    app.add_plugins(bevy_tweening::TweeningPlugin);
    camera_effects::plugin(&mut app);
    app.add_plugins(camera_effects::FadePlugin {
        exit_state: screens::Screen::Gameplay,
    });
    kira_ext::plugin(&mut app);

    #[cfg(feature = "dev")]
    dev_tools::plugin(&mut app);
    asset_tracking::plugin(&mut app);
    screens::plugin(&mut app);
    dbg::plugin(&mut app);
    version::plugin(&mut app);
    app.add_plugins(
        anim_ui::AnimUiPlugin::new()
            .with_state::<Screen>()
            .with_state::<GameState>(),
    );

    app.run()
}

fn show_window_after_warmup(mut window: Query<&mut Window>, mut warmup_frame_count: Local<u32>) {
    *warmup_frame_count += 1;
    if *warmup_frame_count == 5 {
        if let Ok(mut window) = window.single_mut() {
            window.visible = true;
        }
    }
}
