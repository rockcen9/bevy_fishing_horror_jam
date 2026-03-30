use crate::prelude::*;
use signal_smooth::OneEuroFilter;

#[derive(Resource, Default, Debug)]
pub struct PlayerHeadPosition {
    pub position: Vec2,
}

// ── Median filter (spike / false-detection removal) ───────────────────────────

const MEDIAN_WINDOW: usize = 5;

struct MedianFilter {
    buf: [Vec2; MEDIAN_WINDOW],
    len: usize,
    head: usize,
}

impl MedianFilter {
    fn new() -> Self {
        Self {
            buf: [Vec2::ZERO; MEDIAN_WINDOW],
            len: 0,
            head: 0,
        }
    }

    fn push(&mut self, pos: Vec2) -> Vec2 {
        self.buf[self.head] = pos;
        self.head = (self.head + 1) % MEDIAN_WINDOW;
        if self.len < MEDIAN_WINDOW {
            self.len += 1;
        }
        let n = self.len;
        let mut xs = [0.0f32; MEDIAN_WINDOW];
        let mut ys = [0.0f32; MEDIAN_WINDOW];
        for i in 0..n {
            xs[i] = self.buf[i].x;
            ys[i] = self.buf[i].y;
        }
        xs[..n].sort_unstable_by(f32::total_cmp);
        ys[..n].sort_unstable_by(f32::total_cmp);
        Vec2::new(xs[n / 2], ys[n / 2])
    }
}

// ── OneEuro smoothing ─────────────────────────────────────────────────────────

struct XYFilter {
    x: OneEuroFilter,
    y: OneEuroFilter,
}

impl XYFilter {
    fn new() -> Self {
        Self {
            x: OneEuroFilter::new(1.0, 0.007),
            y: OneEuroFilter::new(1.0, 0.007),
        }
    }

    fn filter(&mut self, pos: Vec2, dt: f32) -> Vec2 {
        Vec2::new(self.x.filter(pos.x, dt), self.y.filter(pos.y, dt))
    }
}

#[derive(Resource)]
struct HeadFilter {
    median: MedianFilter,
    euro: XYFilter,
}

impl HeadFilter {
    fn new() -> Self {
        Self {
            median: MedianFilter::new(),
            euro: XYFilter::new(),
        }
    }

    fn filter(&mut self, pos: Vec2, dt: f32) -> Vec2 {
        let denoised = self.median.push(pos);
        self.euro.filter(denoised, dt)
    }
}

// ── Plugin ────────────────────────────────────────────────────────────────────

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<PlayerHeadPosition>();
    #[cfg(feature = "backend")]
    {
        app.insert_resource(HeadFilter::new());
        app.add_systems(Update, sync_head_location_from_detections);
    }
    #[cfg(feature = "dev")]
    app.add_systems(Update, draw_head_gizmo);
}

#[cfg(feature = "backend")]
fn sync_head_location_from_detections(
    detections: Res<yolo::PlayerDetections>,
    mut head: ResMut<PlayerHeadPosition>,
    mut filter: ResMut<HeadFilter>,
    time: Res<Time>,
    mut initialized: Local<bool>,
) {
    let Some(face) = detections.faces.get(0) else {
        return;
    };
    let raw = face.center();
    if *initialized && head.position.distance(raw) > MAX_HEAD_JUMP {
        return;
    }
    *initialized = true;
    head.position = filter.filter(raw, time.delta_secs());
}

#[cfg(feature = "dev")]
fn draw_head_gizmo(head: Res<PlayerHeadPosition>, mut gizmos: Gizmos) {
    gizmos.circle_2d(head.position, 32., bevy::color::palettes::css::YELLOW);
}
/// Maximum distance (pixels) a new detection can be from the current position.
/// Detections that jump further than this are rejected as false positives.
const MAX_HEAD_JUMP: f32 = 300.0;
