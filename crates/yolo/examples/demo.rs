//! Demo: run YOLO pose detection on the webcam feed.
//!
//! cargo run --example demo -p yolo
//!
//! Green boxes = wrists (palms), Cyan boxes = nose (face).

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use yolo::YoloConfig;

fn main() -> AppExit {
    if let Ok(_) = dotenvy::dotenv() {
        println!("Feature dev_mode enabled, .env loaded.");
    }

    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(bevy::log::LogPlugin {
                filter: "warn,yolo=info".to_string(),
                level: bevy::log::Level::INFO,
                ..default()
            })
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Window {
                    visible: false,
                    title: "YOLO Pose Demo — green=wrists  cyan=face".to_string(),
                    fit_canvas_to_parent: true,
                    resolution: bevy::window::WindowResolution::new(
                        yolo::GAME_WIDTH as u32,
                        yolo::GAME_HEIGHT as u32,
                    ),
                    ..default()
                }
                .into(),
                ..default()
            }),
    );

    app.add_systems(Startup, |mut config: ResMut<YoloConfig>| {
        config.run_inference = true;
        config.draw_detections = true;
    });
    app.add_systems(Update, make_window_visible);

    yolo::plugin(&mut app);

    app.run()
}

fn make_window_visible(mut window: Query<&mut Window>, mut frames: Local<u32>) {
    *frames += 1;
    if *frames == 5 {
        if let Ok(mut window) = window.single_mut() {
            window.visible = true;
        }
    }
}
