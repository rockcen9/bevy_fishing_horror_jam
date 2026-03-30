mod items_json;
pub(crate) use items_json::*;

pub fn plugin(app: &mut bevy::prelude::App) {
    items_json::plugin(app);
}
