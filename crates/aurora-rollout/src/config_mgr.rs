//! Configuration management — dynamic configuration, versioning, and validation.

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration value type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConfigValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    List(Vec<String>),
}

impl ConfigValue {
    /// Get as string, if it is one.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// Get as integer, if it is one.
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Self::Integer(i) => Some(*i),
            _ => None,
        }
    }

    /// Get as float, if it is one.
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// Get as boolean, if it is one.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Get as list, if it is one.
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            Self::List(l) => Some(l),
            _ => None,
        }
    }
}

/// A configuration entry with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigEntry {
    pub key: String,
    pub value: ConfigValue,
    pub description: String,
    pub version: u64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub source: ConfigSource,
}

/// Source of a configuration value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfigSource {
    Default,
    File,
    Environment,
    Remote,
    Override,
}

/// Validator function type for configuration values.
pub type ValidatorFn = Box<dyn Fn(&ConfigValue) -> Result<(), String> + Send + Sync>;

/// Configuration validation rule.
pub struct ValidationRule {
    pub key: String,
    pub description: String,
    pub validate: ValidatorFn,
}

/// Dynamic configuration manager.
pub struct ConfigManager {
    entries: RwLock<HashMap<String, ConfigEntry>>,
    validation_rules: Vec<ValidationRule>,
    change_history: RwLock<Vec<ConfigChange>>,
    max_history: usize,
}

/// Record of a configuration change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigChange {
    pub key: String,
    pub old_value: Option<ConfigValue>,
    pub new_value: ConfigValue,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub source: ConfigSource,
}

impl ConfigManager {
    /// Create a new config manager.
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
            validation_rules: Vec::new(),
            change_history: RwLock::new(Vec::new()),
            max_history: 1000,
        }
    }

    /// Set a configuration value.
    pub fn set(
        &self,
        key: &str,
        value: ConfigValue,
        description: &str,
        source: ConfigSource,
    ) -> Result<(), String> {
        // Validate if there's a rule
        for rule in &self.validation_rules {
            if rule.key == key {
                (rule.validate)(&value)?;
            }
        }

        let now = chrono::Utc::now();
        let mut entries = self.entries.write();

        let old_value = entries.get(key).map(|e| e.value.clone());
        let version = entries.get(key).map(|e| e.version + 1).unwrap_or(1);

        entries.insert(
            key.to_string(),
            ConfigEntry {
                key: key.to_string(),
                value: value.clone(),
                description: description.to_string(),
                version,
                updated_at: now,
                source: source.clone(),
            },
        );

        // Record change
        let mut history = self.change_history.write();
        history.push(ConfigChange {
            key: key.to_string(),
            old_value,
            new_value: value,
            timestamp: now,
            source,
        });
        while history.len() > self.max_history {
            history.remove(0);
        }

        Ok(())
    }

    /// Get a configuration value.
    pub fn get(&self, key: &str) -> Option<ConfigValue> {
        self.entries.read().get(key).map(|e| e.value.clone())
    }

    /// Get a configuration entry with metadata.
    pub fn get_entry(&self, key: &str) -> Option<ConfigEntry> {
        self.entries.read().get(key).cloned()
    }

    /// Get a string value with a default.
    pub fn get_string(&self, key: &str, default: &str) -> String {
        self.get(key)
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| default.to_string())
    }

    /// Get an integer value with a default.
    pub fn get_int(&self, key: &str, default: i64) -> i64 {
        self.get(key).and_then(|v| v.as_int()).unwrap_or(default)
    }

    /// Get a float value with a default.
    pub fn get_float(&self, key: &str, default: f64) -> f64 {
        self.get(key).and_then(|v| v.as_float()).unwrap_or(default)
    }

    /// Get a boolean value with a default.
    pub fn get_bool(&self, key: &str, default: bool) -> bool {
        self.get(key).and_then(|v| v.as_bool()).unwrap_or(default)
    }

    /// Add a validation rule.
    pub fn add_validation_rule(&mut self, rule: ValidationRule) {
        self.validation_rules.push(rule);
    }

    /// Remove a configuration key.
    pub fn remove(&self, key: &str) -> Option<ConfigValue> {
        self.entries.write().remove(key).map(|e| e.value)
    }

    /// List all configuration keys.
    pub fn keys(&self) -> Vec<String> {
        self.entries.read().keys().cloned().collect()
    }

    /// Get all entries.
    pub fn all_entries(&self) -> Vec<ConfigEntry> {
        self.entries.read().values().cloned().collect()
    }

    /// Get change history.
    pub fn history(&self, limit: usize) -> Vec<ConfigChange> {
        let history = self.change_history.read();
        history.iter().rev().take(limit).cloned().collect()
    }

    /// Number of configuration entries.
    pub fn entry_count(&self) -> usize {
        self.entries.read().len()
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_set_and_get() {
        let mgr = ConfigManager::new();
        mgr.set(
            "max_speed",
            ConfigValue::Float(120.0),
            "Maximum speed km/h",
            ConfigSource::Default,
        )
        .unwrap();

        assert_eq!(mgr.get_float("max_speed", 0.0), 120.0);
    }

    #[test]
    fn test_config_types() {
        let mgr = ConfigManager::new();

        mgr.set(
            "name",
            ConfigValue::String("aurora".to_string()),
            "",
            ConfigSource::Default,
        )
        .unwrap();
        mgr.set(
            "port",
            ConfigValue::Integer(8080),
            "",
            ConfigSource::Default,
        )
        .unwrap();
        mgr.set("rate", ConfigValue::Float(0.5), "", ConfigSource::Default)
            .unwrap();
        mgr.set(
            "debug",
            ConfigValue::Boolean(true),
            "",
            ConfigSource::Default,
        )
        .unwrap();
        mgr.set(
            "regions",
            ConfigValue::List(vec!["us".to_string(), "eu".to_string()]),
            "",
            ConfigSource::Default,
        )
        .unwrap();

        assert_eq!(mgr.get_string("name", ""), "aurora");
        assert_eq!(mgr.get_int("port", 0), 8080);
        assert_eq!(mgr.get_float("rate", 0.0), 0.5);
        assert!(mgr.get_bool("debug", false));

        let regions = mgr.get("regions").unwrap();
        assert_eq!(regions.as_list().unwrap().len(), 2);
    }

    #[test]
    fn test_config_defaults() {
        let mgr = ConfigManager::new();
        assert_eq!(mgr.get_string("missing", "default"), "default");
        assert_eq!(mgr.get_int("missing", 42), 42);
        assert_eq!(mgr.get_float("missing", 3.15), 3.15);
        assert!(!mgr.get_bool("missing", false));
    }

    #[test]
    fn test_config_versioning() {
        let mgr = ConfigManager::new();
        mgr.set("key", ConfigValue::Integer(1), "", ConfigSource::Default)
            .unwrap();
        let entry = mgr.get_entry("key").unwrap();
        assert_eq!(entry.version, 1);

        mgr.set("key", ConfigValue::Integer(2), "", ConfigSource::Override)
            .unwrap();
        let entry = mgr.get_entry("key").unwrap();
        assert_eq!(entry.version, 2);
        assert_eq!(entry.source, ConfigSource::Override);
    }

    #[test]
    fn test_config_validation() {
        let mut mgr = ConfigManager::new();
        mgr.add_validation_rule(ValidationRule {
            key: "port".to_string(),
            description: "Port must be 1-65535".to_string(),
            validate: Box::new(|v| {
                if let Some(port) = v.as_int() {
                    if (1..=65535).contains(&port) {
                        Ok(())
                    } else {
                        Err(format!("Port {port} out of range 1-65535"))
                    }
                } else {
                    Err("Port must be an integer".to_string())
                }
            }),
        });

        assert!(mgr
            .set(
                "port",
                ConfigValue::Integer(8080),
                "",
                ConfigSource::Default
            )
            .is_ok());
        assert!(mgr
            .set("port", ConfigValue::Integer(0), "", ConfigSource::Default)
            .is_err());
        assert!(mgr
            .set(
                "port",
                ConfigValue::Integer(70000),
                "",
                ConfigSource::Default
            )
            .is_err());
    }

    #[test]
    fn test_config_remove() {
        let mgr = ConfigManager::new();
        mgr.set(
            "temp",
            ConfigValue::Boolean(true),
            "",
            ConfigSource::Default,
        )
        .unwrap();
        assert_eq!(mgr.entry_count(), 1);

        let removed = mgr.remove("temp");
        assert!(removed.is_some());
        assert_eq!(mgr.entry_count(), 0);
    }

    #[test]
    fn test_config_history() {
        let mgr = ConfigManager::new();
        mgr.set("a", ConfigValue::Integer(1), "", ConfigSource::Default)
            .unwrap();
        mgr.set("a", ConfigValue::Integer(2), "", ConfigSource::Override)
            .unwrap();
        mgr.set("b", ConfigValue::Boolean(true), "", ConfigSource::File)
            .unwrap();

        let history = mgr.history(10);
        assert_eq!(history.len(), 3);
        // Most recent first
        assert_eq!(history[0].key, "b");
        assert_eq!(history[1].key, "a");
    }

    #[test]
    fn test_config_history_old_value() {
        let mgr = ConfigManager::new();
        mgr.set("x", ConfigValue::Integer(10), "", ConfigSource::Default)
            .unwrap();
        mgr.set("x", ConfigValue::Integer(20), "", ConfigSource::Override)
            .unwrap();

        let history = mgr.history(10);
        // Most recent change should show old value
        assert_eq!(history[0].old_value, Some(ConfigValue::Integer(10)));
        assert_eq!(history[0].new_value, ConfigValue::Integer(20));
    }

    #[test]
    fn test_config_keys() {
        let mgr = ConfigManager::new();
        mgr.set("a", ConfigValue::Integer(1), "", ConfigSource::Default)
            .unwrap();
        mgr.set("b", ConfigValue::Integer(2), "", ConfigSource::Default)
            .unwrap();
        let mut keys = mgr.keys();
        keys.sort();
        assert_eq!(keys, vec!["a", "b"]);
    }

    #[test]
    fn test_config_value_accessors() {
        assert_eq!(ConfigValue::String("hi".to_string()).as_str(), Some("hi"));
        assert_eq!(ConfigValue::Integer(42).as_int(), Some(42));
        assert_eq!(ConfigValue::Float(1.5).as_float(), Some(1.5));
        assert_eq!(ConfigValue::Boolean(true).as_bool(), Some(true));

        // Wrong type returns None
        assert!(ConfigValue::Integer(42).as_str().is_none());
        assert!(ConfigValue::String("x".to_string()).as_int().is_none());
    }
}
