mod hand_screen;
mod monster_threat;

use bevy::prelude::*;
use dev_debug_panel::{StateDebugPanel, StateDebugPanelPlugin};

use crate::prelude::*;
use crate::screens::loading::LoadingScreen;

pub(super) fn plugin(app: &mut bevy::app::App) {
    hand_screen::plugin(app);
    monster_threat::plugin(app);

    app.add_plugins(StateDebugPanelPlugin::<Screen>::all());
    app.add_plugins(StateDebugPanelPlugin::<LoadingScreen>::all());
    app.add_plugins(StateDebugPanelPlugin::<MonsterState>::all());
    app.add_plugins(StateDebugPanelPlugin::new([Pause(true), Pause(false)]));
    app.add_plugins(StateDebugPanelPlugin::<GameState>::all());

    app.add_systems(Update, toggle_visibility);
}

fn toggle_visibility(
    keys: Res<ButtonInput<KeyCode>>,
    mut panels: Query<&mut Visibility, With<StateDebugPanel>>,
) {
    if keys.pressed(KeyCode::SuperLeft) && keys.just_pressed(KeyCode::Digit5) {
        for mut vis in &mut panels {
            *vis = match *vis {
                Visibility::Hidden => Visibility::Visible,
                _ => Visibility::Hidden,
            };
        }
    }
}
