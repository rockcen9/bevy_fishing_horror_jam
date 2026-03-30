use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::{dev_tools::inspect::command_key_toggle_active, prelude::*};

pub fn debug_resource_plugin(app: &mut App) {
    app.add_plugins(
        bevy_inspector_egui::quick::StateInspectorPlugin::<Screen>::default()
            .run_if(super::command_key_toggle_active(false, KeyCode::Digit2)),
    );
    app.add_plugins(
        bevy_inspector_egui::quick::StateInspectorPlugin::<GameState>::default()
            .run_if(super::command_key_toggle_active(false, KeyCode::Digit2)),
    );

    // resource

    app.add_plugins(
        ResourceInspectorPlugin::<RightHandScreenPosition>::default()
            .run_if(super::command_key_toggle_active(false, KeyCode::Digit3)),
    );
    app.add_plugins(
        ResourceInspectorPlugin::<FishCatchSequence>::default()
            .run_if(command_key_toggle_active(false, KeyCode::Digit2)),
    );
    // app.add_plugins(
    //     ResourceInspectorPlugin::<CustomerAmountState>::default()
    //         .run_if(command_key_toggle_active(false, KeyCode::Digit2)),
    // );
    // app.add_plugins(
    //     ResourceInspectorPlugin::<TotalFoodDelivered>::default()
    //         .run_if(command_key_toggle_active(false, KeyCode::Digit2)),
    // );
    // app.add_plugins(
    //     ResourceInspectorPlugin::<PlayerCoins>::default()
    //         .run_if(command_key_toggle_active(false, KeyCode::Digit2)),
    // );
}
