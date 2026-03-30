use bevy::prelude::*;

/// Runtime configuration for the yolo crate.
#[derive(Resource, Reflect)]
pub struct YoloConfig {
    /// Run YOLO pose inference each frame.
    pub run_inference: bool,
    /// Draw gizmo boxes around detected palms and faces.
    pub draw_detections: bool,
}

impl Default for YoloConfig {
    fn default() -> Self {
        Self {
            run_inference: false,
            draw_detections: false,
        }
    }
}

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<YoloConfig>();
}
