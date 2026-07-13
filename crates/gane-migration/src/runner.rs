//! Migration runner — executes and manages migration plans.

use crate::plan::{MigrationPlan, StepStatus};
use crate::version::{SchemaVersion, VersionTracker};

/// Result of running a migration plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunResult {
    /// All steps completed successfully.
    Success,
    /// A step failed at the given index.
    Failed {
        step_index: usize,
        step_name: String,
    },
    /// Plan was empty — nothing to do.
    Empty,
}

impl RunResult {
    /// Whether the migration succeeded.
    pub fn is_success(&self) -> bool {
        matches!(self, RunResult::Success)
    }
}

/// Simulates running a migration plan by advancing step statuses.
pub struct MigrationRunner {
    version_tracker: VersionTracker,
    plans_executed: u64,
    steps_executed: u64,
    steps_failed: u64,
}

impl MigrationRunner {
    /// Create a new runner starting at the given version.
    pub fn new(initial_version: SchemaVersion) -> Self {
        Self {
            version_tracker: VersionTracker::new(initial_version),
            plans_executed: 0,
            steps_executed: 0,
            steps_failed: 0,
        }
    }

    /// Get the current schema version.
    pub fn current_version(&self) -> &SchemaVersion {
        self.version_tracker.current()
    }

    /// Execute a plan, simulating each step with the provided durations.
    /// If `fail_at` is Some(index), the step at that index will fail.
    pub fn execute(
        &mut self,
        plan: &mut MigrationPlan,
        step_durations_ms: &[u64],
        fail_at: Option<usize>,
    ) -> RunResult {
        if plan.step_count() == 0 {
            return RunResult::Empty;
        }

        self.plans_executed += 1;

        for i in 0..plan.step_count() {
            let step = plan.step_mut(i).unwrap();
            step.mark_running();

            if Some(i) == fail_at {
                step.mark_failed();
                self.steps_failed += 1;
                return RunResult::Failed {
                    step_index: i,
                    step_name: step.name.clone(),
                };
            }

            let duration = step_durations_ms.get(i).copied().unwrap_or(0);
            step.mark_completed(duration);
            self.steps_executed += 1;

            // Advance version tracker
            let target = step.target_version.clone();
            self.version_tracker.advance(target);
        }

        RunResult::Success
    }

    /// Roll back all completed steps in reverse order.
    pub fn rollback(&mut self, plan: &mut MigrationPlan) -> usize {
        let mut rolled_back = 0;
        let step_count = plan.step_count();

        for i in (0..step_count).rev() {
            let step = plan.step_mut(i).unwrap();
            if step.status == StepStatus::Completed && step.reversible {
                step.mark_rolled_back();
                rolled_back += 1;
            }
        }

        // Roll back version tracker to initial
        if rolled_back > 0 {
            self.version_tracker.rollback_to(0);
        }

        rolled_back
    }

    /// Total plans executed.
    pub fn plans_executed(&self) -> u64 {
        self.plans_executed
    }

    /// Total steps executed successfully.
    pub fn steps_executed(&self) -> u64 {
        self.steps_executed
    }

    /// Total steps failed.
    pub fn steps_failed(&self) -> u64 {
        self.steps_failed
    }

    /// Version history length.
    pub fn version_history_len(&self) -> usize {
        self.version_tracker.history_len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::MigrationStep;

    fn make_plan() -> MigrationPlan {
        let mut plan = MigrationPlan::new(
            "test",
            SchemaVersion::new(1, 0, 0),
            SchemaVersion::new(1, 3, 0),
        );
        plan.add_step(MigrationStep::new(
            "step1",
            "First step",
            SchemaVersion::new(1, 1, 0),
        ));
        plan.add_step(MigrationStep::new(
            "step2",
            "Second step",
            SchemaVersion::new(1, 2, 0),
        ));
        plan.add_step(MigrationStep::new(
            "step3",
            "Third step",
            SchemaVersion::new(1, 3, 0),
        ));
        plan
    }

    #[test]
    fn test_successful_execution() {
        let mut runner = MigrationRunner::new(SchemaVersion::new(1, 0, 0));
        let mut plan = make_plan();
        let result = runner.execute(&mut plan, &[100, 200, 50], None);
        assert_eq!(result, RunResult::Success);
        assert!(result.is_success());
        assert!(plan.is_complete());
        assert_eq!(runner.current_version().to_string(), "1.3.0");
        assert_eq!(runner.steps_executed(), 3);
    }

    #[test]
    fn test_failed_execution() {
        let mut runner = MigrationRunner::new(SchemaVersion::new(1, 0, 0));
        let mut plan = make_plan();
        let result = runner.execute(&mut plan, &[100, 200, 50], Some(1));

        assert!(!result.is_success());
        if let RunResult::Failed {
            step_index,
            step_name,
        } = result
        {
            assert_eq!(step_index, 1);
            assert_eq!(step_name, "step2");
        } else {
            panic!("Expected Failed result");
        }
        assert_eq!(runner.steps_executed(), 1);
        assert_eq!(runner.steps_failed(), 1);
    }

    #[test]
    fn test_empty_plan() {
        let mut runner = MigrationRunner::new(SchemaVersion::new(1, 0, 0));
        let mut plan = MigrationPlan::new(
            "empty",
            SchemaVersion::new(1, 0, 0),
            SchemaVersion::new(1, 0, 0),
        );
        let result = runner.execute(&mut plan, &[], None);
        assert_eq!(result, RunResult::Empty);
    }

    #[test]
    fn test_rollback() {
        let mut runner = MigrationRunner::new(SchemaVersion::new(1, 0, 0));
        let mut plan = make_plan();
        runner.execute(&mut plan, &[100, 200, 50], None);

        let rolled_back = runner.rollback(&mut plan);
        assert_eq!(rolled_back, 3);
        assert_eq!(runner.current_version().to_string(), "1.0.0");
    }

    #[test]
    fn test_rollback_skips_irreversible() {
        let mut runner = MigrationRunner::new(SchemaVersion::new(1, 0, 0));
        let mut plan = MigrationPlan::new(
            "test",
            SchemaVersion::new(1, 0, 0),
            SchemaVersion::new(2, 0, 0),
        );
        plan.add_step(MigrationStep::new(
            "s1",
            "reversible",
            SchemaVersion::new(1, 1, 0),
        ));
        plan.add_step(
            MigrationStep::new("s2", "irreversible", SchemaVersion::new(2, 0, 0)).irreversible(),
        );
        runner.execute(&mut plan, &[100, 200], None);

        let rolled_back = runner.rollback(&mut plan);
        assert_eq!(rolled_back, 1); // only reversible step rolled back
    }

    #[test]
    fn test_runner_stats() {
        let mut runner = MigrationRunner::new(SchemaVersion::new(1, 0, 0));
        let mut plan = make_plan();
        runner.execute(&mut plan, &[100, 200, 50], None);
        assert_eq!(runner.plans_executed(), 1);
        assert_eq!(runner.steps_executed(), 3);
        assert_eq!(runner.steps_failed(), 0);
    }

    #[test]
    fn test_version_advances() {
        let mut runner = MigrationRunner::new(SchemaVersion::new(1, 0, 0));
        let mut plan = make_plan();
        runner.execute(&mut plan, &[10, 20, 30], None);
        assert_eq!(runner.version_history_len(), 4); // initial + 3 steps
    }

    #[test]
    fn test_partial_failure_version() {
        let mut runner = MigrationRunner::new(SchemaVersion::new(1, 0, 0));
        let mut plan = make_plan();
        runner.execute(&mut plan, &[10, 20, 30], Some(2));
        // Only steps 0 and 1 completed, so version should be at 1.2.0
        assert_eq!(runner.current_version().to_string(), "1.2.0");
    }
}
