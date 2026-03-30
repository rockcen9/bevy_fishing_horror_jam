#![allow(unused_imports)]
use std::fmt::Debug;

// use crate::{
//     dev_tools::inspect::command_key_toggle_active,
//     prelude::{
//         inventory_grid::InventoryGrid,
//         quirks::{
//             DebugInflictOnhitFriendlyFireQuirk, DebugOnhitIgniteQuirk, FactoryQuirks, ItemQuirks,
//             UnitQuirks,
//         },
//         *,
//     },
// };
use bevy::sprite::Anchor;
// use bevy_hanabi::EffectSpawner;
use crate::dev_tools::command_key_toggle_active;
use crate::prelude::*;
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;
#[allow(unused_variables)]
pub fn debug_component(app: &mut App) {
    let default_toggle = false;

    app.add_plugins(
        FilterQueryInspectorPlugin::<With<DescriptionPanel>>::default()
            .run_if(command_key_toggle_active(default_toggle, KeyCode::Digit4)),
    );

    app.add_plugins(
        FilterQueryInspectorPlugin::<With<Container>>::default()
            .run_if(command_key_toggle_active(default_toggle, KeyCode::Digit4)),
    );
    // app.add_plugins(
    //     FilterQueryInspectorPlugin::<(With<Anchor>, With<InventoryGrid>)>::default()
    //         .run_if(command_key_toggle_active(default_toggle, KeyCode::Digit4)),
    // );
    // app.add_plugins(
    //     FilterQueryInspectorPlugin::<With<Item>>::default()
    //         .run_if(command_key_toggle_active(default_toggle, KeyCode::Digit4)),
    // );
    // app.add_plugins(
    //     FilterQueryInspectorPlugin::<(With<DebugOnhitIgniteQuirk>, With<EnemyFaction>)>::default(
    //     )
    //     .run_if(command_key_toggle_active(default_toggle, KeyCode::Digit4)),
    // );
    // app.add_plugins(
    //     FilterQueryInspectorPlugin::<(
    //         With<SpearHall>,
    //         With<InCityInventory>,
    //         With<MillitaryBuilding>,
    //         // With<SpawnsUnit>,
    //     )>::default()
    //     .run_if(command_key_toggle_active(default_toggle, KeyCode::Digit3)),
    // );
    // // PathFollowing component removed - landmass handles pathfinding internally
    // app.add_plugins(
    //     FilterQueryInspectorPlugin::<With<Pickable>>::default()
    //         .run_if(command_key_toggle_active(default_toggle, KeyCode::Digit3)),
    // );
    // app.add_plugins(
    //     FilterQueryInspectorPlugin::<With<NavigateToLocation>>::default()
    //         .run_if(command_key_toggle_active(default_toggle, KeyCode::Digit3)),
    // );
    // app.add_plugins(
    //     FilterQueryInspectorPlugin::<With<NavigateToLocation>>::default()
    //         .run_if(command_key_toggle_active(default_toggle, KeyCode::Digit3)),
    // );
    // app.add_plugins(
    //     FilterQueryInspectorPlugin::<With<NavigateToLocation>>::default()
    //         .run_if(command_key_toggle_active(default_toggle, KeyCode::Digit3)),
    // );
    // app.add_plugins(
    //     FilterQueryInspectorPlugin::<With<Interactable>>::default()
    //         .run_if(command_key_toggle_active(default_toggle, KeyCode::Digit3)),
    // );
    // app.add_plugins(
    //     FilterQueryInspectorPlugin::<With<TrashCan>>::default()
    //         .run_if(command_key_toggle_active(default_toggle, KeyCode::Digit3)),
    // );
    // app.add_plugins(
    //     FilterQueryInspectorPlugin::<(Without<AwaitingFood>, With<Roaming>)>::default()
    //         .run_if(command_key_toggle_active(default_toggle, KeyCode::Digit3)),
    // );

    // app.add_plugins(
    //     FilterQueryInspectorPlugin::<With<Plate>>::default()
    //         .run_if(command_key_toggle_active(default_toggle, KeyCode::Digit3)),
    // );
    // app.add_plugins(
    //     FilterQueryInspectorPlugin::<With<Salad>>::default()
    //         .run_if(command_key_toggle_active(default_toggle, KeyCode::Digit3)),
    // );

    // app.add_plugins(
    //     FilterQueryInspectorPlugin::<(Without<AwaitingFood>, With<Roaming>, Without<PieceType>)>::default()
    //         .run_if(command_key_toggle_active(default_toggle, KeyCode::Digit2)),
    // );
    // app.add_plugins(
    //     FilterQueryInspectorPlugin::<With<HealthPotion>>::default()
    //         .run_if(command_key_toggle_active(default_toggle, KeyCode::Digit1)),
    // );
    // app.add_plugins(FilterQueryInspectorPlugin::<With<Item>>::default());
    // app.add_plugins(FilterQueryInspectorPlugin::<(With<Enemy>, With<CardSelect>)>::default());
    // app.add_plugins(FilterQueryInspectorPlugin::<With<CardSelect>>::default());
}
