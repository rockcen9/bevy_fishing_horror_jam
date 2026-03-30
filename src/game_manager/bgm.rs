// use crate::game_manager::monster::MonsterState;
use crate::prelude::*;
use kira_ext::BGMEvent;

pub(crate) fn plugin(app: &mut App) {
    // app.add_systems(OnEnter(GameState::Tutorial),   play_night_bgm)
    // .add_systems(OnEnter(GameState::ScanItems),  play_night_bgm)
    app.add_systems(OnEnter(GameState::Idle), play_night_bgm);
    app.add_systems(OnEnter(GameState::End), play_end);
    // .add_systems(OnEnter(GameState::Waiting),    play_night_bgm)
    // .add_systems(OnEnter(GameState::Biting),     play_battle)
    // .add_systems(OnEnter(GameState::Reeling),    play_battle)
    // .add_systems(OnEnter(GameState::Failing),    play_ghost)
    // .add_systems(OnEnter(GameState::Succeeding), play_prepare)
    // .add_systems(OnEnter(GameState::Resolution), play_ghost);
}

fn play_night_bgm(mut commands: Commands) {
    commands.trigger(BGMEvent::new("night"));
}

fn play_end(mut commands: Commands) {
    commands.trigger(BGMEvent::new("end"));
}
