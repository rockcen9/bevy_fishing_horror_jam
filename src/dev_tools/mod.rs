//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{dev_tools::states::log_transitions, prelude::*};

mod debug_panel;
pub(crate) mod log_components;
mod validate_preloading;

// mod cheat_buttons;
// use cheat_buttons::*;

mod change_detection;
use change_detection::*;

mod bevy_inspector_egui;
// mod statistic;

// mod display_resource;
// use display_resource::*;

mod inspect;
pub use inspect::*;

mod pick;
// mod scenario_test;
use crate::{menus::Menu, screens::loading::LoadingScreen};
// pub use scenario_test::*;

pub(super) fn plugin(app: &mut App) {
    bevy_inspector_egui::plugin(app);
    change_detection_plugin(app);
    filter_component_plugin(app);
    pick::plugin(app);

    app.add_systems(
        Update,
        (log_transitions::<Menu>, log_transitions::<LoadingScreen>).chain(),
    );

    app.add_plugins((validate_preloading::plugin, log_components::plugin));
    debug_panel::plugin(app);
    app.add_systems(Update, _print_hover_on_click);
}
pub fn command_key_toggle_active(
    default: bool,
    key: KeyCode,
) -> impl FnMut(Res<ButtonInput<KeyCode>>) -> bool + Clone {
    let mut active = default;
    move |inputs: Res<ButtonInput<KeyCode>>| {
        if inputs.pressed(KeyCode::SuperLeft) && inputs.just_pressed(key) {
            active = !active;
        }
        active
    }
}

// #[cfg(feature = "dev")]
// const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

// #[cfg(feature = "dev")]
// fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
//     options.toggle();
// }
use bevy::picking::hover::HoverMap;

pub fn _print_hover_on_click(
    hover_map: Option<Res<HoverMap>>,
    names: Query<&Name>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if !mouse.just_pressed(MouseButton::Left) || !keys.pressed(KeyCode::SuperLeft) {
        return;
    }

    let Some(hover_map) = hover_map else {
        return;
    };

    println!("--- HoverMap State on Click ---");
    for (_pointer_id, pointer_map) in hover_map.iter() {
        for (entity, hit) in pointer_map.iter() {
            let name = names.get(*entity).map(|n| n.as_str()).unwrap_or("Unknown");
            println!("Entity hit by raycast: {:?} [{}]", entity, name);
            println!("   HitData: {:?}", hit);
        }
    }
    println!("--------------------------------------");
}
