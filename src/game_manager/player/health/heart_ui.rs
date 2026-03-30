use crate::prelude::*;

use super::PlayerHealth;

// heart.png is 1053x754
const HEART_WIDTH: f32 = 80.0;
const HEART_HEIGHT: f32 = HEART_WIDTH * 754.0 / 1053.0;
const HEART_GAP: f32 = 8.0;
const HEART_MARGIN: f32 = 24.0;
const HEART_EMPTY_COLOR: Color = Color::srgba(0.15, 0.15, 0.15, 0.4);

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_heart_ui)
        .add_systems(Update, update_heart_display);
}

#[derive(Component)]
struct HeartFillClipper {
    slot: usize,
}

fn health_to_half_hearts(value: f32) -> u32 {
    if value >= 100.0 {
        6
    } else if value >= 66.0 {
        5
    } else if value >= 49.0 {
        4
    } else if value >= 33.0 {
        3
    } else if value >= 16.0 {
        2
    } else if value > 0.0 {
        1
    } else {
        0
    }
}

fn spawn_heart_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("textures/heart.png");

    commands
        .spawn((
            Name::new("HeartUI"),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(HEART_MARGIN),
                left: Val::Px(HEART_MARGIN),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(HEART_GAP),
                ..default()
            },
        ))
        .with_children(|parent| {
            for slot in 0..3usize {
                parent
                    .spawn((
                        Name::new(format!("HeartSlot{slot}")),
                        Node {
                            width: Val::Px(HEART_WIDTH),
                            height: Val::Px(HEART_HEIGHT),
                            ..default()
                        },
                    ))
                    .with_children(|slot_parent| {
                        // Empty/ghost heart always visible as background
                        slot_parent.spawn((
                            Name::new("HeartEmpty"),
                            ImageNode {
                                image: texture.clone(),
                                color: HEART_EMPTY_COLOR,
                                ..default()
                            },
                            Node {
                                position_type: PositionType::Absolute,
                                width: Val::Px(HEART_WIDTH),
                                height: Val::Px(HEART_HEIGHT),
                                ..default()
                            },
                        ));

                        // Colored fill clipped to show full, half, or nothing
                        slot_parent
                            .spawn((
                                Name::new("HeartFillClipper"),
                                HeartFillClipper { slot },
                                Node {
                                    position_type: PositionType::Absolute,
                                    top: Val::Px(0.0),
                                    left: Val::Px(0.0),
                                    width: Val::Px(HEART_WIDTH),
                                    height: Val::Px(HEART_HEIGHT),
                                    overflow: Overflow::clip(),
                                    ..default()
                                },
                            ))
                            .with_children(|clipper| {
                                clipper.spawn((
                                    Name::new("HeartFillImage"),
                                    ImageNode {
                                        image: texture.clone(),
                                        ..default()
                                    },
                                    Node {
                                        width: Val::Px(HEART_WIDTH),
                                        height: Val::Px(HEART_HEIGHT),
                                        ..default()
                                    },
                                ));
                            });
                    });
            }
        });
}

fn update_heart_display(
    health: Res<PlayerHealth>,
    mut prev_health: Local<f32>,
    mut clipper_query: Query<(&HeartFillClipper, &mut Node)>,
) {
    let current = health.value;
    if current == *prev_health {
        return;
    }
    *prev_health = current;

    let half_hearts = health_to_half_hearts(current);

    for (clipper, mut node) in &mut clipper_query {
        let i = clipper.slot as u32;
        let full_threshold = (i + 1) * 2;
        let half_threshold = i * 2 + 1;

        node.width = if half_hearts >= full_threshold {
            Val::Px(HEART_WIDTH)
        } else if half_hearts == half_threshold {
            Val::Px(HEART_WIDTH / 2.0)
        } else {
            Val::Px(0.0)
        };
    }
}
