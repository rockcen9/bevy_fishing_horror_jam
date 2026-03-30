use bevy::prelude::*;

use crate::{GAME_HEIGHT, GAME_WIDTH};

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct DetectionBox {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    #[allow(dead_code)]
    score: f32,
}

impl DetectionBox {
    pub fn center(&self) -> bevy::math::Vec2 {
        bevy::math::Vec2::new(
            -((self.x1 + self.x2) / 2.0 * GAME_WIDTH - GAME_WIDTH / 2.0),
            -((self.y1 + self.y2) / 2.0 * GAME_HEIGHT - GAME_HEIGHT / 2.0),
        )
    }
}

/// Proportion of the forearm length (elbow→wrist) added past the wrist
/// to estimate the palm centre. Human anatomy: ~30% of forearm length.
const PALM_EXTRAPOLATION_K: f32 = 0.30;

#[derive(Resource, Default)]
pub struct PlayerDetections {
    pub left_wrist: Option<DetectionBox>,
    pub right_wrist: Option<DetectionBox>,
    pub faces: Vec<DetectionBox>,
    pub left_elbow: Option<DetectionBox>,
    pub right_elbow: Option<DetectionBox>,
}

impl PlayerDetections {
    /// Returns (left_palm, right_palm) estimated positions using vector extrapolation:
    ///   P_hand = P_wrist + k * (P_wrist - P_elbow)
    ///
    /// Falls back to raw wrist position when the paired elbow is not visible.
    pub fn palm_centers(&self) -> (Option<bevy::math::Vec2>, Option<bevy::math::Vec2>) {
        let extrapolate = |wrist: &DetectionBox, elbow: Option<&DetectionBox>| {
            let p_wrist = wrist.center();
            match elbow {
                Some(e) => p_wrist + PALM_EXTRAPOLATION_K * (p_wrist - e.center()),
                None => p_wrist,
            }
        };
        let left = self
            .left_wrist
            .as_ref()
            .map(|w| extrapolate(w, self.left_elbow.as_ref()));
        let right = self
            .right_wrist
            .as_ref()
            .map(|w| extrapolate(w, self.right_elbow.as_ref()));
        (left, right)
    }
}

// ── Plugin ────────────────────────────────────────────────────────────────────

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<PlayerDetections>();

    #[cfg(feature = "backend")]
    backend::plugin(app);

    app.add_systems(
        Update,
        draw_detections.run_if(|cfg: Res<crate::YoloConfig>| cfg.draw_detections),
    );
}

// ── Draw ──────────────────────────────────────────────────────────────────────

fn draw_detections(mut gizmos: Gizmos, detections: Res<PlayerDetections>) {
    for b in detections
        .left_wrist
        .iter()
        .chain(detections.right_wrist.iter())
    {
        gizmos.rect_2d(
            Vec2::new(box_cx(b), box_cy(b)),
            Vec2::new(box_w(b), box_h(b)),
            bevy::color::palettes::css::LIME,
        );
    }
    for b in &detections.faces {
        gizmos.rect_2d(
            Vec2::new(box_cx(b), box_cy(b)),
            Vec2::new(box_w(b), box_h(b)),
            bevy::color::palettes::css::AQUA,
        );
    }
    for b in detections
        .left_elbow
        .iter()
        .chain(detections.right_elbow.iter())
    {
        gizmos.rect_2d(
            Vec2::new(box_cx(b), box_cy(b)),
            Vec2::new(box_w(b), box_h(b)),
            bevy::color::palettes::css::ORANGE,
        );
    }
}

#[inline]
fn box_cx(b: &DetectionBox) -> f32 {
    -((b.x1 + b.x2) / 2.0 * GAME_WIDTH - GAME_WIDTH / 2.0)
}
#[inline]
fn box_cy(b: &DetectionBox) -> f32 {
    -((b.y1 + b.y2) / 2.0 * GAME_HEIGHT - GAME_HEIGHT / 2.0)
}
#[inline]
fn box_w(b: &DetectionBox) -> f32 {
    (b.x2 - b.x1) * GAME_WIDTH
}
#[inline]
fn box_h(b: &DetectionBox) -> f32 {
    (b.y2 - b.y1) * GAME_HEIGHT
}

// ── Backend: burn inference ────────────────────────────────────────────────────

/// ML inference systems. Only compiled when the `backend` feature is active.
#[cfg(feature = "backend")]
mod backend {
    use super::*;
    use ai_core::{BackgroundImage, BurnDevice};
    use burn::prelude::*;
    use fast_image_resize as fir;

    use crate::YoloConfig;
    use crate::model::hand_model::Model as PoseModel;

    type WgpuBackend = burn::backend::Wgpu;

    const MODEL_INPUT_SIZE: u32 = 256;
    const KP_VIS_THRESHOLD: f32 = 0.9;
    const MAX_FACES: usize = 1;
    const MAX_WRISTS_PER_HAND: usize = 1;
    const NMS_IOU_THRESHOLD: f32 = 0.45;
    const KP_BOX_HALF: f32 = 0.08;

    const KP_NOSE: usize = 0;
    const KP_LEFT_ELBOW: usize = 7;
    const KP_RIGHT_ELBOW: usize = 8;
    const KP_LEFT_WRIST: usize = 9;
    const KP_RIGHT_WRIST: usize = 10;

    const DET_COUNT: usize = 1344;
    const ANCHOR_BREAK_S16: usize = 1024;
    const ANCHOR_BREAK_S8: usize = 1280;

    // ── wasm32: shared thread-local state ─────────────────────────────────────

    #[cfg(target_arch = "wasm32")]
    use burn::backend::wgpu::WgpuDevice as BurnWgpuDevice;
    #[cfg(target_arch = "wasm32")]
    use std::{cell::RefCell, rc::Rc};

    #[cfg(target_arch = "wasm32")]
    thread_local! {
        static WASM_MODEL:     RefCell<Option<Rc<PoseModel<WgpuBackend>>>> = RefCell::new(None);
        static WASM_DEVICE:    RefCell<Option<BurnWgpuDevice>>             = RefCell::new(None);
        static WASM_RESULT:    RefCell<Option<PlayerDetections>>           = RefCell::new(None);
        static WASM_IN_FLIGHT: RefCell<bool>                               = RefCell::new(false);
    }

    // ── native: channel worker ────────────────────────────────────────────────

    #[cfg(not(target_arch = "wasm32"))]
    #[derive(Resource)]
    struct InferenceWorker {
        tx: crossbeam_channel::Sender<(Vec<u8>, u32, u32)>,
        rx: crossbeam_channel::Receiver<PlayerDetections>,
    }

    // ── Plugin ────────────────────────────────────────────────────────────────

    pub(super) fn plugin(app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            run_inference
                .run_if(|cfg: Res<YoloConfig>| cfg.run_inference)
                .before(super::draw_detections),
        );
    }

    // ── Setup ─────────────────────────────────────────────────────────────────

    fn setup(world: &mut World) {
        let device = world.resource::<BurnDevice>().0.clone();

        #[cfg(target_arch = "wasm32")]
        {
            let model = PoseModel::from_embedded(&device);
            WASM_MODEL.with(|m| *m.borrow_mut() = Some(Rc::new(model)));
            WASM_DEVICE.with(|d| *d.borrow_mut() = Some(device));
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let (frame_tx, frame_rx) = crossbeam_channel::bounded::<(Vec<u8>, u32, u32)>(1);
            let (results_tx, results_rx) =
                crossbeam_channel::bounded::<PlayerDetections>(1);

            std::thread::Builder::new()
                .stack_size(64 * 1024 * 1024)
                .spawn(move || {
                    let model = PoseModel::from_embedded(&device);
                    let resize_opts = fir::ResizeOptions::new().resize_alg(fir::ResizeAlg::Nearest);
                    let mut resizer = fir::Resizer::new();
                    let mut dst = fir::images::Image::new(
                        MODEL_INPUT_SIZE,
                        MODEL_INPUT_SIZE,
                        fir::PixelType::U8x4,
                    );
                    let chw_len = (3 * MODEL_INPUT_SIZE * MODEL_INPUT_SIZE) as usize;
                    let mut pixels = vec![0.0f32; chw_len];

                    while let Ok((raw, w, h)) = frame_rx.recv() {
                        let Some(rgba) = image::RgbaImage::from_raw(w, h, raw) else {
                            continue;
                        };

                        let src = fir::images::ImageRef::new(
                            rgba.width(),
                            rgba.height(),
                            rgba.as_raw(),
                            fir::PixelType::U8x4,
                        )
                        .expect("valid source image");
                        resizer
                            .resize(&src, &mut dst, Some(&resize_opts))
                            .expect("resize");

                        u8x4_to_f32_chw(dst.buffer(), &mut pixels, MODEL_INPUT_SIZE as usize);

                        let t0 = std::time::Instant::now();
                        let input =
                            Tensor::<WgpuBackend, 1>::from_floats(pixels.as_slice(), &device)
                                .reshape([
                                    1i32,
                                    3i32,
                                    MODEL_INPUT_SIZE as i32,
                                    MODEL_INPUT_SIZE as i32,
                                ]);

                        let output = model.forward(input);
                        let output_data = output.into_data();
                        debug!("inference time: {}ms", t0.elapsed().as_millis());

                        let flat: Vec<f32> = output_data.iter::<f32>().collect();
                        let _ = results_tx.send(parse_pose_output(&flat));
                    }
                })
                .unwrap();

            world.insert_resource(InferenceWorker {
                tx: frame_tx,
                rx: results_rx,
            });
        }
    }

    // ── Inference: native ─────────────────────────────────────────────────────

    #[cfg(not(target_arch = "wasm32"))]
    fn run_inference(
        worker: Res<InferenceWorker>,
        bg_image: Res<BackgroundImage>,
        mut detections: ResMut<PlayerDetections>,
        mut pending: Local<bool>,
    ) {
        if let Ok(result) = worker.rx.try_recv() {
            *detections = result;
            *pending = false;
        }
        if *pending {
            return;
        }

        let w = bg_image.width();
        let h = bg_image.height();
        let Some(data) = bg_image.data.as_ref() else {
            return;
        };
        if w == 0 || h == 0 {
            return;
        }

        if worker.tx.try_send((data.clone(), w, h)).is_ok() {
            *pending = true;
        }
    }

    // ── Inference: wasm32 ─────────────────────────────────────────────────────

    #[cfg(target_arch = "wasm32")]
    fn run_inference(bg_image: Res<BackgroundImage>, mut detections: ResMut<PlayerDetections>) {
        WASM_RESULT.with(|r| {
            if let Some(result) = r.borrow_mut().take() {
                *detections = result;
            }
        });

        if WASM_IN_FLIGHT.with(|f| *f.borrow()) {
            return;
        }

        let w = bg_image.width();
        let h = bg_image.height();
        let Some(data) = bg_image.data.as_ref() else {
            return;
        };
        if w == 0 || h == 0 {
            return;
        }

        let raw = data.clone();
        let model = WASM_MODEL.with(|m| m.borrow().clone());
        let device = WASM_DEVICE.with(|d| d.borrow().clone());
        let (Some(model), Some(device)) = (model, device) else {
            return;
        };

        WASM_IN_FLIGHT.with(|f| *f.borrow_mut() = true);

        wasm_bindgen_futures::spawn_local(async move {
            let Some(rgba) = image::RgbaImage::from_raw(w, h, raw) else {
                WASM_IN_FLIGHT.with(|f| *f.borrow_mut() = false);
                return;
            };

            let pixels = resize_and_chw_alloc(&rgba, MODEL_INPUT_SIZE);

            let t0 = js_sys::Date::now();
            let input = Tensor::<WgpuBackend, 1>::from_floats(pixels.as_slice(), &device)
                .reshape([1i32, 3i32, MODEL_INPUT_SIZE as i32, MODEL_INPUT_SIZE as i32]);

            let output = model.forward(input);
            let output_data = output.into_data_async().await.unwrap();
            info!("inference time: {}ms", (js_sys::Date::now() - t0) as u64);

            let flat: Vec<f32> = output_data.iter::<f32>().collect();
            WASM_RESULT.with(|r| *r.borrow_mut() = Some(parse_pose_output(&flat)));
            WASM_IN_FLIGHT.with(|f| *f.borrow_mut() = false);
        });
    }

    // ── Pose output parsing ───────────────────────────────────────────────────

    fn parse_pose_output(flat: &[f32]) -> PlayerDetections {
        if flat.len() < 51 * DET_COUNT {
            return PlayerDetections::default();
        }

        let inv = 1.0 / MODEL_INPUT_SIZE as f32;
        let n = DET_COUNT;

        let mut face_cands: Vec<DetectionBox> = Vec::new();
        let mut left_elbow_cands: Vec<DetectionBox> = Vec::new();
        let mut right_elbow_cands: Vec<DetectionBox> = Vec::new();
        let mut left_wrist_cands: Vec<DetectionBox> = Vec::new();
        let mut right_wrist_cands: Vec<DetectionBox> = Vec::new();

        for a in 0..n {
            let (gx, gy, stride) = anchor_grid(a);

            let decode = |kp: usize| -> (f32, f32, f32) {
                let ch = kp * 3;
                let vis = sigmoid(flat[(ch + 2) * n + a]);
                let raw_x = flat[ch * n + a];
                let raw_y = flat[(ch + 1) * n + a];
                let x = (raw_x + gx + 0.5) * stride * inv;
                let y = (raw_y + gy + 0.5) * stride * inv;
                (x, y, vis)
            };

            let (nx, ny, nv) = decode(KP_NOSE);
            if nv > KP_VIS_THRESHOLD {
                face_cands.push(kp_box(nx, ny, nv));
            }

            let (lex, ley, lev) = decode(KP_LEFT_ELBOW);
            if lev > KP_VIS_THRESHOLD {
                left_elbow_cands.push(kp_box(lex, ley, lev));
            }

            let (rex, rey, rev) = decode(KP_RIGHT_ELBOW);
            if rev > KP_VIS_THRESHOLD {
                right_elbow_cands.push(kp_box(rex, rey, rev));
            }

            let (lx, ly, lv) = decode(KP_LEFT_WRIST);
            if lv > KP_VIS_THRESHOLD {
                left_wrist_cands.push(kp_box(lx, ly, lv));
            }

            let (rx, ry, rv) = decode(KP_RIGHT_WRIST);
            if rv > KP_VIS_THRESHOLD {
                right_wrist_cands.push(kp_box(rx, ry, rv));
            }
        }

        let nms_apply = |mut cands: Vec<DetectionBox>, max: usize| -> Vec<DetectionBox> {
            cands.sort_unstable_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
            let kept = nms_greedy(&cands);
            kept.into_iter()
                .take(max)
                .map(|i| cands[i].clone())
                .collect()
        };

        PlayerDetections {
            left_wrist: nms_apply(left_wrist_cands, MAX_WRISTS_PER_HAND).into_iter().next(),
            right_wrist: nms_apply(right_wrist_cands, MAX_WRISTS_PER_HAND).into_iter().next(),
            faces: nms_apply(face_cands, MAX_FACES),
            left_elbow: nms_apply(left_elbow_cands, MAX_WRISTS_PER_HAND).into_iter().next(),
            right_elbow: nms_apply(right_elbow_cands, MAX_WRISTS_PER_HAND).into_iter().next(),
        }
    }

    fn anchor_grid(a: usize) -> (f32, f32, f32) {
        if a < ANCHOR_BREAK_S16 {
            ((a % 32) as f32, (a / 32) as f32, 8.0)
        } else if a < ANCHOR_BREAK_S8 {
            let b = a - ANCHOR_BREAK_S16;
            ((b % 16) as f32, (b / 16) as f32, 16.0)
        } else {
            let b = a - ANCHOR_BREAK_S8;
            ((b % 8) as f32, (b / 8) as f32, 32.0)
        }
    }

    #[inline]
    fn sigmoid(x: f32) -> f32 {
        1.0 / (1.0 + (-x).exp())
    }

    fn nms_greedy(boxes: &[DetectionBox]) -> Vec<usize> {
        let mut suppressed = vec![false; boxes.len()];
        let mut kept = Vec::new();
        for i in 0..boxes.len() {
            if suppressed[i] {
                continue;
            }
            kept.push(i);
            for j in (i + 1)..boxes.len() {
                if !suppressed[j] && iou(&boxes[i], &boxes[j]) > NMS_IOU_THRESHOLD {
                    suppressed[j] = true;
                }
            }
        }
        kept
    }

    fn iou(a: &DetectionBox, b: &DetectionBox) -> f32 {
        let ix1 = a.x1.max(b.x1);
        let iy1 = a.y1.max(b.y1);
        let ix2 = a.x2.min(b.x2);
        let iy2 = a.y2.min(b.y2);
        let inter = (ix2 - ix1).max(0.0) * (iy2 - iy1).max(0.0);
        if inter == 0.0 {
            return 0.0;
        }
        let area_a = (a.x2 - a.x1) * (a.y2 - a.y1);
        let area_b = (b.x2 - b.x1) * (b.y2 - b.y1);
        let union = area_a + area_b - inter;
        if union <= 0.0 { 0.0 } else { inter / union }
    }

    fn kp_box(cx: f32, cy: f32, score: f32) -> DetectionBox {
        DetectionBox {
            x1: (cx - KP_BOX_HALF).max(0.0),
            y1: (cy - KP_BOX_HALF).max(0.0),
            x2: (cx + KP_BOX_HALF).min(1.0),
            y2: (cy + KP_BOX_HALF).min(1.0),
            score,
        }
    }

    // ── Pixel helpers ─────────────────────────────────────────────────────────

    #[cfg(not(target_arch = "wasm32"))]
    fn u8x4_to_f32_chw(src: &[u8], dst: &mut [f32], size: usize) {
        let inv = 1.0 / 255.0_f32;
        let n = size * size;
        for i in 0..n {
            dst[i] = src[i * 4] as f32 * inv;
            dst[n + i] = src[i * 4 + 1] as f32 * inv;
            dst[2 * n + i] = src[i * 4 + 2] as f32 * inv;
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn resize_and_chw_alloc(rgba: &image::RgbaImage, target_size: u32) -> Vec<f32> {
        let src = fir::images::ImageRef::new(
            rgba.width(),
            rgba.height(),
            rgba.as_raw(),
            fir::PixelType::U8x4,
        )
        .expect("valid source image");
        let mut dst = fir::images::Image::new(target_size, target_size, fir::PixelType::U8x4);
        let opts = fir::ResizeOptions::new().resize_alg(fir::ResizeAlg::Nearest);
        fir::Resizer::new()
            .resize(&src, &mut dst, Some(&opts))
            .expect("resize");

        let n = (target_size * target_size) as usize;
        let mut out = vec![0.0f32; 3 * n];
        let buf = dst.buffer();
        let inv = 1.0 / 255.0_f32;
        for i in 0..n {
            out[i] = buf[i * 4] as f32 * inv;
            out[n + i] = buf[i * 4 + 1] as f32 * inv;
            out[2 * n + i] = buf[i * 4 + 2] as f32 * inv;
        }
        out
    }
}
