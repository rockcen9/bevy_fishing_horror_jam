use crate::prelude::*;

mod spawn_lake_points;
pub use spawn_lake_points::{LakePoint1, LakePoint2, LakePoint3};

pub fn plugin(app: &mut App) {
    spawn_lake_points::plugin(app);
}
