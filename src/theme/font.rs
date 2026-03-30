use bevy_intl::I18n;
use crate::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<FontHandle>();
}

#[derive(Resource)]
pub struct FontHandle {
    pub en_us_font: Handle<Font>,
    pub zh_tw_font: Handle<Font>,
}

impl FontHandle {
    pub fn get(&self, i18n: &I18n) -> bevy::text::FontSource {
        match i18n.get_lang() {
            "zh-tw" => self.zh_tw_font.clone().into(),
            _ => self.en_us_font.clone().into(),
        }
    }
}

impl FromWorld for FontHandle {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let en_us_font = asset_server.load("fonts/Quicksand-Regular.ttf");
        let zh_tw_font = asset_server.load("fonts/jf-openhuninn-2.1.ttf");
        Self {
            en_us_font,
            zh_tw_font,
        }
    }
}
