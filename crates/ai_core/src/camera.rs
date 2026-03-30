use bevy::camera::ScalingMode;
use bevy::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use bevy::camera_controller::pan_camera::PanCamera;

#[derive(Component)]
pub struct MainCamera;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_camera);
}

/// Spawns the main orthographic camera (no real camera capture).
/// When the `backend` feature is active, `backend::plugin` will attach
/// the nokhwa `BackgroundCamera` (native) or start the web camera (wasm).
pub(crate) fn setup_camera(mut commands: Commands) {
    #[cfg(not(target_arch = "wasm32"))]
    commands.spawn((
        Name::new("World Camera"),
        Camera2d,
        MainCamera,
        PanCamera {
            zoom_factor: 1.,
            min_zoom: 0.5,
            max_zoom: 4.,
            key_rotate_ccw: None,
            key_rotate_cw: None,
            ..default()
        },
        Transform::from_translation(Vec3::ZERO),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed { width: 1920.0, height: 1080.0 },
            scale: 1.,
            ..OrthographicProjection::default_2d()
        }),
    ));

    #[cfg(target_arch = "wasm32")]
    commands.spawn((
        Name::new("World Camera"),
        Camera2d,
        MainCamera,
        Transform::from_translation(Vec3::ZERO),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed { width: 1920.0, height: 1080.0 },
            scale: 1.,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

// ── Backend: real camera capture ──────────────────────────────────────────────

/// Camera capture plugins (nokhwa on native, web API on wasm32).
/// Only compiled when the `backend` feature is active.
#[cfg(feature = "backend")]
pub(crate) mod backend {
    use bevy::prelude::*;

    pub(crate) fn plugin(app: &mut App) {
        #[cfg(not(target_arch = "wasm32"))]
        nokhwa_plugin(app);

        #[cfg(target_arch = "wasm32")]
        web_camera_plugin(app);
    }

    // ── Native: nokhwa ────────────────────────────────────────────────────────

    #[cfg(not(target_arch = "wasm32"))]
    fn nokhwa_plugin(app: &mut App) {
        use bevy_nokhwa::camera::BackgroundCamera;
        use bevy_nokhwa::nokhwa::utils::{
            ApiBackend, CameraFormat, CameraIndex, FrameFormat, RequestedFormatType, Resolution,
        };

        app.add_plugins(bevy_nokhwa::BevyNokhwaPlugin)
            .add_systems(Startup, add_background_camera.after(super::setup_camera));

        fn add_background_camera(
            query: Query<Entity, With<super::MainCamera>>,
            mut commands: Commands,
        ) {
            let Ok(entity) = query.single() else {
                return;
            };
            let bg_cam = BackgroundCamera::new(
                ApiBackend::Auto,
                Some(CameraIndex::Index(0)),
                Some(RequestedFormatType::Closest(CameraFormat::new(
                    Resolution::new(640, 480),
                    FrameFormat::YUYV,
                    30,
                ))),
            )
            .unwrap();
            commands.entity(entity).insert(bg_cam);
        }
    }

    // ── wasm32: web camera ────────────────────────────────────────────────────

    #[cfg(target_arch = "wasm32")]
    fn web_camera_plugin(app: &mut App) {
        use std::cell::RefCell;

        use bevy::asset::RenderAssetUsages;
        use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::spawn_local;
        use web_sys::{HtmlCanvasElement, HtmlVideoElement};

        use crate::background_image::BackgroundImage;

        thread_local! {
            static VIDEO: RefCell<Option<HtmlVideoElement>> = const { RefCell::new(None) };
            static CANVAS: RefCell<Option<HtmlCanvasElement>> = const { RefCell::new(None) };
        }

        app.init_resource::<BackgroundImage>()
            .add_systems(Startup, start_camera)
            .add_systems(Update, read_camera_frame);

        fn start_camera() {
            spawn_local(async move {
                let window = match web_sys::window() {
                    Some(w) => w,
                    None => {
                        error!("web_camera: no window");
                        return;
                    }
                };
                let document = match window.document() {
                    Some(d) => d,
                    None => {
                        error!("web_camera: no document");
                        return;
                    }
                };

                let video = document
                    .create_element("video")
                    .unwrap()
                    .dyn_into::<HtmlVideoElement>()
                    .unwrap();
                video.set_autoplay(true);
                video.set_muted(true);
                video.set_attribute("playsinline", "true").unwrap();
                video.set_attribute("style", "display:none").unwrap();
                document.body().unwrap().append_child(&video).unwrap();

                let canvas = document
                    .create_element("canvas")
                    .unwrap()
                    .dyn_into::<HtmlCanvasElement>()
                    .unwrap();
                canvas.set_attribute("style", "display:none").unwrap();
                document.body().unwrap().append_child(&canvas).unwrap();

                let navigator = window.navigator();
                let media_devices = match navigator.media_devices() {
                    Ok(md) => md,
                    Err(e) => {
                        error!("web_camera: navigator.mediaDevices unavailable: {:?}", e);
                        return;
                    }
                };

                let mut constraints = web_sys::MediaStreamConstraints::new();
                constraints.video(&wasm_bindgen::JsValue::TRUE);

                let promise = match media_devices.get_user_media_with_constraints(&constraints) {
                    Ok(p) => p,
                    Err(e) => {
                        error!("web_camera: getUserMedia failed: {:?}", e);
                        return;
                    }
                };

                let stream_js = match wasm_bindgen_futures::JsFuture::from(promise).await {
                    Ok(s) => s,
                    Err(e) => {
                        error!("web_camera: getUserMedia promise rejected: {:?}", e);
                        return;
                    }
                };
                let stream = stream_js.dyn_into::<web_sys::MediaStream>().unwrap();
                video.set_src_object(Some(&stream));

                if let Err(e) =
                    wasm_bindgen_futures::JsFuture::from(video.play().unwrap()).await
                {
                    error!("web_camera: video.play() failed: {:?}", e);
                    return;
                }

                VIDEO.with(|v| *v.borrow_mut() = Some(video));
                CANVAS.with(|c| *c.borrow_mut() = Some(canvas));

                info!("web_camera: camera started");
            });
        }

        fn read_camera_frame(mut bg_image: ResMut<BackgroundImage>) {
            VIDEO.with(|v| {
                CANVAS.with(|c| {
                    let video_ref = v.borrow();
                    let canvas_ref = c.borrow();
                    let (Some(video), Some(canvas)) =
                        (video_ref.as_ref(), canvas_ref.as_ref())
                    else {
                        return;
                    };

                    let w = video.video_width();
                    let h = video.video_height();
                    if w == 0 || h == 0 {
                        return;
                    }

                    canvas.set_width(w);
                    canvas.set_height(h);

                    let ctx = canvas
                        .get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<web_sys::CanvasRenderingContext2d>()
                        .unwrap();

                    if let Err(e) = ctx.draw_image_with_html_video_element(video, 0.0, 0.0) {
                        error!("web_camera: drawImage failed: {:?}", e);
                        return;
                    }

                    let image_data =
                        match ctx.get_image_data(0.0, 0.0, w as f64, h as f64) {
                            Ok(d) => d,
                            Err(e) => {
                                error!("web_camera: getImageData failed: {:?}", e);
                                return;
                            }
                        };

                    let pixels: Vec<u8> = image_data.data().0.to_vec();

                    bg_image.0 = bevy::image::Image::new(
                        Extent3d {
                            width: w,
                            height: h,
                            depth_or_array_layers: 1,
                        },
                        TextureDimension::D2,
                        pixels,
                        TextureFormat::Rgba8Unorm,
                        RenderAssetUsages::default(),
                    );
                });
            });
        }
    }
}
