use crate::prelude::*;
use kira_ext::BGMEvent;

mod attack_animation;
mod charge_arrow;
mod check_escape;
mod shadow;
mod spawn;
mod zone;

const ROAMING_DURATION_SECS: f32 = 5.0;

#[derive(Resource)]
struct RoamingToAttackTimer(Timer);

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.add_sub_state::<MonsterState>();
    app.add_systems(
        OnEnter(MonsterState::Roaming),
        (play_monster_bgm, insert_roaming_timer),
    );
    app.add_systems(OnExit(MonsterState::Roaming), remove_roaming_timer);
    app.add_systems(
        Update,
        tick_roaming_timer.run_if(in_state(MonsterState::Roaming)),
    );

    attack_animation::plugin(app);
    charge_arrow::plugin(app);
    zone::plugin(app);
    spawn::plugin(app);
    check_escape::plugin(app);
    shadow::plugin(app);
}

fn play_monster_bgm(mut commands: Commands) {
    commands.trigger(BGMEvent::new("monster"));
}

fn insert_roaming_timer(mut commands: Commands) {
    commands.insert_resource(RoamingToAttackTimer(Timer::from_seconds(
        ROAMING_DURATION_SECS,
        TimerMode::Once,
    )));
}

fn remove_roaming_timer(mut commands: Commands) {
    commands.remove_resource::<RoamingToAttackTimer>();
}

fn tick_roaming_timer(
    time: Res<Time>,
    mut timer: ResMut<RoamingToAttackTimer>,
    mut next_state: ResMut<NextState<MonsterState>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        next_state.set(MonsterState::PrepareAttack);
    }
}

#[derive(
    SubStates, Debug, Hash, PartialEq, Eq, Clone, Default, Reflect, strum_macros::EnumIter,
)]
#[source(GameState=GameState::Monster)]
pub enum MonsterState {
    #[default]
    Roaming,
    PrepareAttack,
    AttackAnimation,
}
