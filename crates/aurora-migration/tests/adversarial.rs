//! Adversarial tests for aurora-migration

use aurora_migration::plan::{MigrationPlan, MigrationStep, StepStatus};
use aurora_migration::runner::{MigrationRunner, RunResult};
use aurora_migration::version::SchemaVersion;

#[test]
fn adversarial_partial_failure_rollback_irreversible() {
    let mut runner = MigrationRunner::new(SchemaVersion::new(1, 0, 0));
    let mut plan = MigrationPlan::new(
        "mixed_reversibility",
        SchemaVersion::new(1, 0, 0),
        SchemaVersion::new(2, 0, 0),
    );
    plan.add_step(MigrationStep::new(
        "add_column",
        "Add new column",
        SchemaVersion::new(1, 1, 0),
    ));
    plan.add_step(
        MigrationStep::new(
            "drop_old_table",
            "Drop legacy table",
            SchemaVersion::new(1, 2, 0),
        )
        .irreversible(),
    );
    plan.add_step(MigrationStep::new(
        "add_index",
        "Add index",
        SchemaVersion::new(1, 3, 0),
    ));
    plan.add_step(MigrationStep::new(
        "migrate_data",
        "Migrate data",
        SchemaVersion::new(2, 0, 0),
    ));

    let result = runner.execute(&mut plan, &[50, 100, 75, 200], Some(3));

    match &result {
        RunResult::Failed {
            step_index,
            step_name,
        } => {
            assert_eq!(*step_index, 3);
            assert_eq!(step_name, "migrate_data");
        }
        _ => panic!("Expected Failed result, got {:?}", result),
    }

    assert_eq!(runner.current_version().to_string(), "1.3.0");
    assert_eq!(runner.steps_executed(), 3);
    assert_eq!(runner.steps_failed(), 1);

    assert_eq!(plan.steps()[0].status, StepStatus::Completed);
    assert_eq!(plan.steps()[1].status, StepStatus::Completed);
    assert_eq!(plan.steps()[2].status, StepStatus::Completed);
    assert_eq!(plan.steps()[3].status, StepStatus::Failed);

    let rolled_back = runner.rollback(&mut plan);
    assert_eq!(rolled_back, 2);
    assert_eq!(runner.current_version().to_string(), "1.0.0");

    assert_eq!(plan.steps()[0].status, StepStatus::RolledBack);
    assert_eq!(plan.steps()[1].status, StepStatus::Completed);
    assert_eq!(plan.steps()[2].status, StepStatus::RolledBack);
    assert_eq!(plan.steps()[3].status, StepStatus::Failed);
}

#[test]
fn adversarial_version_ordering_and_compatibility() {
    let v1 = SchemaVersion::new(1, 0, 0);
    let v2 = SchemaVersion::new(1, 0, 1);
    let v3 = SchemaVersion::new(1, 1, 0);
    let v4 = SchemaVersion::new(2, 0, 0);

    assert!(v2.is_newer_than(&v1));
    assert!(v3.is_newer_than(&v2));
    assert!(v4.is_newer_than(&v3));
    assert!(!v1.is_newer_than(&v1));
    assert!(!v1.is_newer_than(&v4));

    assert!(v1.is_compatible_with(&v2));
    assert!(v1.is_compatible_with(&v3));
    assert!(!v1.is_compatible_with(&v4));

    assert_eq!(v1.bump_patch().to_string(), "1.0.1");
    assert_eq!(v1.bump_minor().to_string(), "1.1.0");
    assert_eq!(v1.bump_major().to_string(), "2.0.0");
}

#[test]
fn adversarial_plan_progress_and_duration() {
    let mut plan = MigrationPlan::new(
        "progress_test",
        SchemaVersion::new(1, 0, 0),
        SchemaVersion::new(1, 3, 0),
    );
    plan.add_step(MigrationStep::new(
        "s1",
        "Step 1",
        SchemaVersion::new(1, 1, 0),
    ));
    plan.add_step(MigrationStep::new(
        "s2",
        "Step 2",
        SchemaVersion::new(1, 2, 0),
    ));
    plan.add_step(MigrationStep::new(
        "s3",
        "Step 3",
        SchemaVersion::new(1, 3, 0),
    ));

    assert!((plan.progress() - 0.0).abs() < 0.01);
    assert!(!plan.is_complete());

    let mut runner = MigrationRunner::new(SchemaVersion::new(1, 0, 0));
    runner.execute(&mut plan, &[100, 250, 50], None);

    assert!((plan.progress() - 1.0).abs() < 0.01);
    assert!(plan.is_complete());
    assert_eq!(plan.total_duration_ms(), 400);
}
