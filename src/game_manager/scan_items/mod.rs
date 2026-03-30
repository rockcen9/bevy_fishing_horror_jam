use crate::prelude::*;

mod ui;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(ui::plugin);
}
