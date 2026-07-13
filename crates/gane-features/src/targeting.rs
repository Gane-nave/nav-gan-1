//! Targeting rules — define conditions for who sees a feature based on attributes.

use std::collections::HashMap;

/// Comparison operator for targeting conditions.
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    /// Equals.
    Eq,
    /// Not equals.
    Ne,
    /// Greater than (numeric).
    Gt,
    /// Less than (numeric).
    Lt,
    /// Greater than or equal (numeric).
    Gte,
    /// Less than or equal (numeric).
    Lte,
    /// Contains (string).
    Contains,
    /// Starts with (string).
    StartsWith,
    /// Ends with (string).
    EndsWith,
    /// Is one of (list).
    OneOf,
}

/// An attribute value for targeting evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
    /// String value.
    Str(String),
    /// Numeric value.
    Num(f64),
    /// Boolean value.
    Bool(bool),
    /// List of string values.
    List(Vec<String>),
}

impl AttributeValue {
    /// Get as string reference.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Str(s) => Some(s),
            _ => None,
        }
    }

    /// Get as f64.
    pub fn as_num(&self) -> Option<f64> {
        match self {
            Self::Num(n) => Some(*n),
            _ => None,
        }
    }

    /// Get as bool.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

/// A targeting condition.
#[derive(Debug, Clone)]
pub struct Condition {
    /// Attribute name.
    pub attribute: String,
    /// Comparison operator.
    pub operator: Operator,
    /// Value to compare against.
    pub value: AttributeValue,
}

impl Condition {
    /// Create a new condition.
    pub fn new(attribute: &str, operator: Operator, value: AttributeValue) -> Self {
        Self {
            attribute: attribute.to_string(),
            operator,
            value,
        }
    }

    /// Evaluate the condition against a user context.
    pub fn evaluate(&self, context: &UserContext) -> bool {
        let attr = match context.get(&self.attribute) {
            Some(v) => v,
            None => return false,
        };

        match (&self.operator, attr, &self.value) {
            (Operator::Eq, a, b) => a == b,
            (Operator::Ne, a, b) => a != b,
            (Operator::Gt, AttributeValue::Num(a), AttributeValue::Num(b)) => a > b,
            (Operator::Lt, AttributeValue::Num(a), AttributeValue::Num(b)) => a < b,
            (Operator::Gte, AttributeValue::Num(a), AttributeValue::Num(b)) => a >= b,
            (Operator::Lte, AttributeValue::Num(a), AttributeValue::Num(b)) => a <= b,
            (Operator::Contains, AttributeValue::Str(a), AttributeValue::Str(b)) => {
                a.contains(b.as_str())
            }
            (Operator::StartsWith, AttributeValue::Str(a), AttributeValue::Str(b)) => {
                a.starts_with(b.as_str())
            }
            (Operator::EndsWith, AttributeValue::Str(a), AttributeValue::Str(b)) => {
                a.ends_with(b.as_str())
            }
            (Operator::OneOf, AttributeValue::Str(a), AttributeValue::List(list)) => {
                list.contains(a)
            }
            _ => false,
        }
    }
}

/// User context — a set of attributes for targeting evaluation.
#[derive(Debug, Clone)]
pub struct UserContext {
    attributes: HashMap<String, AttributeValue>,
}

impl UserContext {
    /// Create a new user context.
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }

    /// Set a string attribute.
    pub fn set_str(&mut self, key: &str, value: &str) {
        self.attributes
            .insert(key.to_string(), AttributeValue::Str(value.to_string()));
    }

    /// Set a numeric attribute.
    pub fn set_num(&mut self, key: &str, value: f64) {
        self.attributes
            .insert(key.to_string(), AttributeValue::Num(value));
    }

    /// Set a boolean attribute.
    pub fn set_bool(&mut self, key: &str, value: bool) {
        self.attributes
            .insert(key.to_string(), AttributeValue::Bool(value));
    }

    /// Get an attribute.
    pub fn get(&self, key: &str) -> Option<&AttributeValue> {
        self.attributes.get(key)
    }

    /// Check if an attribute exists.
    pub fn has(&self, key: &str) -> bool {
        self.attributes.contains_key(key)
    }

    /// Get attribute count.
    pub fn attribute_count(&self) -> usize {
        self.attributes.len()
    }
}

impl Default for UserContext {
    fn default() -> Self {
        Self::new()
    }
}

/// A targeting rule — a set of conditions combined with AND logic.
#[derive(Debug, Clone)]
pub struct TargetingRule {
    /// Rule identifier.
    pub id: String,
    /// Conditions (all must match).
    pub conditions: Vec<Condition>,
    /// Whether the rule is active.
    pub active: bool,
}

impl TargetingRule {
    /// Create a new targeting rule.
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            conditions: Vec::new(),
            active: true,
        }
    }

    /// Add a condition.
    pub fn add_condition(&mut self, condition: Condition) {
        self.conditions.push(condition);
    }

    /// Evaluate the rule against a user context (all conditions must match).
    pub fn evaluate(&self, context: &UserContext) -> bool {
        if !self.active || self.conditions.is_empty() {
            return false;
        }
        self.conditions.iter().all(|c| c.evaluate(context))
    }
}

/// Targeting engine — evaluates targeting rules for feature flags.
pub struct TargetingEngine {
    rules: Vec<TargetingRule>,
}

impl TargetingEngine {
    /// Create a new targeting engine.
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Add a targeting rule.
    pub fn add_rule(&mut self, rule: TargetingRule) {
        self.rules.push(rule);
    }

    /// Evaluate all rules — returns true if ANY rule matches (OR logic across rules).
    pub fn evaluate(&self, context: &UserContext) -> bool {
        self.rules.iter().any(|r| r.evaluate(context))
    }

    /// Get matching rules.
    pub fn matching_rules(&self, context: &UserContext) -> Vec<&TargetingRule> {
        self.rules.iter().filter(|r| r.evaluate(context)).collect()
    }

    /// Rule count.
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

impl Default for TargetingEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition_eq() {
        let cond = Condition::new(
            "country",
            Operator::Eq,
            AttributeValue::Str("IL".to_string()),
        );
        let mut ctx = UserContext::new();
        ctx.set_str("country", "IL");
        assert!(cond.evaluate(&ctx));

        ctx.set_str("country", "US");
        assert!(!cond.evaluate(&ctx));
    }

    #[test]
    fn test_condition_numeric() {
        let cond = Condition::new("age", Operator::Gte, AttributeValue::Num(18.0));
        let mut ctx = UserContext::new();
        ctx.set_num("age", 25.0);
        assert!(cond.evaluate(&ctx));

        ctx.set_num("age", 15.0);
        assert!(!cond.evaluate(&ctx));
    }

    #[test]
    fn test_condition_contains() {
        let cond = Condition::new(
            "email",
            Operator::Contains,
            AttributeValue::Str("@gane.nav".to_string()),
        );
        let mut ctx = UserContext::new();
        ctx.set_str("email", "user@gane.nav");
        assert!(cond.evaluate(&ctx));

        ctx.set_str("email", "user@other.com");
        assert!(!cond.evaluate(&ctx));
    }

    #[test]
    fn test_condition_one_of() {
        let cond = Condition::new(
            "tier",
            Operator::OneOf,
            AttributeValue::List(vec!["gold".to_string(), "platinum".to_string()]),
        );
        let mut ctx = UserContext::new();
        ctx.set_str("tier", "gold");
        assert!(cond.evaluate(&ctx));

        ctx.set_str("tier", "silver");
        assert!(!cond.evaluate(&ctx));
    }

    #[test]
    fn test_condition_missing_attribute() {
        let cond = Condition::new("plan", Operator::Eq, AttributeValue::Str("pro".to_string()));
        let ctx = UserContext::new();
        assert!(!cond.evaluate(&ctx));
    }

    #[test]
    fn test_targeting_rule_all_conditions() {
        let mut rule = TargetingRule::new("r1");
        rule.add_condition(Condition::new(
            "country",
            Operator::Eq,
            AttributeValue::Str("IL".to_string()),
        ));
        rule.add_condition(Condition::new(
            "age",
            Operator::Gte,
            AttributeValue::Num(18.0),
        ));

        let mut ctx = UserContext::new();
        ctx.set_str("country", "IL");
        ctx.set_num("age", 25.0);
        assert!(rule.evaluate(&ctx));

        ctx.set_num("age", 15.0);
        assert!(!rule.evaluate(&ctx));
    }

    #[test]
    fn test_targeting_engine_any_rule() {
        let mut engine = TargetingEngine::new();

        let mut r1 = TargetingRule::new("r1");
        r1.add_condition(Condition::new(
            "country",
            Operator::Eq,
            AttributeValue::Str("IL".to_string()),
        ));

        let mut r2 = TargetingRule::new("r2");
        r2.add_condition(Condition::new(
            "tier",
            Operator::Eq,
            AttributeValue::Str("premium".to_string()),
        ));

        engine.add_rule(r1);
        engine.add_rule(r2);

        let mut ctx = UserContext::new();
        ctx.set_str("country", "US");
        ctx.set_str("tier", "premium");
        assert!(engine.evaluate(&ctx)); // r2 matches

        let mut ctx2 = UserContext::new();
        ctx2.set_str("country", "IL");
        assert!(engine.evaluate(&ctx2)); // r1 matches
    }

    #[test]
    fn test_targeting_engine_no_match() {
        let mut engine = TargetingEngine::new();
        let mut r1 = TargetingRule::new("r1");
        r1.add_condition(Condition::new(
            "country",
            Operator::Eq,
            AttributeValue::Str("IL".to_string()),
        ));
        engine.add_rule(r1);

        let mut ctx = UserContext::new();
        ctx.set_str("country", "US");
        assert!(!engine.evaluate(&ctx));
    }

    #[test]
    fn test_user_context() {
        let mut ctx = UserContext::new();
        ctx.set_str("name", "Alice");
        ctx.set_num("age", 30.0);
        ctx.set_bool("premium", true);

        assert_eq!(ctx.attribute_count(), 3);
        assert!(ctx.has("name"));
        assert_eq!(ctx.get("name").unwrap().as_str(), Some("Alice"));
        assert_eq!(ctx.get("age").unwrap().as_num(), Some(30.0));
        assert_eq!(ctx.get("premium").unwrap().as_bool(), Some(true));
    }

    #[test]
    fn test_inactive_rule() {
        let mut rule = TargetingRule::new("r1");
        rule.add_condition(Condition::new(
            "x",
            Operator::Eq,
            AttributeValue::Bool(true),
        ));
        rule.active = false;

        let mut ctx = UserContext::new();
        ctx.set_bool("x", true);
        assert!(!rule.evaluate(&ctx));
    }

    #[test]
    fn test_starts_with_ends_with() {
        let starts = Condition::new(
            "url",
            Operator::StartsWith,
            AttributeValue::Str("/api/".to_string()),
        );
        let ends = Condition::new(
            "file",
            Operator::EndsWith,
            AttributeValue::Str(".rs".to_string()),
        );

        let mut ctx = UserContext::new();
        ctx.set_str("url", "/api/v1/nav");
        ctx.set_str("file", "main.rs");
        assert!(starts.evaluate(&ctx));
        assert!(ends.evaluate(&ctx));

        ctx.set_str("url", "/web/home");
        assert!(!starts.evaluate(&ctx));
    }
}
