use crate::prelude::*;
use bevy_tweening::{lens::TransformRotateZLens, *};
use std::time::Duration;

const POLE_LEAN_DURATION_MS: u64 = 300;
/// 30 degrees backward lean.
const POLE_MAX_LEAN_RAD: f32 = std::f32::consts::FRAC_PI_6;

#[derive(Component)]
struct PoleLeanTween;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, update_pole_lean_from_hand_screen.run_if(in_state(GameState::Idle)).run_if(in_state(Pause(false))));
    // app.add_systems(OnExit(GameState::Idle), on_exit_idle);
}

fn update_pole_lean_from_hand_screen(
    hand_screen: Res<RightHandScreenPosition>,
    mut last_state: Local<Option<ScreenHalf>>,
    q_pole: Query<(Entity, &Transform), With<Pole>>,
    mut commands: Commands,
    q_lean_tween: Query<Entity, With<PoleLeanTween>>,
) {
    if *last_state == hand_screen.right_hand {
        return;
    }

    debug!("RightHandScreenPosition state: {:?} → {:?}", *last_state, hand_screen.right_hand);
    *last_state = hand_screen.right_hand;

    let Ok((pole_entity, transform)) = q_pole.single() else {
        return;
    };

    let target = match hand_screen.right_hand {
        Some(ScreenHalf::Top) => -POLE_MAX_LEAN_RAD,
        _ => 0.0,
    };

    let lean_count = q_lean_tween.iter().count();
    debug!(
        "Despawning {} PoleLeanTween(s), current_rot={:.4} rad, target={:.4} rad",
        lean_count,
        transform.rotation.to_euler(EulerRot::XYZ).2,
        target
    );

    for e in q_lean_tween.iter() {
        commands.entity(e).despawn();
    }

    let current = transform.rotation.to_euler(EulerRot::XYZ).2;
    let tween = Tween::new(
        EaseFunction::SineInOut,
        Duration::from_millis(POLE_LEAN_DURATION_MS),
        TransformRotateZLens {
            start: current,
            end: target,
        },
    );

    debug!("Spawning PoleLeanTween: {:.4} → {:.4} rad over {}ms", current, target, POLE_LEAN_DURATION_MS);

    commands.spawn((
        PoleLeanTween,
        TweenAnim::new(tween),
        AnimTarget::component::<Transform>(pole_entity),
    ));
}

// fn on_exit_idle(
//     mut commands: Commands,
//     q_lean_tween: Query<Entity, With<PoleLeanTween>>,
//     mut q_pole: Query<&mut Transform, With<Pole>>,
// ) {
//     for e in q_lean_tween.iter() {
//         commands.entity(e).despawn();
//     }
//     if let Ok(mut transform) = q_pole.single_mut() {
//         transform.rotation = Quat::IDENTITY;
//     }
// }
