use crate::prelude::*;

// ── Constants ────────────────────────────────────────────────────────────────

const FADE_DURATION_SECS: f32 = 1.5;
/// Time between each revealed character (seconds).
const CHAR_DELAY_SECS: f32 = 0.04;
const TEXT_PADDING: f32 = 100.0;
const TEXT_FONT_SIZE: f32 = 28.0;

const SHOW_END_FADE_SECS: f32 = 2.5;
/// Delay after typing finishes before credits fade in.
const SHOW_END_DELAY_SECS: f32 = 1.2;

const CREDITS_TITLE_SIZE: f32 = 80.0;
const CREDITS_SUBTITLE_SIZE: f32 = 30.0;
/// Gap between title bottom and subtitle top (px).
const CREDITS_GAP: f32 = 18.0;
/// Distance from bottom of screen to subtitle baseline.
const CREDITS_BOTTOM: f32 = TEXT_PADDING;

const TERMINAL_MESSAGE: &str = "\
[SYSTEM] SUCCESS: OBJECTIVE_CRITERIA_MET\n\
\n\
[SYSTEM] EXTRACTING: /logs/sim_1042_final_metrics.dat\n\
\n\
[SYSTEM] STATUS: DATA_VALIDATED_AND_UPLOADED\n\
\n\
[SYSTEM] EXECUTING: /usr/bin/purge_environment\n\
\n\
[SYSTEM] INITIALIZING: /usr/bin/prepare_next_iteration";

// ── Markers ──────────────────────────────────────────────────────────────────

#[derive(Component)]
struct EndOverlay;

#[derive(Component)]
struct EndText;

#[derive(Component)]
struct EndCreditsTitle;

#[derive(Component)]
struct EndCreditsSubtitle;

// ── Resources ────────────────────────────────────────────────────────────────

#[derive(Resource)]
struct EndSequence {
    phase: EndPhase,
}

enum EndPhase {
    FadeToBlack { elapsed: f32 },
    Typing { char_index: usize, char_timer: f32 },
    ShowEnd { elapsed: f32 },
    Done,
}

// ── Plugin ───────────────────────────────────────────────────────────────────

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::End), spawn_end_sequence)
        .add_systems(Update, tick_end_sequence.run_if(in_state(GameState::End)));
}

// ── Systems ──────────────────────────────────────────────────────────────────

fn spawn_end_sequence(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    palette: Res<crate::theme::palette::ColorPalette>,
) {
    // Full-screen black overlay — fades from transparent to opaque.
    commands.spawn((
        Name::new("EndOverlay"),
        EndOverlay,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        ZIndex(1000),
    ));

    // Terminal text — top-left monospace.
    commands.spawn((
        Name::new("EndText"),
        EndText,
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

    let quicksand = asset_server.load("fonts/Quicksand-Regular.ttf");

    // "THE END" — large centered title, starts invisible.
    commands.spawn((
        Name::new("EndCreditsTitle"),
        EndCreditsTitle,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(CREDITS_BOTTOM + CREDITS_SUBTITLE_SIZE + CREDITS_GAP),
            left: Val::Percent(0.0),
            right: Val::Percent(0.0),
            width: Val::Percent(100.0),
            ..default()
        },
        Text::new("THE END"),
        TextLayout::new_with_justify(Justify::Center),
        TextFont {
            font: quicksand.clone().into(),
            font_size: FontSize::Px(CREDITS_TITLE_SIZE),
            ..default()
        },
        TextColor(palette.amber.with_alpha(0.0)),
        ZIndex(1001),
    ));

    // "Thank you for playing!" — smaller subtitle, starts invisible.
    commands.spawn((
        Name::new("EndCreditsSubtitle"),
        EndCreditsSubtitle,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(CREDITS_BOTTOM),
            left: Val::Percent(0.0),
            right: Val::Percent(0.0),
            width: Val::Percent(100.0),
            ..default()
        },
        Text::new("Thank you for playing!"),
        TextLayout::new_with_justify(Justify::Center),
        TextFont {
            font: quicksand.into(),
            font_size: FontSize::Px(CREDITS_SUBTITLE_SIZE),
            ..default()
        },
        TextColor(palette.bone.with_alpha(0.0)),
        ZIndex(1001),
    ));

    commands.insert_resource(EndSequence {
        phase: EndPhase::FadeToBlack { elapsed: 0.0 },
    });
}

fn tick_end_sequence(
    time: Res<Time>,
    mut sequence: ResMut<EndSequence>,
    mut overlay: Query<&mut BackgroundColor, With<EndOverlay>>,
    mut text: Query<&mut Text, With<EndText>>,
    mut title_color: Query<&mut TextColor, (With<EndCreditsTitle>, Without<EndCreditsSubtitle>)>,
    mut subtitle_color: Query<&mut TextColor, (With<EndCreditsSubtitle>, Without<EndCreditsTitle>)>,
) {
    let dt = time.delta_secs();

    match &mut sequence.phase {
        EndPhase::FadeToBlack { elapsed } => {
            *elapsed += dt;
            let alpha = (*elapsed / FADE_DURATION_SECS).clamp(0.0, 1.0);
            if let Ok(mut bg) = overlay.single_mut() {
                bg.0 = Color::srgba(0.0, 0.0, 0.0, alpha);
            }
            if *elapsed >= FADE_DURATION_SECS {
                sequence.phase = EndPhase::Typing {
                    char_index: 0,
                    char_timer: 0.0,
                };
            }
        }
        EndPhase::Typing {
            char_index,
            char_timer,
        } => {
            if *char_index >= TERMINAL_MESSAGE.len() {
                sequence.phase = EndPhase::ShowEnd { elapsed: 0.0 };
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
        EndPhase::ShowEnd { elapsed } => {
            *elapsed += dt;
            let fade_t =
                ((*elapsed - SHOW_END_DELAY_SECS) / SHOW_END_FADE_SECS).clamp(0.0, 1.0);
            if let Ok(mut color) = title_color.single_mut() {
                color.set_if_neq(TextColor(color.0.with_alpha(fade_t)));
            }
            if let Ok(mut color) = subtitle_color.single_mut() {
                color.set_if_neq(TextColor(color.0.with_alpha(fade_t)));
            }
            if *elapsed >= SHOW_END_DELAY_SECS + SHOW_END_FADE_SECS {
                sequence.phase = EndPhase::Done;
            }
        }
        EndPhase::Done => {}
    }
}
