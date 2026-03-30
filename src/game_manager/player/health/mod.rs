use crate::game_manager::backpack::{FishCaughtEvent, PrefabList, RestartGameEvent};
use crate::prelude::*;

mod heart_ui;
pub mod mpeg_artifact;
mod spawn;

#[derive(Resource, Reflect)]
pub struct PlayerHealth {
    pub value: f32,
}

impl Default for PlayerHealth {
    fn default() -> Self {
        Self { value: 100.0 }
    }
}

impl PlayerHealth {
    pub fn min() -> f32 {
        0.0
    }

    pub fn max() -> f32 {
        100.0
    }

    pub fn set(&mut self, value: f32) {
        self.value = value.clamp(Self::min(), Self::max());
    }
}

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<PlayerHealth>();
    heart_ui::plugin(app);
    spawn::plugin(app);
    mpeg_artifact::plugin(app);
    app.add_systems(Update, detect_player_death.in_set(PausableSystems))
        .add_observer(on_restart_game);
}

fn on_restart_game(_trigger: On<RestartGameEvent>, mut health: ResMut<PlayerHealth>) {
    health.value = PlayerHealth::max();
}

fn detect_player_death(
    health: Res<PlayerHealth>,
    mut prev_health: Local<f32>,
    mut commands: Commands,
) {
    let current = health.value;
    let previous = *prev_health;
    *prev_health = current;

    if previous > 0.0 && current <= 0.0 {
        commands.trigger(FishCaughtEvent {
            prefab_id: PrefabList::Kai.prefab_id().0,
        });
    }
}
