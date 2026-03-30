use bevy::dev_tools::picking_debug::{DebugPickingMode, DebugPickingPlugin};

use crate::prelude::*;
pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_plugins(DebugPickingPlugin);
    app.insert_resource(DebugPickingMode::Disabled)
        // A system that cycles the debugging state when you press F3:
        .add_systems(
            PreUpdate,
            (|mut mode: ResMut<DebugPickingMode>| {
                *mode = match *mode {
                    DebugPickingMode::Disabled => DebugPickingMode::Normal,
                    DebugPickingMode::Normal => DebugPickingMode::Noisy,
                    DebugPickingMode::Noisy => DebugPickingMode::Disabled,
                }
            })
            .distributive_run_if(bevy::input::common_conditions::input_just_pressed(
                KeyCode::F3,
            )),
        );
}
