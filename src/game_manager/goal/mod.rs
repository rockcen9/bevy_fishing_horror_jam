use crate::game_manager::backpack::FishCaughtEvent;
use crate::prelude::*;
use bevy::ecs::system::command;
use bevy_tweening::{Delay, lens::TextColorLens, *};
use kira_ext::SFXEvent;
use std::time::Duration;

const DISPLAY_SECS: f32 = 2.0;
const FADE_SECS: f32 = 0.5;
const FONT_SIZE: f32 = 40.0;

#[derive(Component)]
struct GoalToast {
    despawn_timer: Timer,
}

pub(crate) fn plugin(app: &mut App) {
    app.add_observer(on_target_caught);
    app.add_systems(Update, tick_goal_toasts);
}

fn on_target_caught(
    trigger: On<FishCaughtEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    palette: Res<crate::theme::palette::ColorPalette>,
) {
    let id = trigger.event().prefab_id.as_str();
    let (num, bar) = match id {
        "target1" => (1u8, "[█░░]"),
        "target2" => (2u8, "[██░]"),
        "target3" => (3u8, "[███]"),
        _ => return,
    };

    commands.trigger(SFXEvent::sfx("heart_beat"));
    let text = format!("Progress: {} {}/3 Logs Restored", bar, num);
    let start_color = palette.ivory;
    let end_color = start_color.with_alpha(0.0);

    let toast_entity = commands
        .spawn((
            Name::new("GoalToast"),
            GoalToast {
                despawn_timer: Timer::from_seconds(DISPLAY_SECS + FADE_SECS, TimerMode::Once),
            },
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                top: Val::Percent(45.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            Text::new(text),
            TextFont {
                font: asset_server
                    .load("fonts/FiraCode-VariableFont_wght.ttf")
                    .into(),
                font_size: FontSize::Px(FONT_SIZE),
                ..default()
            },
            TextColor(start_color),
            TextLayout::new_with_justify(Justify::Center),
            ZIndex(500),
            Pickable::IGNORE,
        ))
        .id();

    let delay = Delay::new(Duration::from_secs_f32(DISPLAY_SECS));
    let fade = Tween::new(
        EaseFunction::Linear,
        Duration::from_secs_f32(FADE_SECS),
        TextColorLens {
            start: start_color,
            end: end_color,
        },
    );
    commands.spawn((
        Name::new("GoalToastFade"),
        TweenAnim::new(delay.then(fade)),
        AnimTarget::component::<TextColor>(toast_entity),
    ));
}

fn tick_goal_toasts(
    mut commands: Commands,
    time: Res<Time>,
    mut toasts: Query<(Entity, &mut GoalToast)>,
) {
    for (entity, mut toast) in &mut toasts {
        if toast.despawn_timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
