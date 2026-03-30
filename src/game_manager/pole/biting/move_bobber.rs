use crate::prelude::*;

// ── Spring / damping per phase ────────────────────────────────────────────────
const SPRING_K_RESTING: f32 = 60.0;
const SPRING_K_NIBBLING: f32 = 40.0;
const SPRING_K_STRUGGLING: f32 = 25.0;

const DAMPING_RESTING: f32 = 0.88;
const DAMPING_NIBBLING: f32 = 0.82;
const DAMPING_STRUGGLING: f32 = 0.75;

// ── Impulse magnitudes ────────────────────────────────────────────────────────
const IMPULSE_NIBBLING: f32 = 150.0;
const IMPULSE_STRUGGLING: f32 = 400.0;

const IMPULSE_INTERVAL_NIBBLING_SECS: f32 = 0.28;
const IMPULSE_INTERVAL_STRUGGLING_SECS: f32 = 0.13;

// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
pub(super) enum BobberPhase {
    Resting,
    Nibbling,
    Struggling,
}

/// Attached to the Bobber entity while in `GameState::Biting`.
#[derive(Component)]
pub(super) struct BobberSpringMotion {
    anchor: Vec2,
    velocity: Vec2,
    pub(super) phase: BobberPhase,
    phase_timer: f32,
    impulse_timer: f32,
    /// LCG seed — seeded from elapsed time so each session differs.
    seed: u32,
}

impl BobberSpringMotion {
    fn new(anchor: Vec2, seed: u32) -> Self {
        Self {
            anchor,
            velocity: Vec2::ZERO,
            phase: BobberPhase::Resting,
            phase_timer: 1.5,
            impulse_timer: 0.0,
            seed,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Biting), attach_bobber_spring_motion);
    app.add_systems(
        Update,
        tick_bobber_spring_simulation
            .run_if(in_state(GameState::Biting))
            .run_if(in_state(Pause(false))),
    );
    app.add_systems(OnExit(GameState::Biting), detach_bobber_spring_motion);
}

fn attach_bobber_spring_motion(
    mut commands: Commands,
    bobber_query: Query<(Entity, &Transform), With<Bobber>>,
    time: Res<Time>,
) {
    let Some((entity, transform)) = bobber_query.iter().next() else {
        debug!("attach_bobber_spring_motion: no Bobber entity found");
        return;
    };
    let anchor = transform.translation.truncate();
    let seed = (time.elapsed_secs() * 1000.0) as u32;
    debug!("attach_bobber_spring_motion: anchor={anchor:?} seed={seed}");
    commands.entity(entity).insert(BobberSpringMotion::new(anchor, seed));
}

fn tick_bobber_spring_simulation(
    mut query: Query<(&mut Transform, &mut BobberSpringMotion), With<Bobber>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let Some((mut transform, mut motion)) = query.iter_mut().next() else {
        debug!("tick_bobber_spring_simulation: no Bobber with BobberSpringMotion found");
        return;
    };

    // ── Phase timer ───────────────────────────────────────────────────────────
    motion.phase_timer -= dt;
    if motion.phase_timer <= 0.0 {
        advance_bobber_phase(&mut motion);
        // Settle at current position when resting — don't spring back to origin.
        if motion.phase == BobberPhase::Resting {
            motion.anchor = transform.translation.truncate();
        }
    }

    // ── Impulses + phase params ───────────────────────────────────────────────
    let (spring_k, damping) = match motion.phase {
        BobberPhase::Resting => (SPRING_K_RESTING, DAMPING_RESTING),

        BobberPhase::Nibbling => {
            motion.impulse_timer -= dt;
            if motion.impulse_timer <= 0.0 {
                let (s1, dx) = lcg_f32(motion.seed);
                let (s2, dy) = lcg_f32(s1);
                motion.seed = s2;
                motion.velocity += Vec2::new(dx, dy) * IMPULSE_NIBBLING;
                motion.impulse_timer = IMPULSE_INTERVAL_NIBBLING_SECS;
            }
            (SPRING_K_NIBBLING, DAMPING_NIBBLING)
        }

        BobberPhase::Struggling => {
            motion.impulse_timer -= dt;
            if motion.impulse_timer <= 0.0 {
                let (s1, dx) = lcg_f32(motion.seed);
                let (s2, dy) = lcg_f32(s1);
                motion.seed = s2;
                // Bias downward — fish pull the bobber under.
                let dir = Vec2::new(dx, dy - 0.5).normalize_or_zero();
                motion.velocity += dir * IMPULSE_STRUGGLING;
                motion.impulse_timer = IMPULSE_INTERVAL_STRUGGLING_SECS;
            }
            (SPRING_K_STRUGGLING, DAMPING_STRUGGLING)
        }
    };

    // ── Spring → damping → integrate ─────────────────────────────────────────
    let displacement = motion.anchor - transform.translation.truncate();
    let new_velocity = (motion.velocity + displacement * spring_k * dt) * damping;
    motion.velocity = new_velocity;
    let new_pos = transform.translation.truncate() + new_velocity * dt;
    transform.translation = new_pos.extend(transform.translation.z);
}

fn detach_bobber_spring_motion(
    mut commands: Commands,
    mut query: Query<(Entity, &BobberSpringMotion), With<Bobber>>,
) {
    let Some((entity, _motion)) = query.iter_mut().next() else { return };
    commands.entity(entity).remove::<BobberSpringMotion>();
}

/// Weighted Markov transition between phases.
fn advance_bobber_phase(motion: &mut BobberSpringMotion) {
    let prev = match motion.phase {
        BobberPhase::Resting => "Resting",
        BobberPhase::Nibbling => "Nibbling",
        BobberPhase::Struggling => "Struggling",
    };

    let (s1, p) = lcg_01(motion.seed);
    let (s2, dur_p) = lcg_01(s1);
    motion.seed = s2;

    motion.phase = match motion.phase {
        BobberPhase::Resting => {
            if p < 0.60 { BobberPhase::Nibbling } else { BobberPhase::Resting }
        }
        BobberPhase::Nibbling => {
            if p < 0.45 { BobberPhase::Resting }
            else if p < 0.75 { BobberPhase::Struggling }
            else { BobberPhase::Nibbling }
        }
        BobberPhase::Struggling => {
            if p < 0.65 { BobberPhase::Resting } else { BobberPhase::Nibbling }
        }
    };

    motion.phase_timer = match motion.phase {
        BobberPhase::Resting    => 1.0 + dur_p * 1.5, // 1.0 – 2.5 s
        BobberPhase::Nibbling   => 0.5 + dur_p * 0.8, // 0.5 – 1.3 s
        BobberPhase::Struggling => 0.3 + dur_p * 0.7, // 0.3 – 1.0 s
    };

    motion.impulse_timer = 0.0; // fire first impulse immediately on entry

    let next = match motion.phase {
        BobberPhase::Resting => "Resting",
        BobberPhase::Nibbling => "Nibbling",
        BobberPhase::Struggling => "Struggling",
    };
    debug!("phase transition: {prev} → {next} (timer={:.2}s)", motion.phase_timer);
}

// ── Minimal LCG — no external crate ──────────────────────────────────────────

/// Returns `(next_seed, value_in_[-1,1])`.
fn lcg_f32(seed: u32) -> (u32, f32) {
    let next = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
    let value = (next as f32 / u32::MAX as f32) * 2.0 - 1.0;
    (next, value)
}

/// Returns `(next_seed, value_in_[0,1])`.
fn lcg_01(seed: u32) -> (u32, f32) {
    let (next, v) = lcg_f32(seed);
    (next, (v + 1.0) * 0.5)
}
