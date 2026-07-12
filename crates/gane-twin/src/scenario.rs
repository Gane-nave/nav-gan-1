//! Scenario management — define, parameterize, and compare what-if
//! scenarios for digital twin simulations.

use chrono::{DateTime, Utc};
use gane_core::types::EntityId;
use std::collections::HashMap;
use tracing::{debug, info};

// ---------------------------------------------------------------------------
// Scenario types
// ---------------------------------------------------------------------------

/// Status of a scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenarioStatus {
    /// Defined but not yet executed.
    Defined,
    /// Currently being executed.
    Executing,
    /// Finished execution.
    Completed,
    /// Execution failed.
    Failed,
}

/// A parameter that can be swept across values.
#[derive(Debug, Clone)]
pub struct ScenarioParameter {
    pub name: String,
    pub description: String,
    pub base_value: f64,
    /// Values to test in a sweep.
    pub sweep_values: Vec<f64>,
    pub unit: String,
}

/// Outcome metrics from a completed scenario.
#[derive(Debug, Clone)]
pub struct ScenarioOutcome {
    pub scenario_id: EntityId,
    pub parameter_values: HashMap<String, f64>,
    pub result_metrics: HashMap<String, f64>,
    pub duration_ticks: u64,
    pub computed_at: DateTime<Utc>,
}

/// A what-if scenario definition.
#[derive(Debug, Clone)]
pub struct Scenario {
    pub id: EntityId,
    pub name: String,
    pub description: String,
    pub parameters: Vec<ScenarioParameter>,
    pub status: ScenarioStatus,
    pub created_at: DateTime<Utc>,
    pub outcomes: Vec<ScenarioOutcome>,
}

/// Comparison result between two scenario outcomes.
#[derive(Debug, Clone)]
pub struct ScenarioComparison {
    pub scenario_a: EntityId,
    pub scenario_b: EntityId,
    /// Metric name → (value_a, value_b, delta, pct_change).
    pub metric_diffs: Vec<MetricDiff>,
    pub winner: Option<EntityId>,
    pub compared_at: DateTime<Utc>,
}

/// A single metric difference between two scenarios.
#[derive(Debug, Clone)]
pub struct MetricDiff {
    pub metric: String,
    pub value_a: f64,
    pub value_b: f64,
    pub delta: f64,
    pub pct_change: f64,
}

/// Ranking criterion for scenarios.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RankingCriterion {
    /// Higher metric value is better.
    HigherIsBetter,
    /// Lower metric value is better.
    LowerIsBetter,
}

// ---------------------------------------------------------------------------
// Scenario manager
// ---------------------------------------------------------------------------

/// Manages what-if scenarios: definition, parameter sweeps, outcome
/// recording, and comparison of alternatives.
pub struct ScenarioManager {
    scenarios: HashMap<EntityId, Scenario>,
}

impl ScenarioManager {
    pub fn new() -> Self {
        Self {
            scenarios: HashMap::new(),
        }
    }

    // -----------------------------------------------------------------------
    // CRUD
    // -----------------------------------------------------------------------

    /// Create a new scenario.
    pub fn create_scenario(
        &mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        parameters: Vec<ScenarioParameter>,
    ) -> EntityId {
        let id = EntityId::new();
        let scenario = Scenario {
            id,
            name: name.into(),
            description: description.into(),
            parameters,
            status: ScenarioStatus::Defined,
            created_at: Utc::now(),
            outcomes: Vec::new(),
        };
        info!(id = %id, name = %scenario.name, "scenario created");
        self.scenarios.insert(id, scenario);
        id
    }

    /// Get a scenario by ID.
    pub fn get_scenario(&self, id: &EntityId) -> Option<&Scenario> {
        self.scenarios.get(id)
    }

    /// Get a mutable reference to a scenario.
    pub fn get_scenario_mut(&mut self, id: &EntityId) -> Option<&mut Scenario> {
        self.scenarios.get_mut(id)
    }

    /// Remove a scenario.
    pub fn remove_scenario(&mut self, id: &EntityId) -> bool {
        self.scenarios.remove(id).is_some()
    }

    /// Number of scenarios.
    pub fn scenario_count(&self) -> usize {
        self.scenarios.len()
    }

    /// List all scenarios.
    pub fn list_scenarios(&self) -> Vec<&Scenario> {
        self.scenarios.values().collect()
    }

    // -----------------------------------------------------------------------
    // Parameter sweeps
    // -----------------------------------------------------------------------

    /// Generate all combinations of parameter sweep values for a scenario.
    pub fn generate_sweep_combinations(
        &self,
        scenario_id: &EntityId,
    ) -> Option<Vec<HashMap<String, f64>>> {
        let scenario = self.scenarios.get(scenario_id)?;

        if scenario.parameters.is_empty() {
            return Some(vec![HashMap::new()]);
        }

        let mut combinations: Vec<HashMap<String, f64>> = vec![HashMap::new()];

        for param in &scenario.parameters {
            let values = if param.sweep_values.is_empty() {
                vec![param.base_value]
            } else {
                param.sweep_values.clone()
            };

            let mut new_combinations = Vec::new();
            for combo in &combinations {
                for &val in &values {
                    let mut new_combo = combo.clone();
                    new_combo.insert(param.name.clone(), val);
                    new_combinations.push(new_combo);
                }
            }
            combinations = new_combinations;
        }

        debug!(
            scenario = %scenario_id,
            count = combinations.len(),
            "generated sweep combinations"
        );
        Some(combinations)
    }

    // -----------------------------------------------------------------------
    // Outcomes
    // -----------------------------------------------------------------------

    /// Record an outcome for a scenario.
    pub fn record_outcome(
        &mut self,
        scenario_id: &EntityId,
        parameter_values: HashMap<String, f64>,
        result_metrics: HashMap<String, f64>,
        duration_ticks: u64,
    ) -> bool {
        let Some(scenario) = self.scenarios.get_mut(scenario_id) else {
            return false;
        };

        let outcome = ScenarioOutcome {
            scenario_id: *scenario_id,
            parameter_values,
            result_metrics,
            duration_ticks,
            computed_at: Utc::now(),
        };

        scenario.outcomes.push(outcome);
        debug!(
            scenario = %scenario_id,
            outcomes = scenario.outcomes.len(),
            "outcome recorded"
        );
        true
    }

    /// Mark a scenario's status.
    pub fn set_status(&mut self, scenario_id: &EntityId, status: ScenarioStatus) -> bool {
        if let Some(scenario) = self.scenarios.get_mut(scenario_id) {
            scenario.status = status;
            return true;
        }
        false
    }

    // -----------------------------------------------------------------------
    // Comparison
    // -----------------------------------------------------------------------

    /// Compare two scenarios on a specific metric.
    pub fn compare(
        &self,
        scenario_a_id: &EntityId,
        scenario_b_id: &EntityId,
        ranking_metric: &str,
        criterion: RankingCriterion,
    ) -> Option<ScenarioComparison> {
        let a = self.scenarios.get(scenario_a_id)?;
        let b = self.scenarios.get(scenario_b_id)?;

        let a_outcome = a.outcomes.last()?;
        let b_outcome = b.outcomes.last()?;

        let mut metric_diffs = Vec::new();

        // Collect all metrics from both outcomes.
        let mut all_metrics: Vec<String> = a_outcome
            .result_metrics
            .keys()
            .chain(b_outcome.result_metrics.keys())
            .cloned()
            .collect();
        all_metrics.sort();
        all_metrics.dedup();

        for metric in &all_metrics {
            let val_a = a_outcome.result_metrics.get(metric).copied().unwrap_or(0.0);
            let val_b = b_outcome.result_metrics.get(metric).copied().unwrap_or(0.0);
            let delta = val_b - val_a;
            let pct = if val_a.abs() > f64::EPSILON {
                (delta / val_a) * 100.0
            } else {
                0.0
            };

            metric_diffs.push(MetricDiff {
                metric: metric.clone(),
                value_a: val_a,
                value_b: val_b,
                delta,
                pct_change: pct,
            });
        }

        // Determine winner based on ranking metric.
        let val_a = a_outcome
            .result_metrics
            .get(ranking_metric)
            .copied()
            .unwrap_or(0.0);
        let val_b = b_outcome
            .result_metrics
            .get(ranking_metric)
            .copied()
            .unwrap_or(0.0);

        let winner = match criterion {
            RankingCriterion::HigherIsBetter => {
                if val_a > val_b {
                    Some(*scenario_a_id)
                } else if val_b > val_a {
                    Some(*scenario_b_id)
                } else {
                    None
                }
            }
            RankingCriterion::LowerIsBetter => {
                if val_a < val_b {
                    Some(*scenario_a_id)
                } else if val_b < val_a {
                    Some(*scenario_b_id)
                } else {
                    None
                }
            }
        };

        Some(ScenarioComparison {
            scenario_a: *scenario_a_id,
            scenario_b: *scenario_b_id,
            metric_diffs,
            winner,
            compared_at: Utc::now(),
        })
    }

    /// Rank all scenarios with outcomes by a metric.
    pub fn rank_by_metric(
        &self,
        metric: &str,
        criterion: RankingCriterion,
    ) -> Vec<(EntityId, f64)> {
        let mut ranked: Vec<(EntityId, f64)> = self
            .scenarios
            .values()
            .filter_map(|s| {
                s.outcomes
                    .last()
                    .and_then(|o| o.result_metrics.get(metric).map(|v| (s.id, *v)))
            })
            .collect();

        match criterion {
            RankingCriterion::HigherIsBetter => {
                ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            }
            RankingCriterion::LowerIsBetter => {
                ranked.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            }
        }

        ranked
    }
}

impl Default for ScenarioManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_params() -> Vec<ScenarioParameter> {
        vec![
            ScenarioParameter {
                name: "speed_limit".into(),
                description: "Max speed".into(),
                base_value: 60.0,
                sweep_values: vec![50.0, 60.0, 70.0],
                unit: "km/h".into(),
            },
            ScenarioParameter {
                name: "signal_timing".into(),
                description: "Green phase duration".into(),
                base_value: 30.0,
                sweep_values: vec![20.0, 30.0],
                unit: "seconds".into(),
            },
        ]
    }

    #[test]
    fn create_and_query_scenario() {
        let mut mgr = ScenarioManager::new();
        let id = mgr.create_scenario("test", "A test scenario", make_params());

        assert_eq!(mgr.scenario_count(), 1);
        let s = mgr.get_scenario(&id).unwrap();
        assert_eq!(s.name, "test");
        assert_eq!(s.status, ScenarioStatus::Defined);
    }

    #[test]
    fn remove_scenario() {
        let mut mgr = ScenarioManager::new();
        let id = mgr.create_scenario("temp", "temporary", vec![]);

        assert!(mgr.remove_scenario(&id));
        assert_eq!(mgr.scenario_count(), 0);
    }

    #[test]
    fn generate_sweep_combinations() {
        let mut mgr = ScenarioManager::new();
        let id = mgr.create_scenario("sweep", "sweep test", make_params());

        let combos = mgr.generate_sweep_combinations(&id).unwrap();
        // 3 speed_limit × 2 signal_timing = 6 combinations.
        assert_eq!(combos.len(), 6);

        // Each combo should have both parameters.
        for combo in &combos {
            assert!(combo.contains_key("speed_limit"));
            assert!(combo.contains_key("signal_timing"));
        }
    }

    #[test]
    fn empty_params_single_combo() {
        let mut mgr = ScenarioManager::new();
        let id = mgr.create_scenario("no_params", "no parameters", vec![]);

        let combos = mgr.generate_sweep_combinations(&id).unwrap();
        assert_eq!(combos.len(), 1);
    }

    #[test]
    fn record_and_retrieve_outcome() {
        let mut mgr = ScenarioManager::new();
        let id = mgr.create_scenario("test", "test", vec![]);

        let mut params = HashMap::new();
        params.insert("speed".into(), 60.0);

        let mut metrics = HashMap::new();
        metrics.insert("avg_travel_time".into(), 12.5);
        metrics.insert("fuel_consumption".into(), 8.2);

        assert!(mgr.record_outcome(&id, params, metrics, 100));

        let s = mgr.get_scenario(&id).unwrap();
        assert_eq!(s.outcomes.len(), 1);
        assert_eq!(s.outcomes[0].duration_ticks, 100);
    }

    #[test]
    fn set_status() {
        let mut mgr = ScenarioManager::new();
        let id = mgr.create_scenario("test", "test", vec![]);

        assert!(mgr.set_status(&id, ScenarioStatus::Executing));
        assert_eq!(
            mgr.get_scenario(&id).unwrap().status,
            ScenarioStatus::Executing
        );
    }

    #[test]
    fn compare_scenarios_higher_is_better() {
        let mut mgr = ScenarioManager::new();
        let a = mgr.create_scenario("A", "Scenario A", vec![]);
        let b = mgr.create_scenario("B", "Scenario B", vec![]);

        let mut metrics_a = HashMap::new();
        metrics_a.insert("throughput".into(), 100.0);
        mgr.record_outcome(&a, HashMap::new(), metrics_a, 50);

        let mut metrics_b = HashMap::new();
        metrics_b.insert("throughput".into(), 150.0);
        mgr.record_outcome(&b, HashMap::new(), metrics_b, 50);

        let cmp = mgr
            .compare(&a, &b, "throughput", RankingCriterion::HigherIsBetter)
            .unwrap();
        assert_eq!(cmp.winner, Some(b));
        assert!(!cmp.metric_diffs.is_empty());
    }

    #[test]
    fn compare_scenarios_lower_is_better() {
        let mut mgr = ScenarioManager::new();
        let a = mgr.create_scenario("A", "A", vec![]);
        let b = mgr.create_scenario("B", "B", vec![]);

        let mut ma = HashMap::new();
        ma.insert("latency".into(), 20.0);
        mgr.record_outcome(&a, HashMap::new(), ma, 50);

        let mut mb = HashMap::new();
        mb.insert("latency".into(), 35.0);
        mgr.record_outcome(&b, HashMap::new(), mb, 50);

        let cmp = mgr
            .compare(&a, &b, "latency", RankingCriterion::LowerIsBetter)
            .unwrap();
        assert_eq!(cmp.winner, Some(a));
    }

    #[test]
    fn rank_by_metric() {
        let mut mgr = ScenarioManager::new();
        let a = mgr.create_scenario("A", "A", vec![]);
        let b = mgr.create_scenario("B", "B", vec![]);
        let c = mgr.create_scenario("C", "C", vec![]);

        let mut ma = HashMap::new();
        ma.insert("score".into(), 70.0);
        mgr.record_outcome(&a, HashMap::new(), ma, 10);

        let mut mb = HashMap::new();
        mb.insert("score".into(), 90.0);
        mgr.record_outcome(&b, HashMap::new(), mb, 10);

        let mut mc = HashMap::new();
        mc.insert("score".into(), 80.0);
        mgr.record_outcome(&c, HashMap::new(), mc, 10);

        let ranked = mgr.rank_by_metric("score", RankingCriterion::HigherIsBetter);
        assert_eq!(ranked.len(), 3);
        assert_eq!(ranked[0].0, b); // highest
        assert_eq!(ranked[1].0, c);
        assert_eq!(ranked[2].0, a); // lowest
    }

    #[test]
    fn metric_diff_pct_change() {
        let mut mgr = ScenarioManager::new();
        let a = mgr.create_scenario("A", "A", vec![]);
        let b = mgr.create_scenario("B", "B", vec![]);

        let mut ma = HashMap::new();
        ma.insert("flow".into(), 100.0);
        mgr.record_outcome(&a, HashMap::new(), ma, 10);

        let mut mb = HashMap::new();
        mb.insert("flow".into(), 120.0);
        mgr.record_outcome(&b, HashMap::new(), mb, 10);

        let cmp = mgr
            .compare(&a, &b, "flow", RankingCriterion::HigherIsBetter)
            .unwrap();

        let flow_diff = cmp
            .metric_diffs
            .iter()
            .find(|d| d.metric == "flow")
            .unwrap();
        assert!((flow_diff.delta - 20.0).abs() < f64::EPSILON);
        assert!((flow_diff.pct_change - 20.0).abs() < f64::EPSILON);
    }
}
