//! What-if analysis engine — evaluates alternative routing decisions and their outcomes.

use std::collections::HashMap;
use std::time::Duration;

/// A what-if hypothesis to evaluate.
#[derive(Debug, Clone)]
pub struct Hypothesis {
    /// Unique hypothesis ID.
    pub id: u64,
    /// Description of the alternative.
    pub description: String,
    /// Changed parameters (key → value).
    pub parameters: HashMap<String, f64>,
}

/// Result of a what-if evaluation.
#[derive(Debug, Clone)]
pub struct WhatIfResult {
    /// Hypothesis that was evaluated.
    pub hypothesis_id: u64,
    /// Estimated travel time.
    pub estimated_time: Duration,
    /// Estimated distance (km).
    pub estimated_distance_km: f64,
    /// Risk score (0.0 = safe, 1.0 = very risky).
    pub risk_score: f64,
    /// Fuel/energy cost estimate (arbitrary units).
    pub energy_cost: f64,
    /// Comfort score (0.0 = worst, 1.0 = best).
    pub comfort_score: f64,
}

/// A comparison between baseline and alternative.
#[derive(Debug, Clone)]
pub struct Comparison {
    /// Baseline result.
    pub baseline: WhatIfResult,
    /// Alternative result.
    pub alternative: WhatIfResult,
    /// Time difference (positive = alternative is slower).
    pub time_diff: f64,
    /// Distance difference (positive = alternative is longer).
    pub distance_diff_km: f64,
    /// Recommendation.
    pub recommendation: Recommendation,
}

/// Recommendation based on what-if analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Recommendation {
    /// Stick with the current plan.
    KeepCurrent,
    /// Switch to the alternative.
    SwitchToAlternative,
    /// Both are roughly equivalent.
    Equivalent,
}

/// What-if analysis engine.
pub struct WhatIfEngine {
    hypotheses: Vec<Hypothesis>,
    results: HashMap<u64, WhatIfResult>,
    baseline: Option<WhatIfResult>,
    next_id: u64,
    /// Threshold for considering alternatives equivalent (seconds).
    equivalence_threshold_secs: f64,
}

impl WhatIfEngine {
    /// Create a new what-if engine.
    pub fn new() -> Self {
        Self {
            hypotheses: Vec::new(),
            results: HashMap::new(),
            baseline: None,
            next_id: 1,
            equivalence_threshold_secs: 60.0,
        }
    }

    /// Set the equivalence threshold.
    pub fn set_equivalence_threshold(&mut self, seconds: f64) {
        self.equivalence_threshold_secs = seconds.max(0.0);
    }

    /// Set the baseline result.
    pub fn set_baseline(&mut self, result: WhatIfResult) {
        self.baseline = Some(result);
    }

    /// Add a hypothesis.
    pub fn add_hypothesis(&mut self, description: &str, parameters: HashMap<String, f64>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.hypotheses.push(Hypothesis {
            id,
            description: description.to_string(),
            parameters,
        });
        id
    }

    /// Record the result of evaluating a hypothesis.
    pub fn record_result(&mut self, result: WhatIfResult) {
        self.results.insert(result.hypothesis_id, result);
    }

    /// Get the number of hypotheses.
    pub fn hypothesis_count(&self) -> usize {
        self.hypotheses.len()
    }

    /// Get the number of evaluated results.
    pub fn result_count(&self) -> usize {
        self.results.len()
    }

    /// Compare a hypothesis result against the baseline.
    pub fn compare(&self, hypothesis_id: u64) -> Option<Comparison> {
        let baseline = self.baseline.as_ref()?;
        let alternative = self.results.get(&hypothesis_id)?;

        let time_diff =
            alternative.estimated_time.as_secs_f64() - baseline.estimated_time.as_secs_f64();
        let distance_diff = alternative.estimated_distance_km - baseline.estimated_distance_km;

        let recommendation = if time_diff.abs() < self.equivalence_threshold_secs
            && (alternative.risk_score - baseline.risk_score).abs() < 0.1
        {
            Recommendation::Equivalent
        } else if time_diff < -self.equivalence_threshold_secs
            && alternative.risk_score <= baseline.risk_score + 0.1
        {
            Recommendation::SwitchToAlternative
        } else {
            Recommendation::KeepCurrent
        };

        Some(Comparison {
            baseline: baseline.clone(),
            alternative: alternative.clone(),
            time_diff,
            distance_diff_km: distance_diff,
            recommendation,
        })
    }

    /// Find the best hypothesis (lowest time with acceptable risk).
    pub fn best_alternative(&self, max_risk: f64) -> Option<u64> {
        self.results
            .values()
            .filter(|r| r.risk_score <= max_risk)
            .min_by(|a, b| {
                a.estimated_time
                    .partial_cmp(&b.estimated_time)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|r| r.hypothesis_id)
    }

    /// Get a hypothesis by ID.
    pub fn get_hypothesis(&self, id: u64) -> Option<&Hypothesis> {
        self.hypotheses.iter().find(|h| h.id == id)
    }
}

impl Default for WhatIfEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn baseline_result() -> WhatIfResult {
        WhatIfResult {
            hypothesis_id: 0,
            estimated_time: Duration::from_secs(1800), // 30 min
            estimated_distance_km: 25.0,
            risk_score: 0.3,
            energy_cost: 5.0,
            comfort_score: 0.8,
        }
    }

    fn faster_alternative(id: u64) -> WhatIfResult {
        WhatIfResult {
            hypothesis_id: id,
            estimated_time: Duration::from_secs(1200), // 20 min
            estimated_distance_km: 28.0,
            risk_score: 0.35,
            energy_cost: 6.0,
            comfort_score: 0.7,
        }
    }

    fn slower_alternative(id: u64) -> WhatIfResult {
        WhatIfResult {
            hypothesis_id: id,
            estimated_time: Duration::from_secs(2400), // 40 min
            estimated_distance_km: 22.0,
            risk_score: 0.2,
            energy_cost: 4.0,
            comfort_score: 0.9,
        }
    }

    fn equivalent_alternative(id: u64) -> WhatIfResult {
        WhatIfResult {
            hypothesis_id: id,
            estimated_time: Duration::from_secs(1830), // 30.5 min
            estimated_distance_km: 25.5,
            risk_score: 0.32,
            energy_cost: 5.1,
            comfort_score: 0.78,
        }
    }

    #[test]
    fn test_add_hypothesis() {
        let mut engine = WhatIfEngine::new();
        let id = engine.add_hypothesis("Take highway", HashMap::new());
        assert_eq!(id, 1);
        assert_eq!(engine.hypothesis_count(), 1);
    }

    #[test]
    fn test_record_result() {
        let mut engine = WhatIfEngine::new();
        let id = engine.add_hypothesis("Alt route", HashMap::new());
        engine.record_result(faster_alternative(id));
        assert_eq!(engine.result_count(), 1);
    }

    #[test]
    fn test_compare_faster() {
        let mut engine = WhatIfEngine::new();
        engine.set_baseline(baseline_result());
        let id = engine.add_hypothesis("Highway shortcut", HashMap::new());
        engine.record_result(faster_alternative(id));

        let cmp = engine.compare(id).unwrap();
        assert!(
            cmp.time_diff < 0.0,
            "Faster alternative should have negative time_diff"
        );
        assert_eq!(cmp.recommendation, Recommendation::SwitchToAlternative);
    }

    #[test]
    fn test_compare_slower() {
        let mut engine = WhatIfEngine::new();
        engine.set_baseline(baseline_result());
        let id = engine.add_hypothesis("Scenic route", HashMap::new());
        engine.record_result(slower_alternative(id));

        let cmp = engine.compare(id).unwrap();
        assert!(
            cmp.time_diff > 0.0,
            "Slower alternative should have positive time_diff"
        );
        assert_eq!(cmp.recommendation, Recommendation::KeepCurrent);
    }

    #[test]
    fn test_compare_equivalent() {
        let mut engine = WhatIfEngine::new();
        engine.set_baseline(baseline_result());
        let id = engine.add_hypothesis("Slight detour", HashMap::new());
        engine.record_result(equivalent_alternative(id));

        let cmp = engine.compare(id).unwrap();
        assert_eq!(cmp.recommendation, Recommendation::Equivalent);
    }

    #[test]
    fn test_compare_no_baseline() {
        let engine = WhatIfEngine::new();
        assert!(engine.compare(1).is_none());
    }

    #[test]
    fn test_compare_nonexistent_hypothesis() {
        let mut engine = WhatIfEngine::new();
        engine.set_baseline(baseline_result());
        assert!(engine.compare(999).is_none());
    }

    #[test]
    fn test_best_alternative() {
        let mut engine = WhatIfEngine::new();
        let id1 = engine.add_hypothesis("Fast risky", HashMap::new());
        let id2 = engine.add_hypothesis("Fast safe", HashMap::new());

        engine.record_result(WhatIfResult {
            hypothesis_id: id1,
            estimated_time: Duration::from_secs(1000),
            estimated_distance_km: 30.0,
            risk_score: 0.9, // Too risky
            energy_cost: 7.0,
            comfort_score: 0.5,
        });
        engine.record_result(WhatIfResult {
            hypothesis_id: id2,
            estimated_time: Duration::from_secs(1200),
            estimated_distance_km: 28.0,
            risk_score: 0.3,
            energy_cost: 6.0,
            comfort_score: 0.7,
        });

        // With max_risk=0.5, should pick id2 (id1 is too risky)
        let best = engine.best_alternative(0.5);
        assert_eq!(best, Some(id2));
    }

    #[test]
    fn test_best_alternative_none_acceptable() {
        let mut engine = WhatIfEngine::new();
        let id = engine.add_hypothesis("Very risky", HashMap::new());
        engine.record_result(WhatIfResult {
            hypothesis_id: id,
            estimated_time: Duration::from_secs(900),
            estimated_distance_km: 20.0,
            risk_score: 0.95,
            energy_cost: 3.0,
            comfort_score: 0.3,
        });
        assert!(engine.best_alternative(0.5).is_none());
    }

    #[test]
    fn test_get_hypothesis() {
        let mut engine = WhatIfEngine::new();
        let id = engine.add_hypothesis("Test route", HashMap::new());
        assert!(engine.get_hypothesis(id).is_some());
        assert!(engine.get_hypothesis(999).is_none());
    }

    #[test]
    fn test_equivalence_threshold() {
        let mut engine = WhatIfEngine::new();
        engine.set_equivalence_threshold(300.0); // 5 minutes
        engine.set_baseline(baseline_result());

        let id = engine.add_hypothesis("Slightly faster", HashMap::new());
        engine.record_result(WhatIfResult {
            hypothesis_id: id,
            estimated_time: Duration::from_secs(1620), // 27 min (3 min faster)
            estimated_distance_km: 26.0,
            risk_score: 0.32,
            energy_cost: 5.2,
            comfort_score: 0.78,
        });

        let cmp = engine.compare(id).unwrap();
        assert_eq!(
            cmp.recommendation,
            Recommendation::Equivalent,
            "3 min diff with 5 min threshold should be Equivalent"
        );
    }
}
