mod casting;
pub(crate) use casting::*;
mod hide;
mod idle;
mod spawn_pole;
pub use spawn_pole::{Pole, PoleMaterial, PoleTip};
mod biting;
pub(crate) use biting::MonsterThreat;
mod waiting;
pub(crate) use waiting::{FishCatchSequence, HookedFish};

pub(crate) fn plugin(app: &mut bevy::app::App) {
    spawn_pole::plugin(app);
    idle::plugin(app);
    casting::plugin(app);
    biting::plugin(app);
    waiting::plugin(app);
    hide::plugin(app);
}
