pub const GAME_WIDTH: f32 = 1920.;
pub const GAME_HEIGHT: f32 = 1080.;

pub use ai_core::BackgroundImage;
pub use ai_core::MainCamera;

pub mod config;
pub use config::YoloConfig;

pub(crate) mod game_manager;
pub use game_manager::{DetectionBox, PlayerDetections};
pub(crate) mod prelude;
pub use prelude::*;

#[cfg(feature = "backend")]
pub(crate) mod model;

pub fn plugin(app: &mut bevy::prelude::App) {
    config::plugin(app);
    ai_core::plugin(app);
    game_manager::plugin(app);
}
