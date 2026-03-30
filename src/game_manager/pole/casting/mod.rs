mod bobber;
mod fish_line;
mod forward;
pub(crate) use bobber::*;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    forward::plugin(app);
    fish_line::plugin(app);
    bobber::plugin(app);
}
