use bevy::{prelude::*, ui::Val::*};

use dev_debug_panel::{StateDebugPanel, next_panel_right};

use crate::prelude::*;

const COLOR_DEFAULT: Color = Color::srgba(0.15, 0.15, 0.15, 0.9);
const COLOR_ACTIVE: Color = Color::srgba(0.2, 0.6, 0.2, 1.0);

#[derive(Resource)]
struct HandPanelRight(f32);

pub(super) fn plugin(app: &mut App) {
    let right = next_panel_right(app);
    app.insert_resource(HandPanelRight(right));
    app.add_systems(OnEnter(Screen::Gameplay), spawn_panel);
    app.add_systems(
        Update,
        (handle_button_press, highlight_active_button).run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component, Clone, Copy)]
enum HandPositionButton {
    Top,
    Bottom,
    None,
}

fn spawn_panel(mut commands: Commands, config: Res<HandPanelRight>, mut spawned: Local<bool>) {
    if *spawned { return; }
    *spawned = true;
    commands.spawn((
        Name::new("HandScreen Debug Panel"),
        StateDebugPanel,
        Node {
            position_type: PositionType::Absolute,
            top: Px(8.0),
            right: Px(config.0),
            width: Px(140.0),
            flex_direction: FlexDirection::Column,
            row_gap: Px(4.0),
            padding: UiRect::all(Px(8.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        children![
            (
                Text("Hand".to_string()),
                TextFont { font_size: FontSize::Px(12.0), ..default() },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ),
            hand_button("Top", HandPositionButton::Top),
            hand_button("Bottom", HandPositionButton::Bottom),
            hand_button("None", HandPositionButton::None),
        ],
    ));
}

fn hand_button(label: &'static str, position: HandPositionButton) -> impl Bundle {
    (
        Name::new(label),
        Button,
        position,
        BackgroundColor(COLOR_DEFAULT),
        Node {
            width: Percent(100.0),
            padding: UiRect::axes(Px(12.0), Px(6.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            overflow: Overflow::clip(),
            ..default()
        },
        children![(
            Text(label.to_string()),
            TextFont {
                font_size: FontSize::Px(14.0),
                ..default()
            },
            TextColor(Color::WHITE),
            Pickable::IGNORE,
        )],
    )
}

fn handle_button_press(
    query: Query<(&Interaction, &HandPositionButton), Changed<Interaction>>,
    mut hand_screen: ResMut<RightHandScreenPosition>,
) {
    for (interaction, btn) in &query {
        if matches!(interaction, Interaction::Pressed) {
            hand_screen.right_hand = match btn {
                HandPositionButton::Top => Some(ScreenHalf::Top),
                HandPositionButton::Bottom => Some(ScreenHalf::Bottom),
                HandPositionButton::None => None,
            };
        }
    }
}

fn highlight_active_button(
    hand_screen: Res<RightHandScreenPosition>,
    mut buttons: Query<(&HandPositionButton, &mut BackgroundColor)>,
) {
    if !hand_screen.is_changed() {
        return;
    }

    for (btn, mut bg) in &mut buttons {
        let is_active = match (btn, &hand_screen.right_hand) {
            (HandPositionButton::Top, Some(ScreenHalf::Top)) => true,
            (HandPositionButton::Bottom, Some(ScreenHalf::Bottom)) => true,
            (HandPositionButton::None, None) => true,
            _ => false,
        };
        bg.0 = if is_active { COLOR_ACTIVE } else { COLOR_DEFAULT };
    }
}
