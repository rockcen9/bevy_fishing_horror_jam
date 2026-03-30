use bevy::{state::state::FreelyMutableState, prelude::*, ui::Val::*};

const COLOR_DEFAULT: Color = Color::srgba(0.15, 0.15, 0.15, 0.9);
const COLOR_ACTIVE: Color = Color::srgba(0.2, 0.6, 0.2, 1.0);
const PANEL_WIDTH: f32 = 140.0;
const PANEL_GAP: f32 = 8.0;
const PANEL_TOP: f32 = 8.0;

/// Marker for all debug panel root nodes.
/// Add this to any custom panel so `toggle_visibility` can find it.
#[derive(Component)]
pub struct StateDebugPanel;

/// Tracks how many panels have been registered so each one can auto-position itself.
/// Call `next_panel_right` to claim the next slot.
#[derive(Resource, Default)]
struct PanelRegistry {
    count: usize,
}

/// Claims the next horizontal slot in the debug panel strip and returns its `right` offset.
/// Call this in `plugin()` before registering the spawn system, then pass the value to the system.
pub fn next_panel_right(app: &mut App) -> f32 {
    let mut registry = app
        .world_mut()
        .get_resource_or_insert_with(PanelRegistry::default);
    let i = registry.count;
    registry.count += 1;
    PANEL_GAP + i as f32 * (PANEL_WIDTH + PANEL_GAP)
}

/// Holds the state variant associated with a button.
#[derive(Component)]
struct StateButton<S: States + FreelyMutableState + Clone + PartialEq + Send + Sync + 'static>(S);

/// Panel configuration stored as a resource so the spawn system can read it.
#[derive(Resource)]
struct PanelConfig<S: States + FreelyMutableState + Clone + PartialEq + Send + Sync + 'static> {
    label: String,
    variants: Vec<(String, S)>,
    right: f32,
}

fn type_label<S>() -> String {
    std::any::type_name::<S>()
        .split("::")
        .last()
        .unwrap_or("State")
        .to_string()
}

/// Plugin that spawns a debug panel with buttons to switch between state variants.
/// Panels auto-append left-to-right based on registration order — no manual positioning needed.
///
/// # Example
/// ```ignore
/// app.add_plugins(StateDebugPanelPlugin::<Screen>::all("Screen"));
/// app.add_plugins(StateDebugPanelPlugin::new("Pause", [("On", Pause(true)), ("Off", Pause(false))]));
/// ```
pub struct StateDebugPanelPlugin<S: States + FreelyMutableState + Clone + PartialEq + Send + Sync + 'static> {
    label: String,
    variants: Vec<(String, S)>,
}

impl<S: States + FreelyMutableState + Clone + PartialEq + Send + Sync + 'static> StateDebugPanelPlugin<S> {
    /// Create a panel with an explicit list of variants, labelled via `Debug`.
    pub fn new(variants: impl IntoIterator<Item = S>) -> Self
    where
        S: std::fmt::Debug,
    {
        Self {
            label: type_label::<S>(),
            variants: variants.into_iter().map(|s| (format!("{s:?}"), s)).collect(),
        }
    }

    /// Automatically enumerate all variants using `EnumIter`, labelled via `Debug`.
    pub fn all() -> Self
    where
        S: strum::IntoEnumIterator + std::fmt::Debug,
    {
        Self {
            label: type_label::<S>(),
            variants: S::iter().map(|s| (format!("{s:?}"), s)).collect(),
        }
    }
}

impl<S: States + FreelyMutableState + Clone + PartialEq + Send + Sync + 'static> Plugin for StateDebugPanelPlugin<S> {
    fn build(&self, app: &mut App) {
        let right = next_panel_right(app);
        app.insert_resource(PanelConfig::<S> {
            label: self.label.clone(),
            variants: self.variants.clone(),
            right,
        });
        app.add_systems(Startup, spawn_panel::<S>);
        app.add_systems(Update, handle_button_press::<S>);
        app.add_systems(Update, highlight_active_button::<S>);
    }
}

fn spawn_panel<S: States + FreelyMutableState + Clone + PartialEq + Send + Sync + 'static>(
    mut commands: Commands,
    config: Res<PanelConfig<S>>,
) {
    commands
        .spawn((
            Name::new(format!("{} Debug Panel", config.label)),
            StateDebugPanel,
            Node {
                position_type: PositionType::Absolute,
                top: Px(PANEL_TOP),
                right: Px(config.right),
                width: Px(PANEL_WIDTH),
                flex_direction: FlexDirection::Column,
                row_gap: Px(4.0),
                padding: UiRect::all(Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text(config.label.clone()),
                TextFont {
                    font_size: FontSize::Px(12.0),
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                TextLayout::new_with_justify(Justify::Center),
            ));
            for (label, state) in &config.variants {
                parent.spawn(state_button(label, state.clone()));
            }
        });
}

fn state_button<S: States + FreelyMutableState + Clone + PartialEq + Send + Sync + 'static>(
    label: &str,
    state: S,
) -> impl Bundle {
    let label = label.to_string();
    (
        Name::new(label.clone()),
        Button,
        StateButton(state),
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
            Text(label),
            TextFont {
                font_size: FontSize::Px(14.0),
                ..default()
            },
            TextColor(Color::WHITE),
            Pickable::IGNORE,
        )],
    )
}

fn handle_button_press<S: States + FreelyMutableState + Clone + PartialEq + Send + Sync + 'static>(
    query: Query<(&Interaction, &StateButton<S>), Changed<Interaction>>,
    next_state: Option<ResMut<NextState<S>>>,
) {
    let Some(mut next_state) = next_state else {
        return;
    };
    for (interaction, btn) in &query {
        if matches!(interaction, Interaction::Pressed) {
            next_state.set(btn.0.clone());
        }
    }
}

fn highlight_active_button<S: States + FreelyMutableState + Clone + PartialEq + Send + Sync + 'static>(
    current: Option<Res<State<S>>>,
    mut buttons: Query<(&StateButton<S>, &mut BackgroundColor)>,
    mut prev: Local<Option<S>>,
) {
    let Some(current) = current else { return };
    let current_val = current.get().clone();

    if *prev == Some(current_val.clone()) {
        return;
    }
    *prev = Some(current_val.clone());

    for (btn, mut bg) in &mut buttons {
        bg.set_if_neq(BackgroundColor(if btn.0 == current_val {
            COLOR_ACTIVE
        } else {
            COLOR_DEFAULT
        }));
    }
}
