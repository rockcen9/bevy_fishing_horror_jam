pub(crate) mod detection;
pub use detection::{DetectionBox, PlayerDetections};
mod player;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    detection::plugin(app);
    player::plugin(app);
}
