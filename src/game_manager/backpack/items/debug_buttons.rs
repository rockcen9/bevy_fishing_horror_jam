use bevy::ui::ui_transform::UiTransform;
use bevy::ui::Val2;

use crate::prelude::*;

use super::super::FishCaughtEvent;
use super::PrefabList;

const SHADOW_OFFSET: f32 = 5.0;
const BTN_W: f32 = 130.0;
const BTN_H: f32 = 44.0;

// Shadow color (semi-transparent black — no palette equivalent)
const COLOR_SHADOW: Color = Color::srgba(0.0, 0.0, 0.0, 0.75);

#[derive(Component)]
struct DebugItemButton(smol_str::SmolStr);

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_debug_buttons)
        .add_systems(Update, handle_debug_button_interaction);
}

fn spawn_debug_buttons(
    mut commands: Commands,
    palette: Res<crate::theme::palette::ColorPalette>,
) {
    commands
        .spawn((
            Name::new("DebugItemButtons"),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0),
                right: Val::Px(20.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                ..default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            for (label, prefab) in [
                ("+ Target 1", PrefabList::Target1),
                ("+ Target 2", PrefabList::Target2),
                ("+ Target 3", PrefabList::Target3),
            ] {
                let id = prefab.prefab_id().0;
                parent
                    .spawn((
                        Name::new(format!("DebugBtn_{label}")),
                        DebugItemButton(id),
                        Button,
                        Node {
                            width: Val::Px(BTN_W),
                            height: Val::Px(BTN_H),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(palette.button_background),
                        BoxShadow::new(
                            COLOR_SHADOW,
                            Val::Px(SHADOW_OFFSET),
                            Val::Px(SHADOW_OFFSET),
                            Val::Px(0.0),
                            Val::Px(3.0),
                        ),
                        UiTransform::default(),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(label),
                            TextFont {
                                font_size: FontSize::Px(15.0),
                                ..default()
                            },
                            TextColor(palette.button_text),
                            Pickable::IGNORE,
                        ));
                    });
            }
        });
}

fn handle_debug_button_interaction(
    mut query: Query<
        (
            &Interaction,
            &DebugItemButton,
            &mut BackgroundColor,
            &mut BoxShadow,
            &mut UiTransform,
        ),
        Changed<Interaction>,
    >,
    mut commands: Commands,
    palette: Res<crate::theme::palette::ColorPalette>,
) {
    for (interaction, btn, mut bg, mut shadow, mut ui_transform) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *bg = BackgroundColor(palette.button_pressed_background);
                // Clear shadow and shift down-right to simulate press
                *shadow = BoxShadow::new(
                    Color::NONE,
                    Val::Px(0.0),
                    Val::Px(0.0),
                    Val::Px(0.0),
                    Val::Px(0.0),
                );
                ui_transform.translation = Val2 {
                    x: Val::Px(SHADOW_OFFSET),
                    y: Val::Px(SHADOW_OFFSET),
                };
                commands.trigger(FishCaughtEvent {
                    prefab_id: btn.0.clone(),
                });
            }
            Interaction::Hovered => {
                *bg = BackgroundColor(palette.button_hovered_background);
                *shadow = BoxShadow::new(
                    COLOR_SHADOW,
                    Val::Px(SHADOW_OFFSET),
                    Val::Px(SHADOW_OFFSET),
                    Val::Px(0.0),
                    Val::Px(3.0),
                );
                ui_transform.translation = Val2::ZERO;
            }
            Interaction::None => {
                *bg = BackgroundColor(palette.button_background);
                *shadow = BoxShadow::new(
                    COLOR_SHADOW,
                    Val::Px(SHADOW_OFFSET),
                    Val::Px(SHADOW_OFFSET),
                    Val::Px(0.0),
                    Val::Px(3.0),
                );
                ui_transform.translation = Val2::ZERO;
            }
        }
    }
}
