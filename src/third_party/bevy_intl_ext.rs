use crate::prelude::*;

use bevy_intl::*;

fn font_path_for_lang(lang: &str) -> &'static str {
    match lang {
        "zh-tw" => "fonts/jf-openhuninn-2.1.ttf",
        _ => "fonts/Quicksand-Regular.ttf",
    }
}

#[allow(dead_code)]
#[derive(Event)]
pub struct LocaleChangedEvent {
    pub lang: String,
    pub font_path: &'static str,
}

pub fn plugin(app: &mut App) {
    app.add_plugins(I18nPlugin::with_config(I18nConfig {
        use_bundled_translations: false, // Force filesystem loading  This gets ignored when bundle-only feature is enabled
        messages_folder: "messages".to_string(),
        default_lang: "en-us".to_string(),
        fallback_lang: "en-us".to_string(),
    }))
    .add_systems(Update, dispatch_locale_changed);
}

fn dispatch_locale_changed(i18n: Res<I18n>, mut commands: Commands) {
    if i18n.is_changed() {
        let lang = i18n.get_lang().to_string();
        let font_path = font_path_for_lang(&lang);
        commands.trigger(LocaleChangedEvent {
            lang: lang,
            font_path,
        });
    }
}
