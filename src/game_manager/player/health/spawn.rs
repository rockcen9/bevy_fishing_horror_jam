use crate::prelude::*;

use super::PlayerHealth;

pub(crate) fn plugin(app: &mut App) {
    #[cfg(feature = "dev")]
    app.add_systems(Startup, spawn_dev_health_display)
        .add_systems(Update, (sync_health_display_text, on_health_button_pressed));
}

// ── dev-only UI ──────────────────────────────────────────────────────────────

#[cfg(feature = "dev")]
#[derive(Component)]
struct DevHealthLabel;

#[cfg(feature = "dev")]
#[derive(Component)]
enum DevHealthButton {
    Add,
    Reduce,
}

#[cfg(feature = "dev")]
const DEV_BTN_COLOR_DEFAULT: Color = Color::srgba(0.15, 0.15, 0.15, 0.9);
#[cfg(feature = "dev")]
const DEV_BTN_COLOR_HOVER: Color = Color::srgba(0.3, 0.3, 0.3, 0.9);

#[cfg(feature = "dev")]
fn build_health_button_bundle(label: &'static str, kind: DevHealthButton) -> impl Bundle {
    use bevy::ui::Val::*;
    (
        Name::new(label),
        Button,
        kind,
        BackgroundColor(DEV_BTN_COLOR_DEFAULT),
        Node {
            padding: UiRect::axes(Px(10.0), Px(4.0)),
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

#[cfg(feature = "dev")]
fn spawn_dev_health_display(mut commands: Commands) {
    use bevy::ui::Val::*;
    commands.spawn((
        Name::new("Health Display"),
        Node {
            position_type: PositionType::Absolute,
            top: Px(8.0),
            left: Px(8.0),
            flex_direction: FlexDirection::Column,
            row_gap: Px(4.0),
            padding: UiRect::all(Px(8.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        children![
            (
                Name::new("HealthLabel"),
                DevHealthLabel,
                Text("HP: 100".to_string()),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(Color::srgb(0.2, 1.0, 0.3)),
            ),
            (
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Px(4.0),
                    ..default()
                },
                children![
                    build_health_button_bundle("+10", DevHealthButton::Add),
                    build_health_button_bundle("-10", DevHealthButton::Reduce),
                ],
            ),
        ],
    ));
}

#[cfg(feature = "dev")]
fn sync_health_display_text(
    health: Res<PlayerHealth>,
    mut label: Query<&mut Text, With<DevHealthLabel>>,
) {
    if !health.is_changed() {
        return;
    }
    if let Ok(mut text) = label.single_mut() {
        text.0 = format!("HP: {:.0}", health.value);
    }
}

#[cfg(feature = "dev")]
fn on_health_button_pressed(
    mut query: Query<(&Interaction, &DevHealthButton, &mut BackgroundColor), Changed<Interaction>>,
    mut health: ResMut<PlayerHealth>,
) {
    for (interaction, btn, mut bg) in &mut query {
        match interaction {
            Interaction::Pressed => {
                let current = health.value;
                match btn {
                    DevHealthButton::Add => health.set(current + 10.0),
                    DevHealthButton::Reduce => health.set(current - 10.0),
                }
                bg.0 = DEV_BTN_COLOR_DEFAULT;
            }
            Interaction::Hovered => bg.0 = DEV_BTN_COLOR_HOVER,
            Interaction::None => bg.0 = DEV_BTN_COLOR_DEFAULT,
        }
    }
}
