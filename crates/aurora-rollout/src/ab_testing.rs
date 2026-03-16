//! A/B testing — experiment management, variant assignment, and result tracking.

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An A/B test experiment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
    pub name: String,
    pub description: String,
    pub variants: Vec<Variant>,
    pub status: ExperimentStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub metrics: Vec<String>,
}

/// Experiment status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExperimentStatus {
    Draft,
    Running,
    Paused,
    Completed,
}

/// A variant in an A/B test.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
    pub name: String,
    pub weight: f64,
    pub config: HashMap<String, String>,
}

impl Variant {
    /// Create a new variant.
    pub fn new(name: &str, weight: f64) -> Self {
        Self {
            name: name.to_string(),
            weight,
            config: HashMap::new(),
        }
    }

    /// Add a config value to the variant.
    pub fn with_config(mut self, key: &str, value: &str) -> Self {
        self.config.insert(key.to_string(), value.to_string());
        self
    }
}

/// Result data for a variant.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VariantResult {
    pub variant_name: String,
    pub participants: u64,
    pub conversions: u64,
    pub metric_values: HashMap<String, f64>,
}

impl VariantResult {
    /// Conversion rate.
    pub fn conversion_rate(&self) -> f64 {
        if self.participants == 0 {
            0.0
        } else {
            self.conversions as f64 / self.participants as f64
        }
    }
}

/// A/B test experiment manager.
pub struct ExperimentManager {
    experiments: RwLock<HashMap<String, Experiment>>,
    assignments: RwLock<HashMap<String, HashMap<String, String>>>, // experiment -> user_id -> variant
    results: RwLock<HashMap<String, HashMap<String, VariantResult>>>, // experiment -> variant -> result
}

impl ExperimentManager {
    /// Create a new experiment manager.
    pub fn new() -> Self {
        Self {
            experiments: RwLock::new(HashMap::new()),
            assignments: RwLock::new(HashMap::new()),
            results: RwLock::new(HashMap::new()),
        }
    }

    /// Create and register a new experiment.
    pub fn create_experiment(
        &self,
        name: &str,
        description: &str,
        variants: Vec<Variant>,
    ) -> Result<(), String> {
        if variants.is_empty() {
            return Err("Experiment must have at least one variant".to_string());
        }

        let total_weight: f64 = variants.iter().map(|v| v.weight).sum();
        if (total_weight - 1.0).abs() > 0.01 {
            return Err(format!(
                "Variant weights must sum to 1.0, got {total_weight}"
            ));
        }

        let experiment = Experiment {
            name: name.to_string(),
            description: description.to_string(),
            variants,
            status: ExperimentStatus::Draft,
            created_at: chrono::Utc::now(),
            metrics: Vec::new(),
        };

        self.experiments
            .write()
            .insert(name.to_string(), experiment);
        Ok(())
    }

    /// Start an experiment.
    pub fn start(&self, experiment_name: &str) -> Result<(), String> {
        let mut experiments = self.experiments.write();
        let exp = experiments
            .get_mut(experiment_name)
            .ok_or_else(|| format!("Experiment '{experiment_name}' not found"))?;

        if exp.status != ExperimentStatus::Draft && exp.status != ExperimentStatus::Paused {
            return Err(format!(
                "Cannot start experiment in {:?} status",
                exp.status
            ));
        }

        exp.status = ExperimentStatus::Running;
        Ok(())
    }

    /// Assign a user to a variant. Returns the assigned variant name.
    pub fn assign_variant(&self, experiment_name: &str, user_id: &str) -> Option<String> {
        // Check if user already assigned
        if let Some(variant) = self
            .assignments
            .read()
            .get(experiment_name)
            .and_then(|m| m.get(user_id))
        {
            return Some(variant.clone());
        }

        let experiments = self.experiments.read();
        let exp = experiments.get(experiment_name)?;

        if exp.status != ExperimentStatus::Running {
            return None;
        }

        // Deterministic assignment based on hash
        let hash = Self::hash_user(experiment_name, user_id);
        let bucket = (hash % 10000) as f64 / 10000.0;

        let mut cumulative = 0.0;
        let mut assigned_variant = exp.variants.last()?.name.clone();
        for variant in &exp.variants {
            cumulative += variant.weight;
            if bucket < cumulative {
                assigned_variant = variant.name.clone();
                break;
            }
        }

        // Record assignment
        self.assignments
            .write()
            .entry(experiment_name.to_string())
            .or_default()
            .insert(user_id.to_string(), assigned_variant.clone());

        // Increment participant count
        self.results
            .write()
            .entry(experiment_name.to_string())
            .or_default()
            .entry(assigned_variant.clone())
            .or_insert_with(|| VariantResult {
                variant_name: assigned_variant.clone(),
                ..Default::default()
            })
            .participants += 1;

        Some(assigned_variant)
    }

    /// Record a conversion for a user in an experiment.
    pub fn record_conversion(&self, experiment_name: &str, user_id: &str) -> bool {
        let variant = match self
            .assignments
            .read()
            .get(experiment_name)
            .and_then(|m| m.get(user_id))
        {
            Some(v) => v.clone(),
            None => return false,
        };

        self.results
            .write()
            .entry(experiment_name.to_string())
            .or_default()
            .entry(variant.clone())
            .or_insert_with(|| VariantResult {
                variant_name: variant,
                ..Default::default()
            })
            .conversions += 1;

        true
    }

    /// Get results for an experiment.
    pub fn get_results(&self, experiment_name: &str) -> Vec<VariantResult> {
        self.results
            .read()
            .get(experiment_name)
            .map(|m| m.values().cloned().collect())
            .unwrap_or_default()
    }

    /// Get the number of experiments.
    pub fn experiment_count(&self) -> usize {
        self.experiments.read().len()
    }

    /// Get experiment info.
    pub fn get_experiment(&self, name: &str) -> Option<Experiment> {
        self.experiments.read().get(name).cloned()
    }

    fn hash_user(experiment: &str, user_id: &str) -> u64 {
        let mut hash: u64 = 14695981039346656037;
        for byte in experiment
            .bytes()
            .chain(b"\0".iter().copied())
            .chain(user_id.bytes())
        {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(1099511628211);
        }
        hash
    }
}

impl Default for ExperimentManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_ab_experiment(mgr: &ExperimentManager) {
        mgr.create_experiment(
            "routing_algo",
            "Compare routing algorithms",
            vec![
                Variant::new("control", 0.5).with_config("algorithm", "dijkstra"),
                Variant::new("treatment", 0.5).with_config("algorithm", "astar"),
            ],
        )
        .unwrap();
    }

    #[test]
    fn test_create_experiment() {
        let mgr = ExperimentManager::new();
        create_ab_experiment(&mgr);
        assert_eq!(mgr.experiment_count(), 1);

        let exp = mgr.get_experiment("routing_algo").unwrap();
        assert_eq!(exp.status, ExperimentStatus::Draft);
        assert_eq!(exp.variants.len(), 2);
    }

    #[test]
    fn test_invalid_weights() {
        let mgr = ExperimentManager::new();
        let result = mgr.create_experiment(
            "bad",
            "Invalid",
            vec![Variant::new("a", 0.3), Variant::new("b", 0.3)],
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("1.0"));
    }

    #[test]
    fn test_empty_variants() {
        let mgr = ExperimentManager::new();
        let result = mgr.create_experiment("empty", "No variants", vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_start_experiment() {
        let mgr = ExperimentManager::new();
        create_ab_experiment(&mgr);

        mgr.start("routing_algo").unwrap();
        let exp = mgr.get_experiment("routing_algo").unwrap();
        assert_eq!(exp.status, ExperimentStatus::Running);
    }

    #[test]
    fn test_assign_variant() {
        let mgr = ExperimentManager::new();
        create_ab_experiment(&mgr);
        mgr.start("routing_algo").unwrap();

        let variant = mgr.assign_variant("routing_algo", "user_1").unwrap();
        assert!(variant == "control" || variant == "treatment");

        // Same user always gets same variant
        let variant2 = mgr.assign_variant("routing_algo", "user_1").unwrap();
        assert_eq!(variant, variant2);
    }

    #[test]
    fn test_assign_variant_not_running() {
        let mgr = ExperimentManager::new();
        create_ab_experiment(&mgr);
        // Not started yet
        assert!(mgr.assign_variant("routing_algo", "user_1").is_none());
    }

    #[test]
    fn test_record_conversion() {
        let mgr = ExperimentManager::new();
        create_ab_experiment(&mgr);
        mgr.start("routing_algo").unwrap();

        mgr.assign_variant("routing_algo", "user_1");
        assert!(mgr.record_conversion("routing_algo", "user_1"));

        // Unassigned user can't convert
        assert!(!mgr.record_conversion("routing_algo", "unknown_user"));
    }

    #[test]
    fn test_experiment_results() {
        let mgr = ExperimentManager::new();
        create_ab_experiment(&mgr);
        mgr.start("routing_algo").unwrap();

        // Assign several users
        for i in 0..10 {
            let user = format!("user_{i}");
            mgr.assign_variant("routing_algo", &user);
            if i % 3 == 0 {
                mgr.record_conversion("routing_algo", &user);
            }
        }

        let results = mgr.get_results("routing_algo");
        assert!(!results.is_empty());

        let total_participants: u64 = results.iter().map(|r| r.participants).sum();
        assert_eq!(total_participants, 10);
    }

    #[test]
    fn test_conversion_rate() {
        let result = VariantResult {
            variant_name: "test".to_string(),
            participants: 100,
            conversions: 25,
            metric_values: HashMap::new(),
        };
        assert!((result.conversion_rate() - 0.25).abs() < 0.001);
    }

    #[test]
    fn test_conversion_rate_zero_participants() {
        let result = VariantResult {
            variant_name: "test".to_string(),
            participants: 0,
            conversions: 0,
            metric_values: HashMap::new(),
        };
        assert_eq!(result.conversion_rate(), 0.0);
    }
}
