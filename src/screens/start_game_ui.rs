//! The "Start Game" UI shown on the Title screen.

use bevy::{prelude::*, ui::Val::*};
use kira_ext::SFXEvent;

use crate::{
    screens::Screen,
    theme::{
        interaction::OnPress,
        prelude::{ColorPalette, UiColorName, widget},
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), spawn_start_game_ui);
}

fn spawn_start_game_ui(
    mut commands: Commands,
    palette: Res<ColorPalette>,
    asset_server: Res<AssetServer>,
) {
    let bg = palette.dark_plum;
    let title_color = palette.ivory;
    let subtitle_color = palette.ash;
    let divider_color = palette.blood_dark;

    commands.spawn((
        Name::new("Start Game UI"),
        Node {
            position_type: PositionType::Absolute,
            width: Percent(100.0),
            height: Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Px(0.0),
            ..default()
        },
        BackgroundColor(bg),
        Pickable::IGNORE,
        DespawnOnExit(Screen::Title),
        children![
            // ── Game Title ────────────────────────────────────────────────────
            (
                Name::new("Title Text"),
                Text("Symbiotic Hook".to_string()),
                TextFont {
                    font_size: FontSize::Px(80.0),
                    ..default()
                },
                TextColor(title_color),
            ),
            // ── Subtitle ──────────────────────────────────────────────────────
            (
                Name::new("Subtitle Text"),
                Text("Colleagues Under the Surface".to_string()),
                TextFont {
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
                TextColor(subtitle_color),
            ),
            // ── Divider ───────────────────────────────────────────────────────
            (
                Name::new("Divider"),
                Node {
                    width: Px(320.0),
                    height: Px(2.0),
                    margin: UiRect::vertical(Px(36.0)),
                    ..default()
                },
                BackgroundColor(divider_color),
            ),
            // ── Start Button ──────────────────────────────────────────────────
            start_button(&palette, &asset_server),
            // ── Hint ──────────────────────────────────────────────────────────
            // (
            //     Name::new("Hint Text"),
            //     Node {
            //         margin: UiRect::top(Px(24.0)),
            //         ..default()
            //     },
            //     Text("press start to begin your descent".to_string()),
            //     TextFont {
            //         font_size: FontSize::Px(14.0),
            //         ..default()
            //     },
            //     TextColor(palette.grime),
            // ),
        ],
    ));
}

/// A large, horror-styled "Start Game" button.
fn start_button(palette: &ColorPalette, asset_server: &AssetServer) -> impl Bundle {
    use crate::theme::interaction::InteractionPalette;

    let button_bg = palette.blood_dark;
    let button_hovered = palette.blood_bright;
    let button_pressed = palette.abyss_red;
    let button_text = palette.ivory;
    let border_color = palette.rust;

    (
        Name::new("Start Button Wrapper"),
        Node::default(),
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Name::new("Start Button"),
                    Button,
                    Node {
                        width: Px(480.0),
                        height: Px(72.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(Px(2.0)),
                        border_radius: BorderRadius::all(Px(4.0)),
                        ..default()
                    },
                    BackgroundColor(button_bg),
                    BorderColor::all(border_color),
                    InteractionPalette {
                        none: button_bg,
                        hovered: button_hovered,
                        pressed: button_pressed,
                    },
                    children![(
                        Name::new("Start Button Text"),
                        Text("START GAME".to_string()),
                        TextFont {
                            font_size: FontSize::Px(28.0),
                            ..default()
                        },
                        TextColor(button_text),
                        Pickable::IGNORE,
                    )],
                ))
                .observe(
                    |_: On<OnPress>,
                     mut next_screen: ResMut<NextState<Screen>>,
                     mut commands: Commands| {
                        commands.trigger(SFXEvent::ui("put"));
                        commands.trigger(SFXEvent::sfx("heavy"));
                        next_screen.set(Screen::Gameplay);
                    },
                );
        })),
    )
}
