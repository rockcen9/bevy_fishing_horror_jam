mod hand;
mod head;

pub(super) fn plugin(app: &mut bevy::prelude::App) {
    hand::plugin(app);
    head::plugin(app);
}
