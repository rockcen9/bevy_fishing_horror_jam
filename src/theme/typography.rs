// typography.rs
use bevy::prelude::*;
use bevy_intl::I18n;

use super::font::FontHandle;

pub const FONT_SIZE_TITLE: f32 = 32.0;
pub const FONT_SIZE_SUBTITLE: f32 = 18.0;
pub const FONT_SIZE_CONTENT: f32 = 16.0;
pub const FONT_SIZE_SMALL: f32 = 12.0;
pub const FONT_SIZE_PANEL: f32 = 20.0;

pub fn title_font(font_handle: &FontHandle, i18n: &I18n) -> TextFont {
    TextFont {
        font: font_handle.get(i18n),
        font_size: FontSize::Px(FONT_SIZE_TITLE),
        ..default()
    }
}

pub fn subtitle_font(font_handle: &FontHandle, i18n: &I18n) -> TextFont {
    TextFont {
        font: font_handle.get(i18n),
        font_size: FontSize::Px(FONT_SIZE_SUBTITLE),
        ..default()
    }
}

pub fn content_font(font_handle: &FontHandle, i18n: &I18n) -> TextFont {
    TextFont {
        font: font_handle.get(i18n),
        font_size: FontSize::Px(FONT_SIZE_CONTENT),
        ..default()
    }
}

pub fn panel_font(font_handle: &FontHandle, i18n: &I18n) -> TextFont {
    TextFont {
        font: font_handle.get(i18n),
        font_size: FontSize::Px(FONT_SIZE_PANEL),
        ..default()
    }
}

pub fn small_font(font_handle: &FontHandle, i18n: &I18n) -> TextFont {
    TextFont {
        font: font_handle.get(i18n),
        font_size: FontSize::Px(FONT_SIZE_SMALL),
        ..default()
    }
}
