use crate::prelude::*;

const POST_HOOK_RETURN_DELAY_SECS: f32 = 1.0;

#[derive(Resource)]
struct PostHookReturnTimer(Timer);

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Succeeding), insert_post_hook_timer);
    app.add_systems(OnEnter(GameState::Failing), insert_post_hook_timer);
    app.add_systems(
        Update,
        tick_post_hook_timer_to_idle
            .run_if(in_state(GameState::Succeeding).or_else(in_state(GameState::Failing))),
    );
}

fn insert_post_hook_timer(mut commands: Commands) {
    commands.insert_resource(PostHookReturnTimer(Timer::from_seconds(
        POST_HOOK_RETURN_DELAY_SECS,
        TimerMode::Once,
    )));
}

fn tick_post_hook_timer_to_idle(
    time: Res<Time>,
    mut timer: ResMut<PostHookReturnTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        next_state.set(GameState::Idle);
    }
}
