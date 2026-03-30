//! The game's main screen states and transitions between them.

mod gameplay;
pub(crate) mod loading;
mod splash;
mod start_game_ui;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        start_game_ui::plugin,
        title::plugin,
    ));
}

/// The game's main screen states.
#[derive(
    States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect, strum_macros::EnumIter,
)]
#[states(scoped_entities)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Loading,
    Gameplay,
}
