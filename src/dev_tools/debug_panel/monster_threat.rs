use bevy::{prelude::*, ui::Val::*};

use dev_debug_panel::StateDebugPanel;

use crate::prelude::*;
use crate::game_manager::MonsterThreat;

#[derive(Component)]
struct ThreatLabel;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_panel);
    app.add_systems(
        Update,
        update_label.run_if(in_state(Screen::Gameplay)),
    );
}

fn spawn_panel(mut commands: Commands, mut spawned: Local<bool>) {
    if *spawned { return; }
    *spawned = true;

    commands.spawn((
        Name::new("MonsterThreat Debug Panel"),
        StateDebugPanel,
        Node {
            position_type: PositionType::Absolute,
            left: Px(16.0),
            top: Percent(50.0),
            padding: UiRect::axes(Px(10.0), Px(6.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        children![(
            Name::new("ThreatLabel"),
            ThreatLabel,
            Text("Threat: 0.00".to_string()),
            TextFont {
                font_size: FontSize::Px(14.0),
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.6, 0.1)),
        )],
    ));
}

fn update_label(
    threat: Res<MonsterThreat>,
    mut label: Query<&mut Text, With<ThreatLabel>>,
) {
    if !threat.is_changed() {
        return;
    }
    if let Ok(mut text) = label.single_mut() {
        text.0 = format!("Threat: {:.2}", threat.level);
    }
}
