use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct ColorPalette {
    // Maggot24 palette — all 24 colors
    pub dark_plum: Color,       // #201114
    pub deep_maroon: Color,     // #30171a
    pub abyss_red: Color,       // #4a0a00
    pub blood_dark: Color,      // #630f00
    pub blood_mid: Color,       // #742401
    pub blood_bright: Color,    // #8f2100
    pub rust_red: Color,        // #a13300
    pub burnt_orange: Color,    // #b45102
    pub amber: Color,           // #c67c3c
    pub flesh: Color,           // #c8a589
    pub bone: Color,            // #ded6c3
    pub ivory: Color,           // #efede9
    pub ash: Color,             // #b6b6b4
    pub moss: Color,            // #85977f
    pub lichen: Color,          // #7b7d72
    pub murk: Color,            // #6a756f
    pub mud: Color,             // #786959
    pub grime: Color,           // #5d5656
    pub brine: Color,           // #475059
    pub dusk: Color,            // #514656
    pub rust: Color,            // #85383b
    pub rot: Color,             // #543634
    pub bark: Color,            // #492b1d
    pub void: Color,            // #212226

    // Semantic UI aliases
    pub screen_background: Color,
    pub header_text: Color,
    pub label_text: Color,
    pub button_background: Color,
    pub button_hovered_background: Color,
    pub button_pressed_background: Color,
    pub button_text: Color,

    // Semantic game aliases
    pub terminal_text: Color,      // bright-green terminal text for end/dead sequences
    pub panel_bg: Color,           // dark horror panel background
    pub panel_image_bg: Color,     // panel image section background
    pub panel_divider: Color,      // panel section divider
    pub panel_content_text: Color, // panel body text
    pub panel_dim_text: Color,     // dimmed/subdued panel text
    pub backpack_slot: Color,      // backpack inventory slot background
}

impl Default for ColorPalette {
    fn default() -> Self {
        let dark_plum       = Color::srgb_u8(0x20, 0x11, 0x14);
        let deep_maroon     = Color::srgb_u8(0x30, 0x17, 0x1a);
        let abyss_red       = Color::srgb_u8(0x4a, 0x0a, 0x00);
        let blood_dark      = Color::srgb_u8(0x63, 0x0f, 0x00);
        let blood_mid       = Color::srgb_u8(0x74, 0x24, 0x01);
        let blood_bright    = Color::srgb_u8(0x8f, 0x21, 0x00);
        let rust_red        = Color::srgb_u8(0xa1, 0x33, 0x00);
        let burnt_orange    = Color::srgb_u8(0xb4, 0x51, 0x02);
        let amber           = Color::srgb_u8(0xc6, 0x7c, 0x3c);
        let flesh           = Color::srgb_u8(0xc8, 0xa5, 0x89);
        let bone            = Color::srgb_u8(0xde, 0xd6, 0xc3);
        let ivory           = Color::srgb_u8(0xef, 0xed, 0xe9);
        let ash             = Color::srgb_u8(0xb6, 0xb6, 0xb4);
        let moss            = Color::srgb_u8(0x85, 0x97, 0x7f);
        let lichen          = Color::srgb_u8(0x7b, 0x7d, 0x72);
        let murk            = Color::srgb_u8(0x6a, 0x75, 0x6f);
        let mud             = Color::srgb_u8(0x78, 0x69, 0x59);
        let grime           = Color::srgb_u8(0x5d, 0x56, 0x56);
        let brine           = Color::srgb_u8(0x47, 0x50, 0x59);
        let dusk            = Color::srgb_u8(0x51, 0x46, 0x56);
        let rust            = Color::srgb_u8(0x85, 0x38, 0x3b);
        let rot             = Color::srgb_u8(0x54, 0x36, 0x34);
        let bark            = Color::srgb_u8(0x49, 0x2b, 0x1d);
        let void            = Color::srgb_u8(0x21, 0x22, 0x26);

        Self {
            dark_plum,
            deep_maroon,
            abyss_red,
            blood_dark,
            blood_mid,
            blood_bright,
            rust_red,
            burnt_orange,
            amber,
            flesh,
            bone,
            ivory,
            ash,
            moss,
            lichen,
            murk,
            mud,
            grime,
            brine,
            dusk,
            rust,
            rot,
            bark,
            void,

            screen_background:          void,
            header_text:                ivory,
            label_text:                 bone,
            button_background:          blood_dark,
            button_hovered_background:  blood_bright,
            button_pressed_background:  abyss_red,
            button_text:                ivory,

            terminal_text:              Color::srgba(0.0, 0.9, 0.4, 1.0),
            panel_bg:                   dark_plum,
            panel_image_bg:             deep_maroon,
            panel_divider:              grime,
            panel_content_text:         bone,
            panel_dim_text:             ash,
            backpack_slot:              grime,
        }
    }
}

impl ColorPalette {
    pub fn get(&self, name: UiColorName) -> Color {
        match name {
            UiColorName::ScreenBackground       => self.screen_background,
            UiColorName::HeaderText             => self.header_text,
            UiColorName::LabelText              => self.label_text,
            UiColorName::ButtonBackground       => self.button_background,
            UiColorName::ButtonHoveredBackground => self.button_hovered_background,
            UiColorName::ButtonPressedBackground => self.button_pressed_background,
            UiColorName::ButtonText             => self.button_text,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UiColorName {
    ScreenBackground,
    HeaderText,
    LabelText,
    ButtonBackground,
    ButtonHoveredBackground,
    ButtonPressedBackground,
    ButtonText,
}
