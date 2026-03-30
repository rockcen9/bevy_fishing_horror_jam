mod player;
use crate::prelude::*;
pub(crate) use player::*;

mod scan_items;

mod interactions;

mod baits;

mod pole;
pub(crate) use pole::MonsterThreat;
pub use pole::*;

mod bgm;

mod background;
mod lake;
pub use lake::{LakePoint1, LakePoint2, LakePoint3};

mod sprite_layer;
pub use sprite_layer::*;

// mod actor;
// pub use actor::*;

mod backpack;
pub(crate) use backpack::*;

mod balance;
pub(crate) use balance::*;

mod ui;
pub(crate) use ui::*;

mod post_hook;

mod end;

mod dead;

mod goal;

mod tutorials;

mod monster;
pub use monster::MonsterState;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_sub_state::<GameState>();
    app.init_state::<Pause>();
    app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));
    player::plugin(app);
    scan_items::plugin(app);
    interactions::plugin(app);
    pole::plugin(app);
    baits::plugin(app);
    bgm::plugin(app);
    background::plugin(app);
    lake::plugin(app);
    // actor::plugin(app);
    backpack::plugin(app);
    balance::plugin(app);
    ui::plugin(app);
    monster::plugin(app);
    post_hook::plugin(app);
    end::plugin(app);
    dead::plugin(app);
    goal::plugin(app);
    tutorials::plugin(app);
}
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum PostPhysicsAppSystems {
    /// Tick timers.
    TickTimers,
    /// Change UI.
    ChangeUi,
    /// Play sounds.
    PlaySounds,
    /// Play animations.
    PlayAnimations,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

#[derive(
    SubStates, Debug, Hash, PartialEq, Eq, Clone, Default, Reflect, strum_macros::EnumIter,
)]
#[source(Screen = Screen::Gameplay)]
pub enum GameState {
    #[default]
    Tutorial,
    Idle,
    UIOpened,
    Casting,
    Waiting,
    Biting,
    Reeling,
    Failing,
    Succeeding,
    Monster,
    Dead,
    End,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub(crate) struct Pause(pub(crate) bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct PausableSystems;
