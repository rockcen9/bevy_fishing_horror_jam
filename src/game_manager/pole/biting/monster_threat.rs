use crate::prelude::*;

/// Tracks how provoked the monster is based on QTE outcomes.
#[derive(Resource, Debug, Default)]
pub(crate) struct MonsterThreat {
    pub success_count: u32,
    pub fail_count: u32,
    /// Accumulated threat level [0.0, 1.0].
    /// +0.2 on QTE success, +0.4 on failure, capped at 1.0.
    pub level: f32,
    /// LCG seed for the dice roll.
    seed: u32,
}

impl MonsterThreat {
    /// Record outcome, update threat level, then roll.
    /// Returns `true` if the dice hits and the monster should start roaming.
    pub(crate) fn record_and_roll(&mut self, success: bool, elapsed: f32) -> bool {
        if success {
            self.success_count += 1;
            self.level = (self.level + 0.2).min(1.0);
        } else {
            self.fail_count += 1;
            self.level = (self.level + 0.5).min(1.0);
        }

        // LCG seeded with elapsed time for entropy.
        self.seed = self
            .seed
            .wrapping_add((elapsed * 1_000_000.0) as u32)
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        let roll = self.seed as f32 / u32::MAX as f32;
        roll < self.level
    }

    pub(crate) fn reset(&mut self) {
        self.level = 0.0;
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MonsterThreat>();
    app.add_systems(OnExit(GameState::Monster), on_exit_roaming_reset_threat);
}

fn on_exit_roaming_reset_threat(mut threat: ResMut<MonsterThreat>) {
    threat.reset();
}
