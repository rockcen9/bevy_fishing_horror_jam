mod hooked_fish;
pub(crate) use hooked_fish::{FishCatchSequence, HookedFish};

pub(crate) fn plugin(app: &mut bevy::app::App) {
    hooked_fish::plugin(app);
}
