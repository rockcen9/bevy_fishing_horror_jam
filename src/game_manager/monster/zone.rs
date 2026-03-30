use crate::prelude::*;

// ── Constants ──────────────────────────────────────────────────────────────

/// Vertical centre of the zone as a fraction of screen height (0 = bottom, 1 = top).
/// Derived from zone.png: red band centre sits at ~63% from top → 37% from bottom.
const ZONE_CENTER_Y_PCT: f32 = 0.37;

/// Zone height as a fraction of screen height.
/// Derived from zone.png: band spans ~22% of screen height.
const ZONE_HEIGHT_PCT: f32 = 0.22;

// ── Resource ───────────────────────────────────────────────────────────────

/// Axis-aligned bounding box for the monster zone in world space.
#[derive(Resource, Debug)]
pub struct MonsterZone {
    pub min_y: f32,
    pub max_y: f32,
}

impl MonsterZone {
    fn compute() -> Self {
        let half_h = crate::GAME_HEIGHT / 2.0;
        let center_y = -half_h + crate::GAME_HEIGHT * ZONE_CENTER_Y_PCT;
        let half_zone = crate::GAME_HEIGHT * ZONE_HEIGHT_PCT / 2.0;
        Self {
            min_y: center_y - half_zone,
            max_y: center_y + half_zone,
        }
    }
}

// ── Plugin ─────────────────────────────────────────────────────────────────

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(MonsterZone::compute());

    #[cfg(feature = "dev")]
    app.add_systems(Update, draw_zone_gizmos);
}

// ── Systems ────────────────────────────────────────────────────────────────

#[cfg(feature = "dev")]
fn draw_zone_gizmos(zone: Res<MonsterZone>, mut gizmos: Gizmos) {
    let center_y = (zone.min_y + zone.max_y) / 2.0;
    let height = zone.max_y - zone.min_y;

    gizmos.rect_2d(
        Vec2::new(0.0, center_y),
        Vec2::new(crate::GAME_WIDTH, height),
        Color::srgba(0.7, 0.05, 0.05, 0.8),
    );
}
