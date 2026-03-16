//! Feature flags — dynamic feature toggles with targeting and override support.

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Feature flag state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlagState {
    /// Feature is enabled for all users.
    Enabled,
    /// Feature is disabled for all users.
    Disabled,
    /// Feature is conditionally enabled based on targeting rules.
    Conditional,
}

/// A feature flag definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlag {
    pub name: String,
    pub description: String,
    pub state: FlagState,
    pub default_value: bool,
    pub targeting_rules: Vec<TargetingRule>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub owner: String,
    pub tags: Vec<String>,
}

/// A targeting rule for conditional feature flags.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetingRule {
    pub attribute: String,
    pub operator: TargetingOperator,
    pub value: String,
    pub result: bool,
}

/// Targeting rule operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetingOperator {
    Equals,
    NotEquals,
    Contains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
    In,
}

impl TargetingRule {
    /// Evaluate this rule against a context.
    pub fn evaluate(&self, context: &EvaluationContext) -> Option<bool> {
        let attr_value = context.attributes.get(&self.attribute)?;

        let matches = match self.operator {
            TargetingOperator::Equals => attr_value == &self.value,
            TargetingOperator::NotEquals => attr_value != &self.value,
            TargetingOperator::Contains => attr_value.contains(&self.value),
            TargetingOperator::StartsWith => attr_value.starts_with(&self.value),
            TargetingOperator::EndsWith => attr_value.ends_with(&self.value),
            TargetingOperator::GreaterThan => {
                attr_value.parse::<f64>().ok()? > self.value.parse::<f64>().ok()?
            }
            TargetingOperator::LessThan => {
                attr_value.parse::<f64>().ok()? < self.value.parse::<f64>().ok()?
            }
            TargetingOperator::In => self.value.split(',').any(|v| v.trim() == attr_value),
        };

        if matches {
            Some(self.result)
        } else {
            None
        }
    }
}

/// Context for evaluating feature flags.
#[derive(Debug, Clone, Default)]
pub struct EvaluationContext {
    pub user_id: Option<String>,
    pub attributes: HashMap<String, String>,
}

impl EvaluationContext {
    /// Create a new evaluation context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the user ID.
    pub fn with_user(mut self, user_id: &str) -> Self {
        self.user_id = Some(user_id.to_string());
        self.attributes
            .insert("user_id".to_string(), user_id.to_string());
        self
    }

    /// Add an attribute.
    pub fn with_attribute(mut self, key: &str, value: &str) -> Self {
        self.attributes.insert(key.to_string(), value.to_string());
        self
    }
}

/// Feature flag manager — stores and evaluates feature flags.
pub struct FlagManager {
    flags: RwLock<HashMap<String, FeatureFlag>>,
    overrides: RwLock<HashMap<String, HashMap<String, bool>>>, // flag_name -> user_id -> value
    evaluation_count: RwLock<u64>,
}

impl FlagManager {
    /// Create a new flag manager.
    pub fn new() -> Self {
        Self {
            flags: RwLock::new(HashMap::new()),
            overrides: RwLock::new(HashMap::new()),
            evaluation_count: RwLock::new(0),
        }
    }

    /// Register a feature flag.
    pub fn register(&self, flag: FeatureFlag) {
        self.flags.write().insert(flag.name.clone(), flag);
    }

    /// Create a simple enabled/disabled flag.
    pub fn create_flag(&self, name: &str, description: &str, enabled: bool) {
        let now = chrono::Utc::now();
        self.register(FeatureFlag {
            name: name.to_string(),
            description: description.to_string(),
            state: if enabled {
                FlagState::Enabled
            } else {
                FlagState::Disabled
            },
            default_value: enabled,
            targeting_rules: Vec::new(),
            created_at: now,
            updated_at: now,
            owner: String::new(),
            tags: Vec::new(),
        });
    }

    /// Evaluate a feature flag for a given context.
    pub fn is_enabled(&self, flag_name: &str, context: &EvaluationContext) -> bool {
        *self.evaluation_count.write() += 1;

        // Check user-level override first
        if let Some(user_id) = &context.user_id {
            if let Some(user_overrides) = self.overrides.read().get(flag_name) {
                if let Some(value) = user_overrides.get(user_id) {
                    return *value;
                }
            }
        }

        let flags = self.flags.read();
        let flag = match flags.get(flag_name) {
            Some(f) => f,
            None => return false,
        };

        match flag.state {
            FlagState::Enabled => true,
            FlagState::Disabled => false,
            FlagState::Conditional => {
                // Evaluate targeting rules — first match wins
                for rule in &flag.targeting_rules {
                    if let Some(result) = rule.evaluate(context) {
                        return result;
                    }
                }
                flag.default_value
            }
        }
    }

    /// Set a user-level override for a flag.
    pub fn set_override(&self, flag_name: &str, user_id: &str, value: bool) {
        self.overrides
            .write()
            .entry(flag_name.to_string())
            .or_default()
            .insert(user_id.to_string(), value);
    }

    /// Remove a user-level override.
    pub fn remove_override(&self, flag_name: &str, user_id: &str) {
        if let Some(user_overrides) = self.overrides.write().get_mut(flag_name) {
            user_overrides.remove(user_id);
        }
    }

    /// Toggle a flag's state.
    pub fn toggle(&self, flag_name: &str) -> Option<FlagState> {
        let mut flags = self.flags.write();
        let flag = flags.get_mut(flag_name)?;
        flag.state = match flag.state {
            FlagState::Enabled => FlagState::Disabled,
            FlagState::Disabled => FlagState::Enabled,
            FlagState::Conditional => FlagState::Disabled,
        };
        flag.updated_at = chrono::Utc::now();
        Some(flag.state)
    }

    /// Get flag info.
    pub fn get_flag(&self, name: &str) -> Option<FeatureFlag> {
        self.flags.read().get(name).cloned()
    }

    /// List all flags.
    pub fn list_flags(&self) -> Vec<FeatureFlag> {
        self.flags.read().values().cloned().collect()
    }

    /// Total number of flag evaluations.
    pub fn evaluation_count(&self) -> u64 {
        *self.evaluation_count.read()
    }

    /// Number of registered flags.
    pub fn flag_count(&self) -> usize {
        self.flags.read().len()
    }
}

impl Default for FlagManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_flag_enabled() {
        let mgr = FlagManager::new();
        mgr.create_flag("dark_mode", "Dark mode UI", true);

        let ctx = EvaluationContext::new();
        assert!(mgr.is_enabled("dark_mode", &ctx));
    }

    #[test]
    fn test_simple_flag_disabled() {
        let mgr = FlagManager::new();
        mgr.create_flag("beta_feature", "Beta feature", false);

        let ctx = EvaluationContext::new();
        assert!(!mgr.is_enabled("beta_feature", &ctx));
    }

    #[test]
    fn test_unknown_flag_returns_false() {
        let mgr = FlagManager::new();
        let ctx = EvaluationContext::new();
        assert!(!mgr.is_enabled("nonexistent", &ctx));
    }

    #[test]
    fn test_conditional_flag_with_targeting() {
        let mgr = FlagManager::new();
        let now = chrono::Utc::now();
        mgr.register(FeatureFlag {
            name: "premium_nav".to_string(),
            description: "Premium navigation features".to_string(),
            state: FlagState::Conditional,
            default_value: false,
            targeting_rules: vec![TargetingRule {
                attribute: "tier".to_string(),
                operator: TargetingOperator::Equals,
                value: "premium".to_string(),
                result: true,
            }],
            created_at: now,
            updated_at: now,
            owner: "nav_team".to_string(),
            tags: vec!["premium".to_string()],
        });

        let free_ctx = EvaluationContext::new().with_attribute("tier", "free");
        assert!(!mgr.is_enabled("premium_nav", &free_ctx));

        let premium_ctx = EvaluationContext::new().with_attribute("tier", "premium");
        assert!(mgr.is_enabled("premium_nav", &premium_ctx));
    }

    #[test]
    fn test_user_override() {
        let mgr = FlagManager::new();
        mgr.create_flag("new_ui", "New UI redesign", false);

        let ctx = EvaluationContext::new().with_user("user_123");
        assert!(!mgr.is_enabled("new_ui", &ctx));

        // Override for specific user
        mgr.set_override("new_ui", "user_123", true);
        assert!(mgr.is_enabled("new_ui", &ctx));

        // Other users not affected
        let other_ctx = EvaluationContext::new().with_user("user_456");
        assert!(!mgr.is_enabled("new_ui", &other_ctx));

        // Remove override
        mgr.remove_override("new_ui", "user_123");
        assert!(!mgr.is_enabled("new_ui", &ctx));
    }

    #[test]
    fn test_toggle_flag() {
        let mgr = FlagManager::new();
        mgr.create_flag("feature_x", "Feature X", true);

        let state = mgr.toggle("feature_x").unwrap();
        assert_eq!(state, FlagState::Disabled);

        let state = mgr.toggle("feature_x").unwrap();
        assert_eq!(state, FlagState::Enabled);
    }

    #[test]
    fn test_toggle_nonexistent() {
        let mgr = FlagManager::new();
        assert!(mgr.toggle("nonexistent").is_none());
    }

    #[test]
    fn test_evaluation_count() {
        let mgr = FlagManager::new();
        mgr.create_flag("f1", "Flag 1", true);

        let ctx = EvaluationContext::new();
        mgr.is_enabled("f1", &ctx);
        mgr.is_enabled("f1", &ctx);
        mgr.is_enabled("f1", &ctx);

        assert_eq!(mgr.evaluation_count(), 3);
    }

    #[test]
    fn test_targeting_operators() {
        let ctx = EvaluationContext::new()
            .with_attribute("region", "us-east")
            .with_attribute("version", "2.5");

        // Contains
        let rule = TargetingRule {
            attribute: "region".to_string(),
            operator: TargetingOperator::Contains,
            value: "east".to_string(),
            result: true,
        };
        assert_eq!(rule.evaluate(&ctx), Some(true));

        // StartsWith
        let rule = TargetingRule {
            attribute: "region".to_string(),
            operator: TargetingOperator::StartsWith,
            value: "us".to_string(),
            result: true,
        };
        assert_eq!(rule.evaluate(&ctx), Some(true));

        // GreaterThan
        let rule = TargetingRule {
            attribute: "version".to_string(),
            operator: TargetingOperator::GreaterThan,
            value: "2.0".to_string(),
            result: true,
        };
        assert_eq!(rule.evaluate(&ctx), Some(true));

        // In
        let rule = TargetingRule {
            attribute: "region".to_string(),
            operator: TargetingOperator::In,
            value: "us-east, us-west, eu-west".to_string(),
            result: true,
        };
        assert_eq!(rule.evaluate(&ctx), Some(true));
    }

    #[test]
    fn test_flag_count() {
        let mgr = FlagManager::new();
        mgr.create_flag("a", "", true);
        mgr.create_flag("b", "", false);
        assert_eq!(mgr.flag_count(), 2);
    }

    #[test]
    fn test_list_flags() {
        let mgr = FlagManager::new();
        mgr.create_flag("x", "desc x", true);
        mgr.create_flag("y", "desc y", false);
        let flags = mgr.list_flags();
        assert_eq!(flags.len(), 2);
    }
}
