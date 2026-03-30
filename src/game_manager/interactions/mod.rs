use crate::prelude::*;

mod spawn_cup;

pub(crate) fn plugin(app: &mut App) {
    spawn_cup::plugin(app);
    #[cfg(feature = "backend")]
    app.add_systems(OnEnter(GameState::Tutorial), on_enter_idle_enable_yolo)
        .add_systems(OnEnter(GameState::Idle), on_enter_idle_enable_yolo);
}

#[cfg(feature = "backend")]
fn on_enter_idle_enable_yolo(mut config: ResMut<yolo::YoloConfig>) {
    config.run_inference = true;
}
