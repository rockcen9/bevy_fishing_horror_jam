//! # camera_effects
//!
//! Camera effects for Bevy: screen shake and fade transitions.
//!
//! ## Usage
//!
//! Add the plugin, then trigger events from any system:
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use camera_effects::{CameraShakeEvent, FadeInEvent, FadeOutEvent};
//!
//! fn my_system(mut commands: Commands) {
//!     commands.trigger(CameraShakeEvent);
//!     commands.trigger(FadeInEvent::default());
//!     commands.trigger(FadeOutEvent { duration: 0.5, target_color: Color::BLACK });
//! }
//! ```

use bevy::prelude::*;

pub mod fade;
pub mod shake;

pub use fade::{FadeInEvent, FadeOutEvent, FadeOverlay, FadeOverlayMarker, FadePlugin};
pub use shake::{CameraShakeEvent, ScreenShake};

/// Add this plugin to your app to enable camera shake effects.
/// For fade effects, add [`FadePlugin`] with your desired exit state.
pub fn plugin(app: &mut App) {
    shake::plugin(app);
}
