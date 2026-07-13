//! Schema migration — transform data between schema versions.

use std::collections::HashMap;

use crate::codec::Value;

/// A migration action for a single field.
#[derive(Debug, Clone)]
pub enum MigrationAction {
    /// Keep the field as-is.
    Keep,
    /// Rename the field (old tag -> new tag).
    Rename { new_tag: u32 },
    /// Remove the field.
    Remove,
    /// Add a new field with a default value.
    AddDefault { tag: u32, value: Value },
    /// Cast a numeric field to a different type.
    Cast { target_tag: u32 },
}

/// A migration plan from one version to another.
#[derive(Debug, Clone)]
pub struct MigrationPlan {
    /// Source schema version.
    pub from_version: u32,
    /// Target schema version.
    pub to_version: u32,
    /// Actions keyed by source field tag.
    actions: HashMap<u32, MigrationAction>,
    /// Fields to add (not present in source).
    additions: Vec<MigrationAction>,
}

impl MigrationPlan {
    /// Create a new migration plan.
    pub fn new(from_version: u32, to_version: u32) -> Self {
        Self {
            from_version,
            to_version,
            actions: HashMap::new(),
            additions: Vec::new(),
        }
    }

    /// Keep a field unchanged.
    pub fn keep(mut self, tag: u32) -> Self {
        self.actions.insert(tag, MigrationAction::Keep);
        self
    }

    /// Rename a field (change its tag).
    pub fn rename(mut self, old_tag: u32, new_tag: u32) -> Self {
        self.actions
            .insert(old_tag, MigrationAction::Rename { new_tag });
        self
    }

    /// Remove a field.
    pub fn remove(mut self, tag: u32) -> Self {
        self.actions.insert(tag, MigrationAction::Remove);
        self
    }

    /// Add a new field with a default value.
    pub fn add_default(mut self, tag: u32, value: Value) -> Self {
        self.additions
            .push(MigrationAction::AddDefault { tag, value });
        self
    }

    /// Get the action for a field tag.
    pub fn action_for(&self, tag: u32) -> Option<&MigrationAction> {
        self.actions.get(&tag)
    }

    /// Apply migration to a set of tagged values.
    /// Input: HashMap<tag, Value>, Output: HashMap<tag, Value>.
    pub fn apply(&self, source: &HashMap<u32, Value>) -> HashMap<u32, Value> {
        let mut result = HashMap::new();

        for (tag, value) in source {
            match self.actions.get(tag) {
                Some(MigrationAction::Keep) | None => {
                    // Default: keep unchanged
                    result.insert(*tag, value.clone());
                }
                Some(MigrationAction::Rename { new_tag }) => {
                    result.insert(*new_tag, value.clone());
                }
                Some(MigrationAction::Remove) => {
                    // Skip this field
                }
                Some(MigrationAction::Cast { target_tag }) => {
                    // Simple numeric casts
                    let casted = cast_value(value);
                    result.insert(*target_tag, casted);
                }
                Some(MigrationAction::AddDefault { .. }) => {
                    // This shouldn't be in the actions map, but handle gracefully
                    result.insert(*tag, value.clone());
                }
            }
        }

        // Apply additions
        for addition in &self.additions {
            if let MigrationAction::AddDefault { tag, value } = addition {
                result.entry(*tag).or_insert_with(|| value.clone());
            }
        }

        result
    }

    /// Number of actions in this plan.
    pub fn action_count(&self) -> usize {
        self.actions.len() + self.additions.len()
    }
}

/// Simple numeric cast (promotes integers to f64).
fn cast_value(value: &Value) -> Value {
    match value {
        Value::U8(v) => Value::F64(*v as f64),
        Value::U16(v) => Value::F64(*v as f64),
        Value::U32(v) => Value::F64(*v as f64),
        Value::U64(v) => Value::F64(*v as f64),
        Value::I32(v) => Value::F64(*v as f64),
        Value::I64(v) => Value::F64(*v as f64),
        Value::F32(v) => Value::F64(*v as f64),
        other => other.clone(),
    }
}

/// Manages migration plans between versions.
pub struct MigrationRegistry {
    /// Plans indexed by (from_version, to_version).
    plans: HashMap<(u32, u32), MigrationPlan>,
}

impl MigrationRegistry {
    /// Create a new migration registry.
    pub fn new() -> Self {
        Self {
            plans: HashMap::new(),
        }
    }

    /// Register a migration plan.
    pub fn register(&mut self, plan: MigrationPlan) {
        self.plans
            .insert((plan.from_version, plan.to_version), plan);
    }

    /// Get a migration plan.
    pub fn get(&self, from: u32, to: u32) -> Option<&MigrationPlan> {
        self.plans.get(&(from, to))
    }

    /// Find a migration path from one version to another (single-hop only).
    pub fn can_migrate(&self, from: u32, to: u32) -> bool {
        self.plans.contains_key(&(from, to))
    }

    /// Apply a migration if available.
    pub fn migrate(
        &self,
        from: u32,
        to: u32,
        data: &HashMap<u32, Value>,
    ) -> Option<HashMap<u32, Value>> {
        let plan = self.get(from, to)?;
        Some(plan.apply(data))
    }

    /// List all available migrations.
    pub fn available_migrations(&self) -> Vec<(u32, u32)> {
        self.plans.keys().copied().collect()
    }
}

impl Default for MigrationRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keep_fields() {
        let plan = MigrationPlan::new(1, 2).keep(1).keep(2);

        let mut source = HashMap::new();
        source.insert(1, Value::F64(32.0));
        source.insert(2, Value::F64(-117.0));

        let result = plan.apply(&source);
        assert_eq!(result.get(&1), Some(&Value::F64(32.0)));
        assert_eq!(result.get(&2), Some(&Value::F64(-117.0)));
    }

    #[test]
    fn test_rename_field() {
        let plan = MigrationPlan::new(1, 2).rename(1, 10);

        let mut source = HashMap::new();
        source.insert(1, Value::String("hello".into()));

        let result = plan.apply(&source);
        assert!(!result.contains_key(&1));
        assert_eq!(result.get(&10), Some(&Value::String("hello".into())));
    }

    #[test]
    fn test_remove_field() {
        let plan = MigrationPlan::new(1, 2).keep(1).remove(2);

        let mut source = HashMap::new();
        source.insert(1, Value::U32(42));
        source.insert(2, Value::String("old".into()));

        let result = plan.apply(&source);
        assert_eq!(result.len(), 1);
        assert!(!result.contains_key(&2));
    }

    #[test]
    fn test_add_default() {
        let plan = MigrationPlan::new(1, 2)
            .keep(1)
            .add_default(3, Value::F32(0.0));

        let mut source = HashMap::new();
        source.insert(1, Value::F64(32.0));

        let result = plan.apply(&source);
        assert_eq!(result.get(&3), Some(&Value::F32(0.0)));
    }

    #[test]
    fn test_add_default_no_overwrite() {
        let plan = MigrationPlan::new(1, 2).add_default(1, Value::F64(0.0));

        let mut source = HashMap::new();
        source.insert(1, Value::F64(99.0));

        let result = plan.apply(&source);
        assert_eq!(result.get(&1), Some(&Value::F64(99.0))); // existing value preserved
    }

    #[test]
    fn test_migration_registry() {
        let mut reg = MigrationRegistry::new();
        let plan = MigrationPlan::new(1, 2)
            .keep(1)
            .add_default(2, Value::Bool(false));
        reg.register(plan);

        assert!(reg.can_migrate(1, 2));
        assert!(!reg.can_migrate(2, 3));

        let mut data = HashMap::new();
        data.insert(1, Value::U32(42));
        let migrated = reg.migrate(1, 2, &data).unwrap();
        assert_eq!(migrated.get(&2), Some(&Value::Bool(false)));
    }

    #[test]
    fn test_complex_migration() {
        let plan = MigrationPlan::new(1, 3)
            .keep(1) // lat stays
            .keep(2) // lon stays
            .remove(3) // remove old altitude
            .rename(4, 5) // rename speed tag
            .add_default(6, Value::String("WGS84".into())); // add datum

        let mut source = HashMap::new();
        source.insert(1, Value::F64(32.0));
        source.insert(2, Value::F64(-117.0));
        source.insert(3, Value::F64(100.0)); // will be removed
        source.insert(4, Value::F64(60.0)); // will be renamed to tag 5

        let result = plan.apply(&source);
        assert_eq!(result.get(&1), Some(&Value::F64(32.0)));
        assert_eq!(result.get(&2), Some(&Value::F64(-117.0)));
        assert!(!result.contains_key(&3));
        assert!(!result.contains_key(&4));
        assert_eq!(result.get(&5), Some(&Value::F64(60.0)));
        assert_eq!(result.get(&6), Some(&Value::String("WGS84".into())));
    }

    #[test]
    fn test_cast_value() {
        let v = cast_value(&Value::U32(42));
        assert_eq!(v, Value::F64(42.0));

        let v = cast_value(&Value::I32(-10));
        assert_eq!(v, Value::F64(-10.0));
    }

    #[test]
    fn test_action_count() {
        let plan = MigrationPlan::new(1, 2)
            .keep(1)
            .rename(2, 3)
            .add_default(4, Value::Bool(true));
        assert_eq!(plan.action_count(), 3);
    }
}
