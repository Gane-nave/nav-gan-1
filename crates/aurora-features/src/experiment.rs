//! A/B experiment tracking — define experiments, assign variants, track metrics.

use std::collections::HashMap;

/// Experiment status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExperimentStatus {
    /// Experiment is in draft (not running).
    Draft,
    /// Experiment is actively running.
    Running,
    /// Experiment has been paused.
    Paused,
    /// Experiment has concluded.
    Concluded,
}

/// A variant in an experiment.
#[derive(Debug, Clone)]
pub struct Variant {
    /// Variant identifier.
    pub id: String,
    /// Variant name.
    pub name: String,
    /// Traffic allocation weight.
    pub weight: u32,
    /// Whether this is the control group.
    pub is_control: bool,
    /// Assigned user count.
    assigned_count: u64,
    /// Conversion count.
    conversions: u64,
    /// Total metric value (for computing averages).
    metric_sum: f64,
}

impl Variant {
    /// Create a new variant.
    pub fn new(id: &str, name: &str, weight: u32) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            weight,
            is_control: false,
            assigned_count: 0,
            conversions: 0,
            metric_sum: 0.0,
        }
    }

    /// Mark as control group.
    pub fn as_control(mut self) -> Self {
        self.is_control = true;
        self
    }

    /// Record an assignment.
    pub fn record_assignment(&mut self) {
        self.assigned_count += 1;
    }

    /// Record a conversion with an optional metric value.
    pub fn record_conversion(&mut self, metric_value: f64) {
        self.conversions += 1;
        self.metric_sum += metric_value;
    }

    /// Get the conversion rate.
    pub fn conversion_rate(&self) -> f64 {
        if self.assigned_count == 0 {
            return 0.0;
        }
        self.conversions as f64 / self.assigned_count as f64
    }

    /// Get the average metric value.
    pub fn avg_metric(&self) -> f64 {
        if self.conversions == 0 {
            return 0.0;
        }
        self.metric_sum / self.conversions as f64
    }

    /// Get assigned count.
    pub fn assigned_count(&self) -> u64 {
        self.assigned_count
    }

    /// Get conversion count.
    pub fn conversion_count(&self) -> u64 {
        self.conversions
    }
}

/// An A/B experiment.
#[derive(Debug, Clone)]
pub struct Experiment {
    /// Experiment identifier.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Description.
    pub description: String,
    /// Current status.
    pub status: ExperimentStatus,
    /// Variants in the experiment.
    pub variants: Vec<Variant>,
    /// User-to-variant assignments.
    assignments: HashMap<String, usize>,
    /// Start time.
    pub started_at_ms: Option<u64>,
    /// End time.
    pub ended_at_ms: Option<u64>,
    /// Creation time.
    pub created_at_ms: u64,
}

impl Experiment {
    /// Create a new experiment.
    pub fn new(id: &str, name: &str, now_ms: u64) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: String::new(),
            status: ExperimentStatus::Draft,
            variants: Vec::new(),
            assignments: HashMap::new(),
            started_at_ms: None,
            ended_at_ms: None,
            created_at_ms: now_ms,
        }
    }

    /// Set description.
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    /// Add a variant.
    pub fn add_variant(&mut self, variant: Variant) {
        self.variants.push(variant);
    }

    /// Start the experiment.
    pub fn start(&mut self, now_ms: u64) -> Result<(), String> {
        if self.variants.len() < 2 {
            return Err("Need at least 2 variants".to_string());
        }
        self.status = ExperimentStatus::Running;
        self.started_at_ms = Some(now_ms);
        Ok(())
    }

    /// Pause the experiment.
    pub fn pause(&mut self) {
        self.status = ExperimentStatus::Paused;
    }

    /// Conclude the experiment.
    pub fn conclude(&mut self, now_ms: u64) {
        self.status = ExperimentStatus::Concluded;
        self.ended_at_ms = Some(now_ms);
    }

    /// Assign a user to a variant (sticky assignment).
    pub fn assign_user(&mut self, user_id: &str) -> Option<&str> {
        if self.status != ExperimentStatus::Running {
            return None;
        }
        if let Some(&idx) = self.assignments.get(user_id) {
            return Some(&self.variants[idx].id);
        }
        let total_weight: u32 = self.variants.iter().map(|v| v.weight).sum();
        if total_weight == 0 {
            return None;
        }
        let hash = simple_hash(user_id);
        let bucket = (hash % total_weight as u64) as u32;
        let mut cumulative = 0u32;
        for (i, variant) in self.variants.iter_mut().enumerate() {
            cumulative += variant.weight;
            if bucket < cumulative {
                variant.record_assignment();
                self.assignments.insert(user_id.to_string(), i);
                return Some(&self.variants[i].id);
            }
        }
        None
    }

    /// Get the variant assigned to a user.
    pub fn get_assignment(&self, user_id: &str) -> Option<&str> {
        self.assignments
            .get(user_id)
            .map(|&idx| self.variants[idx].id.as_str())
    }

    /// Record a conversion for a user.
    pub fn record_conversion(&mut self, user_id: &str, metric_value: f64) -> bool {
        if let Some(&idx) = self.assignments.get(user_id) {
            self.variants[idx].record_conversion(metric_value);
            true
        } else {
            false
        }
    }

    /// Get the winning variant (highest conversion rate).
    pub fn winning_variant(&self) -> Option<&Variant> {
        self.variants.iter().max_by(|a, b| {
            a.conversion_rate()
                .partial_cmp(&b.conversion_rate())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Total participants.
    pub fn total_participants(&self) -> u64 {
        self.assignments.len() as u64
    }

    /// Get variant by ID.
    pub fn get_variant(&self, variant_id: &str) -> Option<&Variant> {
        self.variants.iter().find(|v| v.id == variant_id)
    }
}

/// Simple deterministic hash.
fn simple_hash(s: &str) -> u64 {
    let mut hash: u64 = 5381;
    for b in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(b as u64);
    }
    hash
}

/// Experiment registry — manages multiple experiments.
pub struct ExperimentRegistry {
    experiments: HashMap<String, Experiment>,
}

impl ExperimentRegistry {
    /// Create a new experiment registry.
    pub fn new() -> Self {
        Self {
            experiments: HashMap::new(),
        }
    }

    /// Register an experiment.
    pub fn register(&mut self, experiment: Experiment) -> Result<(), String> {
        if self.experiments.contains_key(&experiment.id) {
            return Err(format!("Experiment '{}' already exists", experiment.id));
        }
        self.experiments.insert(experiment.id.clone(), experiment);
        Ok(())
    }

    /// Get an experiment by ID.
    pub fn get(&self, id: &str) -> Option<&Experiment> {
        self.experiments.get(id)
    }

    /// Get a mutable experiment by ID.
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Experiment> {
        self.experiments.get_mut(id)
    }

    /// Remove an experiment.
    pub fn remove(&mut self, id: &str) -> Option<Experiment> {
        self.experiments.remove(id)
    }

    /// Count of experiments.
    pub fn count(&self) -> usize {
        self.experiments.len()
    }

    /// Running experiments.
    pub fn running_experiments(&self) -> Vec<&Experiment> {
        self.experiments
            .values()
            .filter(|e| e.status == ExperimentStatus::Running)
            .collect()
    }
}

impl Default for ExperimentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_experiment_lifecycle() {
        let mut exp = Experiment::new("exp1", "Button Color Test", 0);
        assert_eq!(exp.status, ExperimentStatus::Draft);

        exp.add_variant(Variant::new("control", "Blue", 50).as_control());
        exp.add_variant(Variant::new("treatment", "Green", 50));

        exp.start(1000).unwrap();
        assert_eq!(exp.status, ExperimentStatus::Running);

        exp.conclude(5000);
        assert_eq!(exp.status, ExperimentStatus::Concluded);
    }

    #[test]
    fn test_experiment_needs_two_variants() {
        let mut exp = Experiment::new("exp1", "Test", 0);
        exp.add_variant(Variant::new("v1", "Only One", 100));
        assert!(exp.start(0).is_err());
    }

    #[test]
    fn test_experiment_assignment_sticky() {
        let mut exp = Experiment::new("exp1", "Test", 0);
        exp.add_variant(Variant::new("a", "A", 50));
        exp.add_variant(Variant::new("b", "B", 50));
        exp.start(0).unwrap();

        let first = exp.assign_user("user1").unwrap().to_string();
        let second = exp.assign_user("user1").unwrap().to_string();
        assert_eq!(first, second, "Assignment must be sticky");
    }

    #[test]
    fn test_experiment_conversion_tracking() {
        let mut exp = Experiment::new("exp1", "Test", 0);
        exp.add_variant(Variant::new("a", "A", 50));
        exp.add_variant(Variant::new("b", "B", 50));
        exp.start(0).unwrap();

        exp.assign_user("user1");
        assert!(exp.record_conversion("user1", 1.0));
        assert!(!exp.record_conversion("unknown", 1.0));

        assert_eq!(exp.total_participants(), 1);
    }

    #[test]
    fn test_variant_metrics() {
        let mut v = Variant::new("v1", "Test", 50);
        v.record_assignment();
        v.record_assignment();
        v.record_conversion(10.0);

        assert_eq!(v.assigned_count(), 2);
        assert_eq!(v.conversion_count(), 1);
        assert!((v.conversion_rate() - 0.5).abs() < f64::EPSILON);
        assert!((v.avg_metric() - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_experiment_distribution() {
        let mut exp = Experiment::new("exp1", "Test", 0);
        exp.add_variant(Variant::new("a", "A", 50));
        exp.add_variant(Variant::new("b", "B", 50));
        exp.start(0).unwrap();

        let mut a_count = 0;
        let mut b_count = 0;
        for i in 0..1000 {
            match exp.assign_user(&format!("user_{i}")).unwrap() {
                "a" => a_count += 1,
                "b" => b_count += 1,
                _ => {}
            }
        }
        // Should be roughly 50/50
        assert!(a_count > 350, "A={a_count}");
        assert!(b_count > 350, "B={b_count}");
    }

    #[test]
    fn test_experiment_winning_variant() {
        let mut exp = Experiment::new("exp1", "Test", 0);
        exp.add_variant(Variant::new("a", "A", 50));
        exp.add_variant(Variant::new("b", "B", 50));
        exp.start(0).unwrap();

        // Assign and convert more for variant a
        for i in 0..100 {
            exp.assign_user(&format!("u{i}"));
        }
        // Find who got variant a and convert them
        for i in 0..100 {
            let uid = format!("u{i}");
            if exp.get_assignment(&uid) == Some("a") {
                exp.record_conversion(&uid, 1.0);
            }
        }

        let winner = exp.winning_variant().unwrap();
        assert_eq!(winner.id, "a");
    }

    #[test]
    fn test_experiment_registry() {
        let mut reg = ExperimentRegistry::new();
        let mut exp = Experiment::new("exp1", "Test", 0);
        exp.add_variant(Variant::new("a", "A", 50));
        exp.add_variant(Variant::new("b", "B", 50));
        exp.start(0).unwrap();

        reg.register(exp).unwrap();
        assert_eq!(reg.count(), 1);
        assert_eq!(reg.running_experiments().len(), 1);
    }

    #[test]
    fn test_experiment_no_assign_when_not_running() {
        let mut exp = Experiment::new("exp1", "Test", 0);
        exp.add_variant(Variant::new("a", "A", 50));
        exp.add_variant(Variant::new("b", "B", 50));

        // Still in draft
        assert!(exp.assign_user("user1").is_none());
    }
}
