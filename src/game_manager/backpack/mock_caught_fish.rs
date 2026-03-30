use bevy::{prelude::*, ui::Val::*};

use crate::prelude::*;

use super::FishCaughtEvent;
use crate::game_manager::pole::FishCatchSequence;
use crate::theme::interaction::{InteractionPalette, OnPress};

#[derive(Component)]
struct MockCaughtFishButton;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_mock_button);
}

fn spawn_mock_button(mut commands: Commands, palette: Res<crate::theme::palette::ColorPalette>) {
    let button_bg = palette.blood_dark;
    let button_hovered = palette.blood_bright;
    let button_pressed = palette.abyss_red;
    let button_text = palette.ivory;

    commands
        .spawn((
            Name::new("MockCaughtFishButton"),
            MockCaughtFishButton,
            Button,
            Node {
                position_type: PositionType::Absolute,
                bottom: Px(20.0),
                left: Percent(50.0),
                width: Px(240.0),
                height: Px(48.0),
                margin: UiRect::left(Px(-120.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Px(2.0)),
                border_radius: BorderRadius::all(Px(4.0)),
                ..default()
            },
            BackgroundColor(button_bg),
            BorderColor::all(palette.rust),
            InteractionPalette {
                none: button_bg,
                hovered: button_hovered,
                pressed: button_pressed,
            },
            DespawnOnExit(Screen::Gameplay),
            children![(
                Name::new("MockCaughtFishButtonText"),
                Text("Mock Catch Fish".to_string()),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextColor(button_text),
                Pickable::IGNORE,
            )],
        ))
        .observe(
            |_: On<OnPress>,
             mut bucket: ResMut<FishCatchSequence>,
             mut commands: Commands| {
                let Some(prefab_id) = bucket.pick_next_fish_prefab_id() else {
                    return;
                };
                bucket.catch_index += 1;
                commands.trigger(FishCaughtEvent {
                    prefab_id: prefab_id.0,
                });
            },
        );
}
