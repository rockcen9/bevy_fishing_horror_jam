use bevy::prelude::*;

pub mod background_image;
pub use background_image::BackgroundImage;

pub mod burn_plugin;
#[cfg(feature = "backend")]
pub use burn_plugin::backend::{BurnDevice, BurnPlugin};

pub mod camera;
pub use camera::MainCamera;

/// Bevy plugin that registers the camera and, when the `backend` feature is
/// active, the Burn device and real camera capture pipeline.
///
/// Safe to depend on from multiple crates — `plugin()` checks `is_plugin_added`
/// so the infra is initialized exactly once.
pub struct AiCorePlugin;

impl Plugin for AiCorePlugin {
    fn build(&self, app: &mut App) {
        camera::plugin(app);

        #[cfg(feature = "backend")]
        {
            app.add_plugins(BurnPlugin);
            camera::backend::plugin(app);
        }

        // Without the backend the real camera pipeline never runs, so no one
        // inserts BackgroundImage.  Initialize it with its Default so that
        // systems like `capture::on_capture_action` can still access it safely
        // (they will simply see an empty image and do nothing useful).
        #[cfg(not(feature = "backend"))]
        app.init_resource::<BackgroundImage>();
    }
}

pub fn plugin(app: &mut App) {
    if !app.is_plugin_added::<AiCorePlugin>() {
        app.add_plugins(AiCorePlugin);
    }
}
