//! Migration plan — defines a sequence of migration steps.

use crate::version::SchemaVersion;

/// Status of a migration step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepStatus {
    /// Not yet executed.
    Pending,
    /// Currently running.
    Running,
    /// Completed successfully.
    Completed,
    /// Failed with error.
    Failed,
    /// Rolled back after failure.
    RolledBack,
}

impl StepStatus {
    /// Display name.
    pub fn as_str(&self) -> &'static str {
        match self {
            StepStatus::Pending => "pending",
            StepStatus::Running => "running",
            StepStatus::Completed => "completed",
            StepStatus::Failed => "failed",
            StepStatus::RolledBack => "rolled_back",
        }
    }

    /// Whether the step has finished (success or failure).
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            StepStatus::Completed | StepStatus::Failed | StepStatus::RolledBack
        )
    }
}

/// A single migration step.
#[derive(Debug, Clone)]
pub struct MigrationStep {
    /// Step name.
    pub name: String,
    /// Description of what this step does.
    pub description: String,
    /// Target version after this step.
    pub target_version: SchemaVersion,
    /// Whether this step is reversible.
    pub reversible: bool,
    /// Current status.
    pub status: StepStatus,
    /// Duration in milliseconds (set after execution).
    pub duration_ms: Option<u64>,
}

impl MigrationStep {
    /// Create a new migration step.
    pub fn new(name: &str, description: &str, target_version: SchemaVersion) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            target_version,
            reversible: true,
            status: StepStatus::Pending,
            duration_ms: None,
        }
    }

    /// Mark as irreversible.
    pub fn irreversible(mut self) -> Self {
        self.reversible = false;
        self
    }

    /// Mark step as running.
    pub fn mark_running(&mut self) {
        self.status = StepStatus::Running;
    }

    /// Mark step as completed.
    pub fn mark_completed(&mut self, duration_ms: u64) {
        self.status = StepStatus::Completed;
        self.duration_ms = Some(duration_ms);
    }

    /// Mark step as failed.
    pub fn mark_failed(&mut self) {
        self.status = StepStatus::Failed;
    }

    /// Mark step as rolled back.
    pub fn mark_rolled_back(&mut self) {
        self.status = StepStatus::RolledBack;
    }
}

/// A migration plan consisting of ordered steps.
pub struct MigrationPlan {
    /// Plan name.
    pub name: String,
    /// Source version.
    pub from_version: SchemaVersion,
    /// Target version.
    pub to_version: SchemaVersion,
    /// Ordered steps.
    steps: Vec<MigrationStep>,
}

impl MigrationPlan {
    /// Create a new migration plan.
    pub fn new(name: &str, from: SchemaVersion, to: SchemaVersion) -> Self {
        Self {
            name: name.to_string(),
            from_version: from,
            to_version: to,
            steps: Vec::new(),
        }
    }

    /// Add a step to the plan.
    pub fn add_step(&mut self, step: MigrationStep) {
        self.steps.push(step);
    }

    /// Get all steps.
    pub fn steps(&self) -> &[MigrationStep] {
        &self.steps
    }

    /// Get a mutable step by index.
    pub fn step_mut(&mut self, index: usize) -> Option<&mut MigrationStep> {
        self.steps.get_mut(index)
    }

    /// Number of steps.
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Number of completed steps.
    pub fn completed_count(&self) -> usize {
        self.steps
            .iter()
            .filter(|s| s.status == StepStatus::Completed)
            .count()
    }

    /// Number of pending steps.
    pub fn pending_count(&self) -> usize {
        self.steps
            .iter()
            .filter(|s| s.status == StepStatus::Pending)
            .count()
    }

    /// Whether all steps are completed.
    pub fn is_complete(&self) -> bool {
        !self.steps.is_empty() && self.steps.iter().all(|s| s.status == StepStatus::Completed)
    }

    /// Whether any step has failed.
    pub fn has_failures(&self) -> bool {
        self.steps.iter().any(|s| s.status == StepStatus::Failed)
    }

    /// Total duration of all completed steps.
    pub fn total_duration_ms(&self) -> u64 {
        self.steps.iter().filter_map(|s| s.duration_ms).sum()
    }

    /// Progress as a ratio (0.0 to 1.0).
    pub fn progress(&self) -> f64 {
        if self.steps.is_empty() {
            return 0.0;
        }
        self.completed_count() as f64 / self.steps.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_plan() -> MigrationPlan {
        let mut plan = MigrationPlan::new(
            "upgrade",
            SchemaVersion::new(1, 0, 0),
            SchemaVersion::new(2, 0, 0),
        );
        plan.add_step(MigrationStep::new(
            "add_column",
            "Add new column",
            SchemaVersion::new(1, 1, 0),
        ));
        plan.add_step(MigrationStep::new(
            "migrate_data",
            "Transform data",
            SchemaVersion::new(1, 2, 0),
        ));
        plan.add_step(
            MigrationStep::new("drop_old", "Drop old column", SchemaVersion::new(2, 0, 0))
                .irreversible(),
        );
        plan
    }

    #[test]
    fn test_plan_creation() {
        let plan = sample_plan();
        assert_eq!(plan.step_count(), 3);
        assert_eq!(plan.pending_count(), 3);
        assert_eq!(plan.completed_count(), 0);
        assert!(!plan.is_complete());
    }

    #[test]
    fn test_step_lifecycle() {
        let mut step = MigrationStep::new("s1", "desc", SchemaVersion::new(1, 0, 0));
        assert_eq!(step.status, StepStatus::Pending);
        assert!(!step.status.is_terminal());

        step.mark_running();
        assert_eq!(step.status, StepStatus::Running);

        step.mark_completed(150);
        assert_eq!(step.status, StepStatus::Completed);
        assert!(step.status.is_terminal());
        assert_eq!(step.duration_ms, Some(150));
    }

    #[test]
    fn test_step_failure_and_rollback() {
        let mut step = MigrationStep::new("s1", "desc", SchemaVersion::new(1, 0, 0));
        step.mark_running();
        step.mark_failed();
        assert_eq!(step.status, StepStatus::Failed);
        assert!(step.status.is_terminal());

        step.mark_rolled_back();
        assert_eq!(step.status, StepStatus::RolledBack);
    }

    #[test]
    fn test_plan_progress() {
        let mut plan = sample_plan();
        assert!((plan.progress() - 0.0).abs() < f64::EPSILON);

        plan.step_mut(0).unwrap().mark_completed(100);
        let expected = 1.0 / 3.0;
        assert!((plan.progress() - expected).abs() < 0.01);

        plan.step_mut(1).unwrap().mark_completed(200);
        plan.step_mut(2).unwrap().mark_completed(50);
        assert!((plan.progress() - 1.0).abs() < f64::EPSILON);
        assert!(plan.is_complete());
    }

    #[test]
    fn test_plan_total_duration() {
        let mut plan = sample_plan();
        plan.step_mut(0).unwrap().mark_completed(100);
        plan.step_mut(1).unwrap().mark_completed(200);
        assert_eq!(plan.total_duration_ms(), 300);
    }

    #[test]
    fn test_plan_has_failures() {
        let mut plan = sample_plan();
        assert!(!plan.has_failures());
        plan.step_mut(1).unwrap().mark_failed();
        assert!(plan.has_failures());
    }

    #[test]
    fn test_irreversible_step() {
        let plan = sample_plan();
        assert!(plan.steps()[0].reversible);
        assert!(!plan.steps()[2].reversible);
    }

    #[test]
    fn test_step_status_display() {
        assert_eq!(StepStatus::Pending.as_str(), "pending");
        assert_eq!(StepStatus::Completed.as_str(), "completed");
        assert_eq!(StepStatus::RolledBack.as_str(), "rolled_back");
    }

    #[test]
    fn test_empty_plan() {
        let plan = MigrationPlan::new(
            "empty",
            SchemaVersion::new(1, 0, 0),
            SchemaVersion::new(1, 0, 0),
        );
        assert!(!plan.is_complete());
        assert!((plan.progress() - 0.0).abs() < f64::EPSILON);
    }
}
