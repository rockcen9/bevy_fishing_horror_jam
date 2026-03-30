use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::prelude::*;
pub fn plugin(app: &mut App) {
    if !app.is_plugin_added::<EguiPlugin>() {
        app.add_plugins(EguiPlugin::default());
    }
    app.add_plugins(
        WorldInspectorPlugin::default()
            .run_if(super::command_key_toggle_active(false, KeyCode::Digit1)),
    );
}
