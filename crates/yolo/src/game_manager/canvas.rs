use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;


pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(ShapePlugin);
}
