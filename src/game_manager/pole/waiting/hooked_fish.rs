use crate::game_manager::backpack::{PrefabId, PrefabList};
use crate::game_manager::pole::Bobber;
use crate::prelude::*;

const WAITING_PHASE_DURATION_SECS: f32 = 3.0;

/// Attached to the bobber at the start of `GameState::Biting`, identifying the selected fish.
#[derive(Component)]
pub(crate) struct HookedFish(pub PrefabId);

/// Defines the pools and order in which fish appear.
///
/// Sequence: 3 random → target[0] → 3 random → target[1] → 2 random → target[2] → repeat.
/// (11-item cycle to match the 11 available backpack slots after the journal occupies slot 0)
#[derive(Resource, Reflect)]
pub(crate) struct FishCatchSequence {
    #[reflect(ignore)]
    pub random: Vec<PrefabList>,
    #[reflect(ignore)]
    pub target: Vec<PrefabList>,
    pub(crate) catch_index: usize,
}
impl Default for FishCatchSequence {
    fn default() -> Self {
        Self {
            random: vec![
                PrefabList::BitBass,
                PrefabList::CodeE,
                PrefabList::FinTech,
                PrefabList::SideSwimmer,
                PrefabList::DeepLearn,
                PrefabList::MissEel,
                PrefabList::DepthCarp,
                PrefabList::ReefEr,
                PrefabList::HowitzPerch,
            ],
            target: vec![
                PrefabList::Target1,
                PrefabList::Target2,
                PrefabList::Target3,
            ],
            catch_index: 0,
        }
    }
}

impl FishCatchSequence {
    pub(crate) fn pick_next_fish_prefab_id(&self) -> Option<PrefabId> {
        // 11-item cycle: positions 3, 7, 10 are targets; all others are random.
        let pos = self.catch_index % 11;
        let target_idx = match pos {
            3 => Some(0),
            7 => Some(1),
            10 => Some(2),
            _ => None,
        };
        if let Some(idx) = target_idx {
            self.target
                .get(idx % self.target.len().max(1))
                .map(PrefabList::prefab_id)
        } else {
            self.random
                .get(self.catch_index % self.random.len().max(1))
                .map(PrefabList::prefab_id)
        }
    }
}

#[derive(Resource, Default)]
struct WaitingPhaseTimer {
    elapsed: f32,
}

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<FishCatchSequence>()
        .init_resource::<WaitingPhaseTimer>()
        .add_systems(OnEnter(GameState::Waiting), reset_wait_timer)
        .add_systems(OnEnter(GameState::Biting), insert_hooked_fish)
        .add_systems(
            Update,
            tick_wait_timer_to_biting
                .run_if(in_state(GameState::Waiting))
                .run_if(in_state(Pause(false))),
        );
}

fn reset_wait_timer(mut timer: ResMut<WaitingPhaseTimer>) {
    timer.elapsed = 0.0;
    debug!("WaitingPhaseTimer reset");
}

/// Runs every frame during Waiting. Transitions to Biting after WAITING_PHASE_DURATION_SECS seconds.
fn tick_wait_timer_to_biting(
    time: Res<Time>,
    mut timer: ResMut<WaitingPhaseTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    timer.elapsed += time.delta_secs();
    if timer.elapsed >= WAITING_PHASE_DURATION_SECS {
        debug!(
            "wait timer expired ({:.2}s), transitioning to Biting",
            timer.elapsed
        );
        next_state.set(GameState::Biting);
    }
}

/// Runs on entering Biting (StateTransition schedule). Commands flush before Update,
/// so biting systems always see HookedFish on the bobber.
fn insert_hooked_fish(
    bucket: ResMut<FishCatchSequence>,
    bobber: Query<Entity, With<Bobber>>,
    mut commands: Commands,
) {
    debug!(
        "insert_hooked_fish: random={}, target={}, catch_index={}",
        bucket.random.len(),
        bucket.target.len(),
        bucket.catch_index,
    );

    let Ok(bobber_entity) = bobber.single() else {
        warn!("insert_hooked_fish: no Bobber entity found — HookedFish not inserted");
        return;
    };

    let prefab_id = bucket.pick_next_fish_prefab_id();

    match prefab_id {
        Some(id) => {
            debug!(
                "inserting HookedFish({}) on bobber {:?}",
                id.0, bobber_entity
            );
            commands.entity(bobber_entity).insert(HookedFish(id));
        }
        None => {
            warn!("insert_hooked_fish: FishCatchSequence.random/target empty — HookedFish not inserted");
        }
    }
}
