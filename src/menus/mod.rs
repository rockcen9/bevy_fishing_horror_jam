use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.init_state::<Menu>();
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Reflect)]
#[states(scoped_entities)]
pub enum Menu {
    #[default]
    None,
    Main,
    Pause,
}
