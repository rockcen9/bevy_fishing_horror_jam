//! Reusable UI widgets & theming.

// Unused utilities may trigger this lints undesirably.
#![allow(dead_code)]

pub(crate) mod font;
pub(crate) mod interaction;
pub(crate) mod palette;
pub(crate) mod typography;
pub(crate) mod widget;

pub(crate) use font::FontHandle;
pub(crate) use typography::{panel_font, title_font};

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub(crate) use super::{
        font::FontHandle,
        interaction::InteractionPalette,
        palette::{ColorPalette, UiColorName},
        typography::{content_font, panel_font, subtitle_font, title_font},
        widget,
    };
}

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<palette::ColorPalette>();
    app.add_plugins(interaction::plugin);
    font::plugin(app);
}
