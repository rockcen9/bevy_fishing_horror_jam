use crate::game_manager::backpack::RestartGameEvent;
use crate::prelude::*;

// ── Constants ────────────────────────────────────────────────────────────────

const FADE_DURATION_SECS: f32 = 1.5;
const CHAR_DELAY_SECS: f32 = 0.04;
const TEXT_PADDING: f32 = 100.0;
const TEXT_FONT_SIZE: f32 = 28.0;
// Terminal text color sourced from ColorPalette::terminal_text in spawn_dead_sequence
const TERMINAL_MESSAGE: &str = "[SYSTEM]: Restarting..............................";

// ── Markers ──────────────────────────────────────────────────────────────────

#[derive(Component)]
struct DeadOverlay;

#[derive(Component)]
struct DeadText;

// ── Resources ────────────────────────────────────────────────────────────────

#[derive(Resource)]
struct DeadSequence {
    phase: DeadPhase,
}

enum DeadPhase {
    FadeToBlack { elapsed: f32 },
    Typing { char_index: usize, char_timer: f32 },
    Done,
}

// ── Plugin ───────────────────────────────────────────────────────────────────

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Dead), spawn_dead_sequence)
        .add_systems(Update, tick_dead_sequence.run_if(in_state(GameState::Dead)))
        .add_systems(OnExit(GameState::Dead), despawn_dead_ui);
}

// ── Systems ──────────────────────────────────────────────────────────────────

fn spawn_dead_sequence(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    palette: Res<crate::theme::palette::ColorPalette>,
) {
    commands.spawn((
        Name::new("DeadOverlay"),
        DeadOverlay,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        ZIndex(1000),
    ));

    commands.spawn((
        Name::new("DeadText"),
        DeadText,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(TEXT_PADDING),
            left: Val::Px(TEXT_PADDING),
            ..default()
        },
        Text::new(""),
        TextFont {
            font: asset_server
                .load("fonts/FiraCode-VariableFont_wght.ttf")
                .into(),
            font_size: FontSize::Px(TEXT_FONT_SIZE),
            ..default()
        },
        TextColor(palette.terminal_text),
        ZIndex(1001),
    ));

    commands.insert_resource(DeadSequence {
        phase: DeadPhase::FadeToBlack { elapsed: 0.0 },
    });
}

fn tick_dead_sequence(
    time: Res<Time>,
    mut sequence: ResMut<DeadSequence>,
    mut overlay: Query<&mut BackgroundColor, With<DeadOverlay>>,
    mut text: Query<&mut Text, With<DeadText>>,
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let dt = time.delta_secs();

    match &mut sequence.phase {
        DeadPhase::FadeToBlack { elapsed } => {
            *elapsed += dt;
            let alpha = (*elapsed / FADE_DURATION_SECS).clamp(0.0, 1.0);
            if let Ok(mut bg) = overlay.single_mut() {
                bg.0 = Color::srgba(0.0, 0.0, 0.0, alpha);
            }
            if *elapsed >= FADE_DURATION_SECS {
                sequence.phase = DeadPhase::Typing {
                    char_index: 0,
                    char_timer: 0.0,
                };
            }
        }
        DeadPhase::Typing {
            char_index,
            char_timer,
        } => {
            if *char_index >= TERMINAL_MESSAGE.len() {
                // Typing complete — reset game and return to Idle.
                commands.trigger(RestartGameEvent);
                next_game_state.set(GameState::Idle);
                sequence.phase = DeadPhase::Done;
                return;
            }
            *char_timer += dt;
            while *char_timer >= CHAR_DELAY_SECS && *char_index < TERMINAL_MESSAGE.len() {
                *char_timer -= CHAR_DELAY_SECS;
                *char_index += 1;
                while *char_index < TERMINAL_MESSAGE.len()
                    && !TERMINAL_MESSAGE.is_char_boundary(*char_index)
                {
                    *char_index += 1;
                }
            }
            if let Ok(mut t) = text.single_mut() {
                t.0 = TERMINAL_MESSAGE[..*char_index].to_string();
            }
        }
        DeadPhase::Done => {} // waiting for state transition
    }
}

fn despawn_dead_ui(
    mut commands: Commands,
    overlay: Query<Entity, With<DeadOverlay>>,
    text: Query<Entity, With<DeadText>>,
) {
    for entity in &overlay {
        commands.entity(entity).despawn();
    }
    for entity in &text {
        commands.entity(entity).despawn();
    }
}
