use std::time::Duration;

use bevy::math::curve::easing::EaseFunction;
use bevy::prelude::*;
use bevy_tweening::{Lens, Tween, TweenAnim};

/// Trigger this event to fade from a solid color to transparent (reveal).
#[derive(Event)]
pub struct FadeInEvent {
    /// Duration of the fade in seconds.
    pub duration: f32,
    /// The starting opaque color (default: black).
    pub start_color: Color,
}

impl Default for FadeInEvent {
    fn default() -> Self {
        Self {
            duration: 1.0,
            start_color: Color::BLACK,
        }
    }
}

/// Trigger this event to fade from transparent to a solid color (cover).
#[derive(Event)]
pub struct FadeOutEvent {
    /// Duration of the fade in seconds.
    pub duration: f32,
    /// The target opaque color (default: black).
    pub target_color: Color,
}

impl Default for FadeOutEvent {
    fn default() -> Self {
        Self {
            duration: 1.0,
            target_color: Color::BLACK,
        }
    }
}

/// Resource tracking the fade overlay entity.
#[derive(Resource, Default, Debug)]
pub struct FadeOverlay {
    pub entity: Option<Entity>,
}

/// Marker component for the full-screen fade overlay UI node.
#[derive(Component)]
pub struct FadeOverlayMarker;

struct BackgroundColorLens {
    start: Color,
    end: Color,
}

impl Lens<BackgroundColor> for BackgroundColorLens {
    fn lerp(&mut self, mut target: bevy::ecs::world::Mut<BackgroundColor>, ratio: f32) {
        let s = self.start.to_srgba();
        let e = self.end.to_srgba();
        target.0 = Color::srgba(
            s.red.lerp(e.red, ratio),
            s.green.lerp(e.green, ratio),
            s.blue.lerp(e.blue, ratio),
            s.alpha.lerp(e.alpha, ratio),
        );
    }
}

fn spawn_overlay(commands: &mut Commands, color: Color) -> Entity {
    commands
        .spawn((
            Name::new("Fade Overlay"),
            FadeOverlayMarker,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(color),
            ZIndex(1000),
        ))
        .id()
}

fn despawn_existing(
    commands: &mut Commands,
    fade_overlay: &mut FadeOverlay,
    overlay_query: &Query<Entity, With<FadeOverlayMarker>>,
) {
    if let Some(existing) = fade_overlay.entity {
        if let Ok(entity) = overlay_query.get(existing) {
            commands.entity(entity).despawn();
        }
        fade_overlay.entity = None;
    }
}

pub fn handle_fade_in(
    trigger: On<FadeInEvent>,
    mut commands: Commands,
    mut fade_overlay: ResMut<FadeOverlay>,
    overlay_query: Query<Entity, With<FadeOverlayMarker>>,
) {
    despawn_existing(&mut commands, &mut fade_overlay, &overlay_query);

    let entity = spawn_overlay(&mut commands, trigger.start_color);

    let tween = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_secs_f32(trigger.duration),
        BackgroundColorLens {
            start: trigger.start_color,
            end: trigger.start_color.with_alpha(0.0),
        },
    );
    commands.entity(entity).insert(TweenAnim::new(tween));
    fade_overlay.entity = Some(entity);
}

pub fn handle_fade_out(
    trigger: On<FadeOutEvent>,
    mut commands: Commands,
    mut fade_overlay: ResMut<FadeOverlay>,
    overlay_query: Query<Entity, With<FadeOverlayMarker>>,
) {
    despawn_existing(&mut commands, &mut fade_overlay, &overlay_query);

    let entity = spawn_overlay(&mut commands, trigger.target_color.with_alpha(0.0));

    let tween = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_secs_f32(trigger.duration),
        BackgroundColorLens {
            start: trigger.target_color.with_alpha(0.0),
            end: trigger.target_color,
        },
    );
    commands.entity(entity).insert(TweenAnim::new(tween));
    fade_overlay.entity = Some(entity);
}

/// Despawns the overlay once it becomes fully transparent after a fade-in.
pub fn cleanup_fade_overlay_on_complete(
    mut commands: Commands,
    mut fade_overlay: ResMut<FadeOverlay>,
    overlay_query: Query<(Entity, &BackgroundColor), With<FadeOverlayMarker>>,
) {
    let Some(overlay_entity) = fade_overlay.entity else {
        return;
    };
    if let Ok((entity, bg_color)) = overlay_query.get(overlay_entity) {
        if bg_color.0.to_srgba().alpha <= 0.01 {
            commands.entity(entity).despawn();
            fade_overlay.entity = None;
        }
    }
}

fn cleanup_fade_on_exit(
    mut commands: Commands,
    mut fade_overlay: ResMut<FadeOverlay>,
    overlay_query: Query<Entity, With<FadeOverlayMarker>>,
) {
    if let Some(entity) = fade_overlay.entity.take() {
        if let Ok(overlay_entity) = overlay_query.get(entity) {
            commands.entity(overlay_entity).despawn();
        }
    }
}

fn unpause_time_on_exit(mut time: ResMut<Time<Virtual>>) {
    if time.is_paused() {
        time.unpause();
    }
}

/// Plugin that adds fade-in/fade-out effects.
///
/// Provide the state that, when exited, should clean up any active fade overlay
/// and unpause virtual time.
///
/// ```rust,no_run
/// app.add_plugins(FadePlugin { exit_state: Screen::Gameplay });
/// ```
pub struct FadePlugin<S: States> {
    /// The state whose `OnExit` triggers overlay cleanup and time unpause.
    pub exit_state: S,
}

impl<S: States + Clone> Plugin for FadePlugin<S> {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<bevy_tweening::TweeningPlugin>() {
            app.add_plugins(bevy_tweening::TweeningPlugin);
        }
        app.init_resource::<FadeOverlay>();
        app.add_observer(handle_fade_in);
        app.add_observer(handle_fade_out);
        app.add_systems(Update, cleanup_fade_overlay_on_complete);
        app.add_systems(
            OnExit(self.exit_state.clone()),
            (cleanup_fade_on_exit, unpause_time_on_exit),
        );
    }
}
