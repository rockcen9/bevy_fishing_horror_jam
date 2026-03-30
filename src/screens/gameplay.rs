//! The screen state for the main gameplay.

use bevy::{input::common_conditions::input_just_pressed, prelude::*, ui::Val::*};
// #[cfg(feature = "web")]
// use bevy_fix_cursor_unlock_web::ForceUnlockCursor;

use camera_effects::fade::FadeOutEvent;

use crate::{Pause, menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), fade_out_on_enter);
    // Toggle pause on key press.
    app.add_systems(
        Update,
        (
            // (set_game_paused, spawn_pause_overlay, set_pause_menu_open).run_if(
            //     in_state(Screen::Gameplay)
            //         .and_then(in_state(Menu::None))
            //         .and_then(input_just_pressed(KeyCode::KeyP).or_else(input_just_pressed(KeyCode::Escape))),
            // ),
            set_menu_closed.run_if(
                in_state(Screen::Gameplay)
                    .and_then(not(in_state(Menu::None)))
                    .and_then(input_just_pressed(KeyCode::KeyP)),
            ),
        ),
    );
    app.add_systems(
        OnExit(Screen::Gameplay),
        (set_menu_closed, set_game_unpaused),
    );
    app.add_systems(
        OnEnter(Menu::None),
        set_game_unpaused.run_if(in_state(Screen::Gameplay)),
    );
    // #[cfg(feature = "web")]
    // app.add_observer(open_pause_menu_on_cursor_force_unlock);
}

fn fade_out_on_enter(mut commands: Commands) {
    commands.trigger(FadeOutEvent::default());
}

fn set_game_unpaused(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(false));
}

// #[cfg(feature = "web")]
// fn open_pause_menu_on_cursor_force_unlock(
//     _unlock: On<ForceUnlockCursor>,
//     mut next_menu: ResMut<NextState<Menu>>,
// ) {
//     next_menu.set(Menu::Pause);
// }

fn set_menu_closed(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
