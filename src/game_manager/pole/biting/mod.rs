mod bend;
mod detect;
mod move_bobber;
mod pull_up;
mod qte;
mod monster_threat;
pub(crate) use monster_threat::MonsterThreat;

use crate::prelude::*;

/// Gesture detection runs first, then pull animation input reads the event,
/// then QTE input reads it — preserving the original PullInput → QteInput order.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(super) enum BitingSet {
    GestureDetect,
    PullInput,
    QteInput,
}

pub(crate) fn plugin(app: &mut App) {
    app.configure_sets(
        Update,
        (
            BitingSet::GestureDetect,
            BitingSet::PullInput,
            BitingSet::QteInput,
        )
            .chain(),
    );
    bend::plugin(app);
    detect::plugin(app);
    move_bobber::plugin(app);
    pull_up::plugin(app);
    qte::plugin(app);
    monster_threat::plugin(app);
}
