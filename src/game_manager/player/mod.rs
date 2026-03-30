use crate::prelude::*;
mod hand_drawing;
pub(crate) use hand_drawing::{LeftHand, RightHand};
mod hand_screen;
pub use hand_screen::{RightHandScreenPosition, ScreenHalf};
mod head;
pub use head::PlayerHeadPosition;
mod health;
pub use health::PlayerHealth;
#[cfg(not(feature = "backend"))]
mod mock;

pub(crate) fn plugin(app: &mut App) {
    hand_drawing::plugin(app);
    hand_screen::plugin(app);
    head::plugin(app);
    health::plugin(app);
    #[cfg(not(feature = "backend"))]
    mock::plugin(app);
}
