use crate::prelude::*;
use signal_smooth::OneEuroFilter;

#[derive(Component)]
pub(crate) struct LeftHand;

#[derive(Component)]
pub(crate) struct RightHand;

// ── Median filter (spike removal) ─────────────────────────────────────────────
// Keeps a rolling window of the last N raw positions.
// X and Y are sorted independently and the median is returned.
// Single-frame spikes land at the extremes of the sorted list and are never
// selected as the median, so they are silently discarded.

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
        let mut xs: [f32; MEDIAN_WINDOW] = [0.0; MEDIAN_WINDOW];
        let mut ys: [f32; MEDIAN_WINDOW] = [0.0; MEDIAN_WINDOW];
        for i in 0..n {
            xs[i] = self.buf[i].x;
            ys[i] = self.buf[i].y;
        }
        xs[..n].sort_unstable_by(f32::total_cmp);
        ys[..n].sort_unstable_by(f32::total_cmp);
        Vec2::new(xs[n / 2], ys[n / 2])
    }

    fn reset(&mut self) {
        self.len = 0;
        self.head = 0;
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
            x: OneEuroFilter::new(1.0, 0.1),
            y: OneEuroFilter::new(1.0, 0.1),
        }
    }

    fn filter(&mut self, pos: Vec2, dt: f32) -> Vec2 {
        Vec2::new(self.x.filter(pos.x, dt), self.y.filter(pos.y, dt))
    }
}

// ── Combined pipeline: median → OneEuro ───────────────────────────────────────

struct HandFilter {
    median: MedianFilter,
    euro: XYFilter,
}

impl HandFilter {
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

    fn reset(&mut self) {
        self.median.reset();
    }
}

#[derive(Resource)]
struct HandFilters {
    #[cfg(feature = "left_hand")]
    left: HandFilter,
    right: HandFilter,
}

impl HandFilters {
    fn new() -> Self {
        Self {
            #[cfg(feature = "left_hand")]
            left: HandFilter::new(),
            right: HandFilter::new(),
        }
    }
}

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(HandFilters::new());
    #[cfg(all(feature = "backend", feature = "dev"))]
    app.add_systems(Startup, enable_yolo_draw_detections);
    app.add_systems(OnEnter(GameState::Tutorial), spawn_hand_sprites)
        .add_systems(OnEnter(GameState::Idle), spawn_hand_sprites)
        .add_systems(OnExit(GameState::Idle), hide_hand_sprites)
        .add_systems(OnExit(GameState::Waiting), hide_hand_sprites)
        .add_systems(OnExit(GameState::Biting), hide_hand_sprites)
        .add_systems(OnExit(GameState::Reeling), hide_hand_sprites);
    #[cfg(feature = "backend")]
    app.add_systems(
        Update,
        sync_hands_to_palm_detections.run_if(
            in_state(GameState::Idle)
                .or_else(in_state(GameState::Tutorial))
                .or_else(in_state(GameState::Casting))
                .or_else(in_state(GameState::Waiting))
                .or_else(in_state(GameState::Biting))
                .or_else(in_state(GameState::Succeeding))
                .or_else(in_state(GameState::Failing))
                .or_else(in_state(GameState::Reeling))
                .or_else(in_state(GameState::UIOpened)),
        ),
    );
    // app.add_systems(Update, draw_hand_gizmos.run_if(in_state(GameState::Idle)));
}

fn spawn_hand_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    existing: Query<(), With<RightHand>>,
) {
    if !existing.is_empty() {
        return;
    }
    let fist_scale = 0.2;
    #[cfg(feature = "left_hand")]
    commands.spawn((
        Name::new("LeftHand"),
        LeftHand,
        SpriteLayer::LeftHand,
        Sprite {
            image: asset_server.load("textures/left_hand.png"),
            ..default()
        },
        Transform::from_xyz(0., 0., 10.).with_scale(Vec3::splat(fist_scale)),
        Visibility::Hidden,
    ));

    commands.spawn((
        Name::new("RightHand"),
        RightHand,
        SpriteLayer::RightHand,
        Sprite {
            image: asset_server.load("textures/right_hand.png"),
            ..default()
        },
        Transform::from_xyz(0., 0., 10.).with_scale(Vec3::splat(fist_scale)),
        Visibility::Hidden,
    ));
}

// fn draw_hand_gizmos(
//     mut gizmos: Gizmos,
//     left: Query<(&Transform, &Visibility), With<LeftHand>>,
//     right: Query<(&Transform, &Visibility), With<RightHand>>,
// ) {
//     if let Ok((t, v)) = left.single() {
//         if *v != Visibility::Hidden {
//             gizmos.circle_2d(t.translation.truncate(), 32., Color::srgb(0.0, 1.0, 0.0));
//         }
//     }
//     if let Ok((t, v)) = right.single() {
//         if *v != Visibility::Hidden {
//             gizmos.circle_2d(t.translation.truncate(), 32., Color::srgb(1.0, 0.0, 0.0));
//         }
//     }
// }

#[cfg(all(feature = "backend", feature = "dev"))]
fn enable_yolo_draw_detections(mut cfg: ResMut<yolo::YoloConfig>) {
    cfg.draw_detections = true;
}

fn hide_hand_sprites(
    mut left: Query<&mut Visibility, (With<LeftHand>, Without<RightHand>)>,
    mut right: Query<&mut Visibility, (With<RightHand>, Without<LeftHand>)>,
) {
    if let Ok(mut lv) = left.single_mut() {
        lv.set_if_neq(Visibility::Hidden);
    }
    if let Ok(mut rv) = right.single_mut() {
        rv.set_if_neq(Visibility::Hidden);
    }
}

#[cfg(feature = "backend")]
fn sync_hands_to_palm_detections(
    detections: Res<yolo::PlayerDetections>,
    mut filters: ResMut<HandFilters>,
    time: Res<Time>,
    mut left: Query<(&mut Transform, &mut Visibility), (With<LeftHand>, Without<RightHand>)>,
    mut right: Query<(&mut Transform, &mut Visibility), (With<RightHand>, Without<LeftHand>)>,
) {
    let Ok((mut rt, mut rv)) = right.single_mut() else {
        return;
    };

    let dt = time.delta_secs();
    let (left_palm, right_palm) = detections.palm_centers();

    #[cfg(feature = "left_hand")]
    if let Ok((mut lt, mut lv)) = left.single_mut() {
        if let Some(pos) = left_palm {
            let smoothed = filters.left.filter(pos, dt);
            lt.translation = smoothed.extend(10.);
            lv.set_if_neq(Visibility::Inherited);
        } else {
            filters.left.reset();
            lv.set_if_neq(Visibility::Hidden);
        }
    }

    if let Some(pos) = right_palm {
        let smoothed = filters.right.filter(pos, dt);
        rt.translation = smoothed.extend(10.);
        rv.set_if_neq(Visibility::Inherited);
    } else {
        filters.right.reset();
        rv.set_if_neq(Visibility::Hidden);
    }
}
