use std::time::Duration;

use bevy::prelude::*;
use rand::RngExt;

const SHAKE_DURATION: f32 = 0.3;
const MAX_ANGLE: f32 = 1.;
const MAX_OFFSET: f32 = 25.0;
const TRAUMA: f32 = 0.5;

/// Trigger this event to start a camera shake effect.
#[derive(Event)]
pub struct CameraShakeEvent;

/// Resource that tracks the current shake state.
#[derive(Resource, Default)]
pub struct ScreenShake {
    max_angle: f32,
    max_offset: f32,
    trauma: f32,
    timer: Timer,
    is_active: bool,
    original_position: Vec3,
    original_rotation: Quat,
}

impl ScreenShake {
    pub fn start_light_shake(&mut self, current_position: Vec3, current_rotation: Quat) {
        self.max_angle = MAX_ANGLE;
        self.max_offset = MAX_OFFSET;
        self.trauma = TRAUMA;
        self.timer = Timer::new(Duration::from_secs_f32(SHAKE_DURATION), TimerMode::Once);
        self.is_active = true;
        self.original_position = current_position;
        self.original_rotation = current_rotation;
    }

    pub fn is_shaking(&self) -> bool {
        self.is_active && !self.timer.is_finished()
    }
}

pub fn trigger_camera_shake(
    _trigger: On<CameraShakeEvent>,
    mut screen_shake: ResMut<ScreenShake>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    if let Ok(transform) = camera_query.single() {
        if screen_shake.is_shaking() {
            return;
        }
        screen_shake.start_light_shake(transform.translation, transform.rotation);
    }
}

pub fn apply_camera_shake(
    time: Res<Time>,
    mut screen_shake: ResMut<ScreenShake>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if !screen_shake.is_active {
        return;
    }

    screen_shake.timer.tick(time.delta());

    if screen_shake.timer.is_finished() {
        if let Ok(mut transform) = camera_query.single_mut() {
            transform.translation = screen_shake.original_position;
            transform.rotation = screen_shake.original_rotation;
        }
        screen_shake.is_active = false;
        return;
    }

    let time_progress = screen_shake.timer.elapsed_secs() / SHAKE_DURATION;
    let shake_intensity = screen_shake.trauma * (1.0 - time_progress);
    let shake = shake_intensity * shake_intensity;

    let mut rng = rand::rng();
    let angle = (screen_shake.max_angle * shake).to_radians() * rng.random_range(-1.0..1.0f32);
    let offset_x = screen_shake.max_offset * shake * rng.random_range(-1.0..1.0f32);
    let offset_y = screen_shake.max_offset * shake * rng.random_range(-1.0..1.0f32);

    if let Ok(mut transform) = camera_query.single_mut() {
        transform.translation =
            screen_shake.original_position + Vec3::new(offset_x, offset_y, 0.0);
        transform.rotation = screen_shake.original_rotation * Quat::from_rotation_z(angle);
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<ScreenShake>();
    app.add_observer(trigger_camera_shake);
    app.add_systems(Update, apply_camera_shake);
}
