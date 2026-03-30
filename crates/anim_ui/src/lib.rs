mod despawn_on;
pub use despawn_on::{AnimDespawnOnExit, IsDespawning};

mod spawn_on;
pub use spawn_on::AnimSpawnOn;

mod during;
pub use during::AnimDuring;

mod despawn;
pub use despawn::AnimDespawn;

use bevy::prelude::*;

// ── Base plugin (shared systems, added once) ──────────────────────────────────

struct AnimUiBasePlugin;

impl Plugin for AnimUiBasePlugin {
    fn build(&self, app: &mut App) {
        despawn_on::plugin(app);
        spawn_on::plugin(app);
        despawn::plugin(app);
    }
}

// ── Public plugin ─────────────────────────────────────────────────────────────

/// Add once, chaining every `States` type that uses [`AnimDespawnOnExit`]:
///
/// ```rust
/// app.add_plugins(
///     AnimUiPlugin::new()
///         .with_state::<GameState>()
///         .with_state::<MenuState>(),
/// );
/// ```
pub struct AnimUiPlugin {
    registrations: Vec<fn(&mut App)>,
}

impl AnimUiPlugin {
    pub fn new() -> Self {
        Self {
            registrations: Vec::new(),
        }
    }

    pub fn with_state<S: States>(mut self) -> Self {
        self.registrations.push(despawn_on::register::<S>);
        self.registrations.push(during::register::<S>);
        self
    }
}

impl Plugin for AnimUiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<AnimUiBasePlugin>() {
            app.add_plugins(AnimUiBasePlugin);
        }
        for register in &self.registrations {
            register(app);
        }
    }
}
