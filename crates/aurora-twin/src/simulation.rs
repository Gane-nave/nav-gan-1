//! Simulation engine — runs what-if scenarios on digital twin state,
//! stepping through discrete time ticks with configurable models.

use aurora_core::types::EntityId;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

// ---------------------------------------------------------------------------
// Simulation types
// ---------------------------------------------------------------------------

/// Status of a simulation run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimulationStatus {
    /// Configured but not yet started.
    Ready,
    /// Currently executing time steps.
    Running,
    /// Paused mid-execution.
    Paused,
    /// Ran to completion.
    Completed,
    /// Terminated early due to error or user cancellation.
    Aborted,
}

/// A single state variable tracked in the simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimVariable {
    pub name: String,
    pub value: f64,
    pub unit: String,
}

/// A recorded observation at one simulation tick.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickObservation {
    pub tick: u64,
    pub sim_time: DateTime<Utc>,
    pub variables: HashMap<String, f64>,
}

/// A rule that modifies variables each tick.
#[derive(Debug, Clone)]
pub struct SimulationRule {
    pub id: EntityId,
    pub name: String,
    /// Variable to modify.
    pub target_variable: String,
    /// Delta to apply per tick.
    pub delta_per_tick: f64,
    /// Minimum value clamp.
    pub min_value: Option<f64>,
    /// Maximum value clamp.
    pub max_value: Option<f64>,
}

/// Configuration for a simulation run.
#[derive(Debug, Clone)]
pub struct SimulationConfig {
    pub name: String,
    /// Duration of each tick (seconds).
    pub tick_duration_s: f64,
    /// Total number of ticks to simulate.
    pub total_ticks: u64,
    /// Initial variable values.
    pub initial_state: HashMap<String, SimVariable>,
    /// Rules that drive the simulation.
    pub rules: Vec<SimulationRule>,
}

/// A completed simulation run with recorded observations.
#[derive(Debug, Clone)]
pub struct SimulationRun {
    pub id: EntityId,
    pub config_name: String,
    pub status: SimulationStatus,
    pub current_tick: u64,
    pub total_ticks: u64,
    pub start_time: DateTime<Utc>,
    pub observations: Vec<TickObservation>,
    pub final_state: HashMap<String, f64>,
    pub completed_at: Option<DateTime<Utc>>,
}

// ---------------------------------------------------------------------------
// Simulation engine
// ---------------------------------------------------------------------------

/// The simulation engine executes discrete-time simulations on digital
/// twin state, applying rules each tick and recording observations.
pub struct SimulationEngine {
    /// Active simulation state.
    current_state: HashMap<String, f64>,
    /// Rules to apply each tick.
    rules: Vec<SimulationRule>,
    /// Completed runs.
    history: Vec<SimulationRun>,
    /// Currently running simulation (if any).
    active_run: Option<SimulationRun>,
    tick_duration_s: f64,
}

impl SimulationEngine {
    pub fn new() -> Self {
        Self {
            current_state: HashMap::new(),
            rules: Vec::new(),
            history: Vec::new(),
            active_run: None,
            tick_duration_s: 1.0,
        }
    }

    /// Start a new simulation run from a configuration.
    pub fn start(&mut self, config: SimulationConfig) -> EntityId {
        let run_id = EntityId::new();

        let initial: HashMap<String, f64> = config
            .initial_state
            .iter()
            .map(|(k, v)| (k.clone(), v.value))
            .collect();

        self.current_state = initial.clone();
        self.rules = config.rules;
        self.tick_duration_s = config.tick_duration_s;

        let run = SimulationRun {
            id: run_id,
            config_name: config.name.clone(),
            status: SimulationStatus::Running,
            current_tick: 0,
            total_ticks: config.total_ticks,
            start_time: Utc::now(),
            observations: vec![TickObservation {
                tick: 0,
                sim_time: Utc::now(),
                variables: initial.clone(),
            }],
            final_state: initial,
            completed_at: None,
        };

        info!(id = %run_id, name = %config.name, ticks = config.total_ticks, "simulation started");
        self.active_run = Some(run);
        run_id
    }

    /// Advance the simulation by one tick. Returns the current tick number,
    /// or None if no simulation is running.
    pub fn step(&mut self) -> Option<u64> {
        let run = self.active_run.as_mut()?;
        if run.status != SimulationStatus::Running {
            return None;
        }

        run.current_tick += 1;
        let tick = run.current_tick;

        // Apply rules.
        for rule in &self.rules {
            let value = self
                .current_state
                .get(&rule.target_variable)
                .copied()
                .unwrap_or(0.0);
            let mut new_val = value + rule.delta_per_tick;
            if let Some(min) = rule.min_value {
                new_val = new_val.max(min);
            }
            if let Some(max) = rule.max_value {
                new_val = new_val.min(max);
            }
            self.current_state
                .insert(rule.target_variable.clone(), new_val);
        }

        // Record observation.
        let obs = TickObservation {
            tick,
            sim_time: run.start_time
                + Duration::seconds((tick as f64 * self.tick_duration_s) as i64),
            variables: self.current_state.clone(),
        };
        run.observations.push(obs);
        run.final_state = self.current_state.clone();

        // Check completion.
        if tick >= run.total_ticks {
            run.status = SimulationStatus::Completed;
            run.completed_at = Some(Utc::now());
            debug!(tick, "simulation completed");
        }

        Some(tick)
    }

    /// Run the simulation to completion (all remaining ticks).
    pub fn run_to_completion(&mut self) -> Option<u64> {
        loop {
            {
                let tick = self.step()?;
                if let Some(run) = &self.active_run {
                    if run.status == SimulationStatus::Completed {
                        return Some(tick);
                    }
                }
            }
        }
    }

    /// Pause the current simulation.
    pub fn pause(&mut self) -> bool {
        if let Some(run) = &mut self.active_run {
            if run.status == SimulationStatus::Running {
                run.status = SimulationStatus::Paused;
                return true;
            }
        }
        false
    }

    /// Resume a paused simulation.
    pub fn resume(&mut self) -> bool {
        if let Some(run) = &mut self.active_run {
            if run.status == SimulationStatus::Paused {
                run.status = SimulationStatus::Running;
                return true;
            }
        }
        false
    }

    /// Abort the current simulation.
    pub fn abort(&mut self) -> bool {
        if let Some(run) = &mut self.active_run {
            run.status = SimulationStatus::Aborted;
            run.completed_at = Some(Utc::now());
            return true;
        }
        false
    }

    /// Finalize the current run and move it to history.
    pub fn finalize(&mut self) -> Option<EntityId> {
        let run = self.active_run.take()?;
        let id = run.id;
        self.history.push(run);
        Some(id)
    }

    /// Get the current state of the simulation.
    pub fn current_state(&self) -> &HashMap<String, f64> {
        &self.current_state
    }

    /// Get the active run.
    pub fn active_run(&self) -> Option<&SimulationRun> {
        self.active_run.as_ref()
    }

    /// Get historical runs.
    pub fn history(&self) -> &[SimulationRun] {
        &self.history
    }

    /// Get a variable's current value.
    pub fn get_variable(&self, name: &str) -> Option<f64> {
        self.current_state.get(name).copied()
    }

    /// Number of completed runs in history.
    pub fn history_count(&self) -> usize {
        self.history.len()
    }
}

impl Default for SimulationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_config(ticks: u64) -> SimulationConfig {
        let mut initial = HashMap::new();
        initial.insert(
            "speed".into(),
            SimVariable {
                name: "speed".into(),
                value: 50.0,
                unit: "km/h".into(),
            },
        );
        initial.insert(
            "fuel".into(),
            SimVariable {
                name: "fuel".into(),
                value: 100.0,
                unit: "liters".into(),
            },
        );

        let rules = vec![
            SimulationRule {
                id: EntityId::new(),
                name: "accelerate".into(),
                target_variable: "speed".into(),
                delta_per_tick: 5.0,
                min_value: Some(0.0),
                max_value: Some(120.0),
            },
            SimulationRule {
                id: EntityId::new(),
                name: "consume_fuel".into(),
                target_variable: "fuel".into(),
                delta_per_tick: -2.0,
                min_value: Some(0.0),
                max_value: None,
            },
        ];

        SimulationConfig {
            name: "test_drive".into(),
            tick_duration_s: 1.0,
            total_ticks: ticks,
            initial_state: initial,
            rules,
        }
    }

    #[test]
    fn start_creates_run() {
        let mut engine = SimulationEngine::new();
        let id = engine.start(simple_config(10));

        let run = engine.active_run().unwrap();
        assert_eq!(run.id, id);
        assert_eq!(run.status, SimulationStatus::Running);
        assert_eq!(run.current_tick, 0);
    }

    #[test]
    fn step_advances_tick() {
        let mut engine = SimulationEngine::new();
        engine.start(simple_config(10));

        let tick = engine.step().unwrap();
        assert_eq!(tick, 1);

        // Speed increased by 5, fuel decreased by 2.
        assert!((engine.get_variable("speed").unwrap() - 55.0).abs() < f64::EPSILON);
        assert!((engine.get_variable("fuel").unwrap() - 98.0).abs() < f64::EPSILON);
    }

    #[test]
    fn run_to_completion() {
        let mut engine = SimulationEngine::new();
        engine.start(simple_config(5));

        let final_tick = engine.run_to_completion().unwrap();
        assert_eq!(final_tick, 5);

        let run = engine.active_run().unwrap();
        assert_eq!(run.status, SimulationStatus::Completed);
        assert_eq!(run.observations.len(), 6); // tick 0 + 5 steps
    }

    #[test]
    fn clamping_works() {
        let mut engine = SimulationEngine::new();
        // Speed starts at 50, +5/tick, max 120 → after 20 ticks should cap at 120.
        engine.start(simple_config(20));
        engine.run_to_completion();

        let speed = engine.get_variable("speed").unwrap();
        assert!((speed - 120.0).abs() < f64::EPSILON);

        // Fuel starts at 100, -2/tick, min 0 → after 50 ticks = 0.
        // After 20 ticks: 100 - 40 = 60.
        let fuel = engine.get_variable("fuel").unwrap();
        assert!((fuel - 60.0).abs() < f64::EPSILON);
    }

    #[test]
    fn pause_and_resume() {
        let mut engine = SimulationEngine::new();
        engine.start(simple_config(10));

        engine.step();
        assert!(engine.pause());
        assert_eq!(
            engine.active_run().unwrap().status,
            SimulationStatus::Paused
        );

        // Step while paused should return None.
        assert!(engine.step().is_none());

        assert!(engine.resume());
        assert_eq!(
            engine.active_run().unwrap().status,
            SimulationStatus::Running
        );
        assert!(engine.step().is_some());
    }

    #[test]
    fn abort_stops_simulation() {
        let mut engine = SimulationEngine::new();
        engine.start(simple_config(10));
        engine.step();

        assert!(engine.abort());
        assert_eq!(
            engine.active_run().unwrap().status,
            SimulationStatus::Aborted
        );
    }

    #[test]
    fn finalize_moves_to_history() {
        let mut engine = SimulationEngine::new();
        engine.start(simple_config(3));
        engine.run_to_completion();

        let id = engine.finalize().unwrap();
        assert!(engine.active_run().is_none());
        assert_eq!(engine.history_count(), 1);
        assert_eq!(engine.history()[0].id, id);
    }

    #[test]
    fn observations_record_all_ticks() {
        let mut engine = SimulationEngine::new();
        engine.start(simple_config(3));
        engine.run_to_completion();

        let run = engine.active_run().unwrap();
        assert_eq!(run.observations.len(), 4); // 0,1,2,3

        // Check tick 0 has initial values.
        assert!((run.observations[0].variables["speed"] - 50.0).abs() < f64::EPSILON);
        // Tick 3: speed = 50 + 3*5 = 65.
        assert!((run.observations[3].variables["speed"] - 65.0).abs() < f64::EPSILON);
    }

    #[test]
    fn no_step_without_active_run() {
        let mut engine = SimulationEngine::new();
        assert!(engine.step().is_none());
    }

    #[test]
    fn fuel_depletes_to_zero() {
        let mut engine = SimulationEngine::new();
        engine.start(simple_config(60)); // 60 ticks × -2 = -120 but clamped at 0
        engine.run_to_completion();

        let fuel = engine.get_variable("fuel").unwrap();
        assert!((fuel - 0.0).abs() < f64::EPSILON);
    }
}
