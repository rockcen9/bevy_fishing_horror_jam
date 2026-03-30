//! The title screen that appears after the splash screen.

use bevy::prelude::*;

use crate::{menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), set_main_menu_open);
    app.add_systems(OnExit(Screen::Title), set_menu_closed);
}

fn set_main_menu_open(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn set_menu_closed(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
