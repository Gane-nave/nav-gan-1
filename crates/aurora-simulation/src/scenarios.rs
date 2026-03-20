//! Scenario engine — defines and executes predefined navigation scenarios for testing and validation.

use std::collections::HashMap;
use std::time::Duration;

/// Scenario difficulty level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Difficulty {
    /// Simple, well-known route.
    Easy,
    /// Moderate complexity (some turns, moderate traffic).
    Medium,
    /// Complex route with challenging conditions.
    Hard,
    /// Extreme conditions (weather, closures, sensor degradation).
    Extreme,
}

/// Scenario category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScenarioCategory {
    /// Standard navigation from A to B.
    Navigation,
    /// Emergency response routing.
    Emergency,
    /// Multi-modal transport scenario.
    MultiModal,
    /// Adverse weather conditions.
    Weather,
    /// Urban dense traffic.
    UrbanTraffic,
    /// Highway long-distance.
    Highway,
    /// Sensor failure scenarios.
    SensorFailure,
    /// Network outage / offline.
    Offline,
}

/// A condition that applies during the scenario.
#[derive(Debug, Clone)]
pub struct ScenarioCondition {
    /// Condition name.
    pub name: String,
    /// When this condition activates (offset from scenario start).
    pub activate_at: Duration,
    /// Duration of the condition.
    pub duration: Duration,
    /// Severity (0.0 = none, 1.0 = maximum).
    pub severity: f64,
}

/// Scenario execution result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenarioResult {
    /// Scenario passed all assertions.
    Passed,
    /// Scenario failed one or more assertions.
    Failed,
    /// Scenario was not run.
    NotRun,
    /// Scenario was skipped (e.g., missing prerequisites).
    Skipped,
}

/// A scenario definition.
#[derive(Debug, Clone)]
pub struct Scenario {
    /// Unique scenario ID.
    pub id: u64,
    /// Scenario name.
    pub name: String,
    /// Description.
    pub description: String,
    /// Category.
    pub category: ScenarioCategory,
    /// Difficulty level.
    pub difficulty: Difficulty,
    /// Expected duration.
    pub expected_duration: Duration,
    /// Conditions that apply during this scenario.
    pub conditions: Vec<ScenarioCondition>,
    /// Tags for filtering.
    pub tags: Vec<String>,
    /// Last result.
    pub result: ScenarioResult,
}

/// Builder for creating scenarios without too many arguments.
pub struct ScenarioBuilder {
    /// Scenario name.
    pub name: String,
    /// Description.
    pub description: String,
    /// Category.
    pub category: ScenarioCategory,
    /// Difficulty level.
    pub difficulty: Difficulty,
    /// Expected duration.
    pub expected_duration: Duration,
    /// Conditions.
    pub conditions: Vec<ScenarioCondition>,
    /// Tags.
    pub tags: Vec<String>,
}

impl ScenarioBuilder {
    /// Create a new scenario builder.
    pub fn new(name: &str, category: ScenarioCategory, difficulty: Difficulty) -> Self {
        Self {
            name: name.to_string(),
            description: String::new(),
            category,
            difficulty,
            expected_duration: Duration::from_secs(0),
            conditions: Vec::new(),
            tags: Vec::new(),
        }
    }

    /// Set the description.
    pub fn description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    /// Set the expected duration.
    pub fn expected_duration(mut self, dur: Duration) -> Self {
        self.expected_duration = dur;
        self
    }

    /// Set conditions.
    pub fn conditions(mut self, conditions: Vec<ScenarioCondition>) -> Self {
        self.conditions = conditions;
        self
    }

    /// Set tags.
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}

/// Scenario engine — manages a library of test scenarios.
pub struct ScenarioEngine {
    scenarios: Vec<Scenario>,
    next_id: u64,
    results: HashMap<u64, ScenarioResult>,
}

impl ScenarioEngine {
    /// Create a new scenario engine.
    pub fn new() -> Self {
        Self {
            scenarios: Vec::new(),
            next_id: 1,
            results: HashMap::new(),
        }
    }

    /// Register a new scenario from a builder.
    pub fn register(&mut self, builder: ScenarioBuilder) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.scenarios.push(Scenario {
            id,
            name: builder.name,
            description: builder.description,
            category: builder.category,
            difficulty: builder.difficulty,
            expected_duration: builder.expected_duration,
            conditions: builder.conditions,
            tags: builder.tags,
            result: ScenarioResult::NotRun,
        });
        id
    }

    /// Get total scenario count.
    pub fn scenario_count(&self) -> usize {
        self.scenarios.len()
    }

    /// Get a scenario by ID.
    pub fn get(&self, id: u64) -> Option<&Scenario> {
        self.scenarios.iter().find(|s| s.id == id)
    }

    /// Filter scenarios by category.
    pub fn by_category(&self, category: ScenarioCategory) -> Vec<&Scenario> {
        self.scenarios
            .iter()
            .filter(|s| s.category == category)
            .collect()
    }

    /// Filter scenarios by difficulty.
    pub fn by_difficulty(&self, difficulty: Difficulty) -> Vec<&Scenario> {
        self.scenarios
            .iter()
            .filter(|s| s.difficulty == difficulty)
            .collect()
    }

    /// Filter scenarios by tag.
    pub fn by_tag(&self, tag: &str) -> Vec<&Scenario> {
        self.scenarios
            .iter()
            .filter(|s| s.tags.iter().any(|t| t == tag))
            .collect()
    }

    /// Mark a scenario result.
    pub fn set_result(&mut self, id: u64, result: ScenarioResult) -> bool {
        if let Some(scenario) = self.scenarios.iter_mut().find(|s| s.id == id) {
            scenario.result = result;
            self.results.insert(id, result);
            true
        } else {
            false
        }
    }

    /// Get pass rate as a fraction.
    pub fn pass_rate(&self) -> f64 {
        let total = self.results.len();
        if total == 0 {
            return 0.0;
        }
        let passed = self
            .results
            .values()
            .filter(|r| **r == ScenarioResult::Passed)
            .count();
        passed as f64 / total as f64
    }

    /// Get summary counts: (passed, failed, skipped, not_run).
    pub fn summary(&self) -> (usize, usize, usize, usize) {
        let mut passed = 0;
        let mut failed = 0;
        let mut skipped = 0;
        let mut not_run = 0;
        for s in &self.scenarios {
            match s.result {
                ScenarioResult::Passed => passed += 1,
                ScenarioResult::Failed => failed += 1,
                ScenarioResult::Skipped => skipped += 1,
                ScenarioResult::NotRun => not_run += 1,
            }
        }
        (passed, failed, skipped, not_run)
    }
}

impl Default for ScenarioEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_engine() -> ScenarioEngine {
        let mut engine = ScenarioEngine::new();
        engine.register(
            ScenarioBuilder::new(
                "Basic city route",
                ScenarioCategory::Navigation,
                Difficulty::Easy,
            )
            .description("Navigate through downtown")
            .expected_duration(Duration::from_secs(600))
            .tags(vec!["city".to_string(), "basic".to_string()]),
        );
        engine.register(
            ScenarioBuilder::new("Highway rain", ScenarioCategory::Weather, Difficulty::Hard)
                .description("Long highway route in heavy rain")
                .expected_duration(Duration::from_secs(3600))
                .conditions(vec![ScenarioCondition {
                    name: "Heavy rain".to_string(),
                    activate_at: Duration::from_secs(300),
                    duration: Duration::from_secs(1800),
                    severity: 0.8,
                }])
                .tags(vec!["highway".to_string(), "weather".to_string()]),
        );
        engine.register(
            ScenarioBuilder::new(
                "Sensor failure recovery",
                ScenarioCategory::SensorFailure,
                Difficulty::Extreme,
            )
            .description("GPS outage during urban navigation")
            .expected_duration(Duration::from_secs(1200))
            .conditions(vec![ScenarioCondition {
                name: "GPS blackout".to_string(),
                activate_at: Duration::from_secs(120),
                duration: Duration::from_secs(300),
                severity: 1.0,
            }])
            .tags(vec!["sensor".to_string(), "gps".to_string()]),
        );
        engine
    }

    #[test]
    fn test_register_scenarios() {
        let engine = make_engine();
        assert_eq!(engine.scenario_count(), 3);
    }

    #[test]
    fn test_sequential_ids() {
        let engine = make_engine();
        assert!(engine.get(1).is_some());
        assert!(engine.get(2).is_some());
        assert!(engine.get(3).is_some());
        assert!(engine.get(4).is_none());
    }

    #[test]
    fn test_filter_by_category() {
        let engine = make_engine();
        let nav = engine.by_category(ScenarioCategory::Navigation);
        assert_eq!(nav.len(), 1);
        assert_eq!(nav[0].name, "Basic city route");
    }

    #[test]
    fn test_filter_by_difficulty() {
        let engine = make_engine();
        let hard = engine.by_difficulty(Difficulty::Hard);
        assert_eq!(hard.len(), 1);
        assert_eq!(hard[0].name, "Highway rain");
    }

    #[test]
    fn test_filter_by_tag() {
        let engine = make_engine();
        let weather = engine.by_tag("weather");
        assert_eq!(weather.len(), 1);
        let sensor = engine.by_tag("sensor");
        assert_eq!(sensor.len(), 1);
    }

    #[test]
    fn test_set_result() {
        let mut engine = make_engine();
        assert!(engine.set_result(1, ScenarioResult::Passed));
        assert!(engine.set_result(2, ScenarioResult::Failed));
        assert!(!engine.set_result(999, ScenarioResult::Passed)); // Nonexistent
    }

    #[test]
    fn test_pass_rate() {
        let mut engine = make_engine();
        engine.set_result(1, ScenarioResult::Passed);
        engine.set_result(2, ScenarioResult::Passed);
        engine.set_result(3, ScenarioResult::Failed);
        assert!((engine.pass_rate() - 2.0 / 3.0).abs() < 0.01);
    }

    #[test]
    fn test_pass_rate_empty() {
        let engine = ScenarioEngine::new();
        assert_eq!(engine.pass_rate(), 0.0);
    }

    #[test]
    fn test_summary() {
        let mut engine = make_engine();
        engine.set_result(1, ScenarioResult::Passed);
        engine.set_result(2, ScenarioResult::Failed);
        // Third scenario is NotRun
        let (passed, failed, skipped, not_run) = engine.summary();
        assert_eq!(passed, 1);
        assert_eq!(failed, 1);
        assert_eq!(skipped, 0);
        assert_eq!(not_run, 1);
    }

    #[test]
    fn test_scenario_conditions() {
        let engine = make_engine();
        let scenario = engine.get(2).unwrap();
        assert_eq!(scenario.conditions.len(), 1);
        assert_eq!(scenario.conditions[0].name, "Heavy rain");
        assert!((scenario.conditions[0].severity - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_difficulty_ordering() {
        assert!(Difficulty::Easy < Difficulty::Medium);
        assert!(Difficulty::Medium < Difficulty::Hard);
        assert!(Difficulty::Hard < Difficulty::Extreme);
    }
}
