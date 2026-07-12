//! Feature flag management — create, update, evaluate feature flags with conditions.

use std::collections::HashMap;

/// Feature flag state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlagState {
    /// Flag is enabled for all.
    Enabled,
    /// Flag is disabled for all.
    Disabled,
    /// Flag uses rollout rules.
    Conditional,
}

/// A feature flag.
#[derive(Debug, Clone)]
pub struct FeatureFlag {
    /// Flag identifier.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Description.
    pub description: String,
    /// Current state.
    pub state: FlagState,
    /// Tags for categorization.
    pub tags: Vec<String>,
    /// Creation time (epoch millis).
    pub created_at_ms: u64,
    /// Last modified time (epoch millis).
    pub updated_at_ms: u64,
    /// Owner/team responsible.
    pub owner: String,
    /// Default value when no rules match.
    pub default_value: bool,
    /// Evaluation count.
    evaluations: u64,
    /// True evaluation count.
    true_count: u64,
}

impl FeatureFlag {
    /// Create a new feature flag.
    pub fn new(id: &str, name: &str, now_ms: u64) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: String::new(),
            state: FlagState::Disabled,
            tags: Vec::new(),
            created_at_ms: now_ms,
            updated_at_ms: now_ms,
            owner: String::new(),
            default_value: false,
            evaluations: 0,
            true_count: 0,
        }
    }

    /// Set description.
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    /// Set owner.
    pub fn with_owner(mut self, owner: &str) -> Self {
        self.owner = owner.to_string();
        self
    }

    /// Set tags.
    pub fn with_tags(mut self, tags: &[&str]) -> Self {
        self.tags = tags.iter().map(|t| t.to_string()).collect();
        self
    }

    /// Evaluate the flag for a given context.
    pub fn evaluate(&mut self) -> bool {
        self.evaluations += 1;
        let result = match self.state {
            FlagState::Enabled => true,
            FlagState::Disabled => false,
            FlagState::Conditional => self.default_value,
        };
        if result {
            self.true_count += 1;
        }
        result
    }

    /// Enable the flag.
    pub fn enable(&mut self, now_ms: u64) {
        self.state = FlagState::Enabled;
        self.updated_at_ms = now_ms;
    }

    /// Disable the flag.
    pub fn disable(&mut self, now_ms: u64) {
        self.state = FlagState::Disabled;
        self.updated_at_ms = now_ms;
    }

    /// Set to conditional.
    pub fn set_conditional(&mut self, default_value: bool, now_ms: u64) {
        self.state = FlagState::Conditional;
        self.default_value = default_value;
        self.updated_at_ms = now_ms;
    }

    /// Get evaluation count.
    pub fn evaluation_count(&self) -> u64 {
        self.evaluations
    }

    /// Get true rate (0.0–1.0).
    pub fn true_rate(&self) -> f64 {
        if self.evaluations == 0 {
            return 0.0;
        }
        self.true_count as f64 / self.evaluations as f64
    }
}

/// Feature flag registry — manages all flags.
pub struct FlagRegistry {
    flags: HashMap<String, FeatureFlag>,
}

impl FlagRegistry {
    /// Create a new flag registry.
    pub fn new() -> Self {
        Self {
            flags: HashMap::new(),
        }
    }

    /// Register a new flag.
    pub fn register(&mut self, flag: FeatureFlag) -> Result<(), String> {
        if self.flags.contains_key(&flag.id) {
            return Err(format!("Flag '{}' already exists", flag.id));
        }
        self.flags.insert(flag.id.clone(), flag);
        Ok(())
    }

    /// Get a flag by ID.
    pub fn get(&self, id: &str) -> Option<&FeatureFlag> {
        self.flags.get(id)
    }

    /// Get a mutable flag by ID.
    pub fn get_mut(&mut self, id: &str) -> Option<&mut FeatureFlag> {
        self.flags.get_mut(id)
    }

    /// Evaluate a flag by ID.
    pub fn evaluate(&mut self, id: &str) -> Option<bool> {
        self.flags.get_mut(id).map(|f| f.evaluate())
    }

    /// Check if a flag is enabled (convenience).
    pub fn is_enabled(&mut self, id: &str) -> bool {
        self.evaluate(id).unwrap_or(false)
    }

    /// Remove a flag.
    pub fn remove(&mut self, id: &str) -> Option<FeatureFlag> {
        self.flags.remove(id)
    }

    /// List all flag IDs.
    pub fn flag_ids(&self) -> Vec<&str> {
        self.flags.keys().map(|k| k.as_str()).collect()
    }

    /// Count of registered flags.
    pub fn count(&self) -> usize {
        self.flags.len()
    }

    /// Find flags by tag.
    pub fn find_by_tag(&self, tag: &str) -> Vec<&FeatureFlag> {
        self.flags
            .values()
            .filter(|f| f.tags.iter().any(|t| t == tag))
            .collect()
    }

    /// Find flags by owner.
    pub fn find_by_owner(&self, owner: &str) -> Vec<&FeatureFlag> {
        self.flags.values().filter(|f| f.owner == owner).collect()
    }

    /// Get enabled flags count.
    pub fn enabled_count(&self) -> usize {
        self.flags
            .values()
            .filter(|f| f.state == FlagState::Enabled)
            .count()
    }

    /// Get disabled flags count.
    pub fn disabled_count(&self) -> usize {
        self.flags
            .values()
            .filter(|f| f.state == FlagState::Disabled)
            .count()
    }
}

impl Default for FlagRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flag_lifecycle() {
        let mut flag = FeatureFlag::new("dark_mode", "Dark Mode", 1000);
        assert_eq!(flag.state, FlagState::Disabled);
        assert!(!flag.evaluate());

        flag.enable(2000);
        assert_eq!(flag.state, FlagState::Enabled);
        assert!(flag.evaluate());
        assert_eq!(flag.updated_at_ms, 2000);
    }

    #[test]
    fn test_flag_conditional() {
        let mut flag = FeatureFlag::new("beta", "Beta Feature", 1000);
        flag.set_conditional(true, 2000);

        assert_eq!(flag.state, FlagState::Conditional);
        assert!(flag.evaluate());
    }

    #[test]
    fn test_flag_evaluation_stats() {
        let mut flag = FeatureFlag::new("f1", "Test", 0);
        flag.enable(0);

        for _ in 0..10 {
            flag.evaluate();
        }

        assert_eq!(flag.evaluation_count(), 10);
        assert!((flag.true_rate() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_flag_builder() {
        let flag = FeatureFlag::new("f1", "Test", 0)
            .with_description("A test flag")
            .with_owner("team-nav")
            .with_tags(&["navigation", "beta"]);

        assert_eq!(flag.description, "A test flag");
        assert_eq!(flag.owner, "team-nav");
        assert_eq!(flag.tags.len(), 2);
    }

    #[test]
    fn test_registry_basic() {
        let mut reg = FlagRegistry::new();
        let flag = FeatureFlag::new("f1", "Flag 1", 0);
        reg.register(flag).unwrap();

        assert_eq!(reg.count(), 1);
        assert!(reg.get("f1").is_some());
        assert!(!reg.is_enabled("f1"));
    }

    #[test]
    fn test_registry_duplicate_rejected() {
        let mut reg = FlagRegistry::new();
        reg.register(FeatureFlag::new("f1", "Flag 1", 0)).unwrap();
        let err = reg.register(FeatureFlag::new("f1", "Dupe", 0)).unwrap_err();
        assert!(err.contains("already exists"));
    }

    #[test]
    fn test_registry_evaluate() {
        let mut reg = FlagRegistry::new();
        let mut flag = FeatureFlag::new("f1", "Flag 1", 0);
        flag.enable(0);
        reg.register(flag).unwrap();

        assert!(reg.is_enabled("f1"));
        assert!(!reg.is_enabled("nonexistent"));
    }

    #[test]
    fn test_registry_find_by_tag() {
        let mut reg = FlagRegistry::new();
        reg.register(FeatureFlag::new("f1", "A", 0).with_tags(&["nav"]))
            .unwrap();
        reg.register(FeatureFlag::new("f2", "B", 0).with_tags(&["nav", "beta"]))
            .unwrap();
        reg.register(FeatureFlag::new("f3", "C", 0).with_tags(&["beta"]))
            .unwrap();

        assert_eq!(reg.find_by_tag("nav").len(), 2);
        assert_eq!(reg.find_by_tag("beta").len(), 2);
        assert_eq!(reg.find_by_tag("missing").len(), 0);
    }

    #[test]
    fn test_registry_remove() {
        let mut reg = FlagRegistry::new();
        reg.register(FeatureFlag::new("f1", "Flag 1", 0)).unwrap();
        assert_eq!(reg.count(), 1);
        reg.remove("f1");
        assert_eq!(reg.count(), 0);
    }

    #[test]
    fn test_registry_enabled_disabled_count() {
        let mut reg = FlagRegistry::new();
        let mut f1 = FeatureFlag::new("f1", "A", 0);
        f1.enable(0);
        let f2 = FeatureFlag::new("f2", "B", 0);
        let mut f3 = FeatureFlag::new("f3", "C", 0);
        f3.enable(0);

        reg.register(f1).unwrap();
        reg.register(f2).unwrap();
        reg.register(f3).unwrap();

        assert_eq!(reg.enabled_count(), 2);
        assert_eq!(reg.disabled_count(), 1);
    }
}
