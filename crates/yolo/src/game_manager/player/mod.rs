use crate::prelude::*;
mod hand_drawing;

pub(crate) fn plugin(_app: &mut App) {
    hand_drawing::plugin(_app);
}
