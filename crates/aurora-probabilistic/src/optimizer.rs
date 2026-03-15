//! Probabilistic router — multi-objective route optimization with uncertainty propagation.

use aurora_confidence::calculator::SegmentEstimate;
use aurora_confidence::ConfidenceCalculator;
use aurora_core::map::RoadSegment;
use aurora_core::route::EtaDistribution;
use aurora_core::types::EntityId;
use aurora_risk::engine::{EnvironmentConditions, RiskEngine, SegmentHistory};
use tracing::debug;

use crate::explanation::{ExplanationGraph, ExplanationNode, ObjectiveContribution};

/// Weights for multi-objective optimization.
#[derive(Debug, Clone)]
pub struct ObjectiveWeights {
    pub time: f64,
    pub distance: f64,
    pub risk: f64,
    pub stability: f64,
    pub cognitive_load: f64,
    pub energy: f64,
    pub emissions: f64,
}

impl Default for ObjectiveWeights {
    fn default() -> Self {
        Self {
            time: 0.35,
            distance: 0.15,
            risk: 0.20,
            stability: 0.10,
            cognitive_load: 0.05,
            energy: 0.10,
            emissions: 0.05,
        }
    }
}

/// A candidate route with its segments for probabilistic evaluation.
#[derive(Debug, Clone)]
pub struct CandidateRoute {
    pub id: EntityId,
    pub label: String,
    pub segments: Vec<RoadSegment>,
    pub segment_histories: Vec<SegmentHistory>,
}

/// Result of probabilistic route evaluation.
#[derive(Debug, Clone)]
pub struct RouteEvaluation {
    pub route_id: EntityId,
    pub label: String,
    /// Composite score [0, 1] where lower is better.
    pub composite_score: f64,
    /// Individual objective scores.
    pub objectives: ObjectiveScores,
    /// ETA probability distribution.
    pub eta: EtaDistribution,
    /// Explanation of why this route was ranked this way.
    pub explanation: ExplanationGraph,
}

/// Normalized scores for each optimization objective.
#[derive(Debug, Clone)]
pub struct ObjectiveScores {
    pub time: f64,
    pub distance: f64,
    pub risk: f64,
    pub stability: f64,
    pub cognitive_load: f64,
    pub energy: f64,
    pub emissions: f64,
}

/// Probabilistic router that evaluates and ranks candidate routes.
pub struct ProbabilisticRouter {
    risk_engine: RiskEngine,
    confidence_calc: ConfidenceCalculator,
    weights: ObjectiveWeights,
}

impl ProbabilisticRouter {
    pub fn new() -> Self {
        Self {
            risk_engine: RiskEngine::new(),
            confidence_calc: ConfidenceCalculator::new(),
            weights: ObjectiveWeights::default(),
        }
    }

    pub fn with_weights(weights: ObjectiveWeights) -> Self {
        Self {
            risk_engine: RiskEngine::new(),
            confidence_calc: ConfidenceCalculator::new(),
            weights,
        }
    }

    /// Evaluate a single candidate route.
    pub fn evaluate(
        &self,
        candidate: &CandidateRoute,
        env: &EnvironmentConditions,
    ) -> RouteEvaluation {
        let seg_refs: Vec<&RoadSegment> = candidate.segments.iter().collect();

        // Risk scoring.
        let risk_score = self
            .risk_engine
            .score_route(&seg_refs, env, &candidate.segment_histories);

        // Build segment estimates for confidence calculation.
        let seg_estimates: Vec<SegmentEstimate> = candidate
            .segments
            .iter()
            .zip(candidate.segment_histories.iter())
            .map(|(seg, hist)| {
                let expected = seg.travel_time_s.unwrap_or_else(|| {
                    let speed_mps = seg.speed_limit_kmh.unwrap_or(50.0) / 3.6;
                    seg.length_m / speed_mps
                });
                // Std dev heuristic: 10% of expected + risk-based uncertainty.
                let base_std = expected * 0.1;
                let risk_std = expected * 0.05 * (hist.accident_count as f64 / 5.0).min(1.0);
                SegmentEstimate {
                    expected_s: expected,
                    std_dev_s: base_std + risk_std,
                    risk_score: risk_score.score,
                    reliability: 0.8,
                }
            })
            .collect();

        let eta = self
            .confidence_calc
            .compute_eta(candidate.id, &seg_estimates);

        // Compute individual objectives.
        let total_distance: f64 = candidate.segments.iter().map(|s| s.length_m).sum();
        let total_time = eta.expected_s;

        let objectives = ObjectiveScores {
            time: normalize_time(total_time),
            distance: normalize_distance(total_distance),
            risk: risk_score.score,
            stability: 1.0 - eta.volatility,
            cognitive_load: estimate_cognitive_load(&candidate.segments),
            energy: estimate_energy(total_distance, &candidate.segments),
            emissions: estimate_emissions(total_distance, &candidate.segments),
        };

        // Composite score (weighted sum, lower is better).
        let composite = self.weights.time * objectives.time
            + self.weights.distance * objectives.distance
            + self.weights.risk * objectives.risk
            + self.weights.stability * (1.0 - objectives.stability)
            + self.weights.cognitive_load * objectives.cognitive_load
            + self.weights.energy * objectives.energy
            + self.weights.emissions * objectives.emissions;

        // Build explanation graph.
        let explanation = build_explanation(candidate, &objectives, &self.weights, composite);

        debug!(
            route_id = %candidate.id,
            label = %candidate.label,
            composite,
            "route evaluated"
        );

        RouteEvaluation {
            route_id: candidate.id,
            label: candidate.label.clone(),
            composite_score: composite.clamp(0.0, 1.0),
            objectives,
            eta,
            explanation,
        }
    }

    /// Evaluate and rank multiple candidate routes. Best route first.
    pub fn rank(
        &self,
        candidates: &[CandidateRoute],
        env: &EnvironmentConditions,
    ) -> Vec<RouteEvaluation> {
        let mut evaluations: Vec<RouteEvaluation> =
            candidates.iter().map(|c| self.evaluate(c, env)).collect();

        evaluations.sort_by(|a, b| {
            a.composite_score
                .partial_cmp(&b.composite_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if let Some(best) = evaluations.first() {
            debug!(
                best_route = %best.route_id,
                best_score = best.composite_score,
                candidate_count = evaluations.len(),
                "routes ranked"
            );
        }

        evaluations
    }
}

impl Default for ProbabilisticRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// Normalize travel time to [0, 1] — 1 hour = 0.5, 2 hours = 1.0.
fn normalize_time(seconds: f64) -> f64 {
    (seconds / 7200.0).min(1.0)
}

/// Normalize distance to [0, 1] — 50 km = 0.5, 100 km = 1.0.
fn normalize_distance(metres: f64) -> f64 {
    (metres / 100_000.0).min(1.0)
}

/// Estimate cognitive load from road complexity.
fn estimate_cognitive_load(segments: &[RoadSegment]) -> f64 {
    if segments.is_empty() {
        return 0.0;
    }

    let mut complexity = 0.0;

    for seg in segments {
        if seg.tunnel {
            complexity += 0.15;
        }
        if seg.speed_limit_kmh.unwrap_or(0.0) > 100.0 {
            complexity += 0.1;
        }
        if seg.geometry.len() > 5 {
            complexity += 0.05;
        }
    }

    (complexity / segments.len() as f64).min(1.0)
}

/// Estimate energy consumption (normalized).
fn estimate_energy(total_distance: f64, segments: &[RoadSegment]) -> f64 {
    let base = normalize_distance(total_distance);

    let unpaved_fraction = segments
        .iter()
        .filter(|s| {
            s.surface_type == aurora_core::map::SurfaceType::Gravel
                || s.surface_type == aurora_core::map::SurfaceType::Dirt
        })
        .count() as f64
        / segments.len().max(1) as f64;

    (base * (1.0 + 0.3 * unpaved_fraction)).min(1.0)
}

/// Estimate emissions (proportional to energy with speed penalty).
fn estimate_emissions(total_distance: f64, segments: &[RoadSegment]) -> f64 {
    let base = estimate_energy(total_distance, segments);
    let avg_speed = segments
        .iter()
        .filter_map(|s| s.speed_limit_kmh)
        .sum::<f64>()
        / segments
            .iter()
            .filter(|s| s.speed_limit_kmh.is_some())
            .count()
            .max(1) as f64;

    let speed_factor = if avg_speed > 100.0 {
        1.2
    } else if avg_speed > 60.0 {
        1.0
    } else {
        0.9
    };

    (base * speed_factor).min(1.0)
}

/// Build an explanation graph for a route evaluation.
fn build_explanation(
    candidate: &CandidateRoute,
    objectives: &ObjectiveScores,
    weights: &ObjectiveWeights,
    composite: f64,
) -> ExplanationGraph {
    let contributions = vec![
        ObjectiveContribution {
            name: "Travel Time".into(),
            raw_score: objectives.time,
            weight: weights.time,
            weighted_contribution: objectives.time * weights.time,
        },
        ObjectiveContribution {
            name: "Distance".into(),
            raw_score: objectives.distance,
            weight: weights.distance,
            weighted_contribution: objectives.distance * weights.distance,
        },
        ObjectiveContribution {
            name: "Risk".into(),
            raw_score: objectives.risk,
            weight: weights.risk,
            weighted_contribution: objectives.risk * weights.risk,
        },
        ObjectiveContribution {
            name: "Network Stability".into(),
            raw_score: 1.0 - objectives.stability,
            weight: weights.stability,
            weighted_contribution: (1.0 - objectives.stability) * weights.stability,
        },
        ObjectiveContribution {
            name: "Cognitive Load".into(),
            raw_score: objectives.cognitive_load,
            weight: weights.cognitive_load,
            weighted_contribution: objectives.cognitive_load * weights.cognitive_load,
        },
        ObjectiveContribution {
            name: "Energy".into(),
            raw_score: objectives.energy,
            weight: weights.energy,
            weighted_contribution: objectives.energy * weights.energy,
        },
        ObjectiveContribution {
            name: "Emissions".into(),
            raw_score: objectives.emissions,
            weight: weights.emissions,
            weighted_contribution: objectives.emissions * weights.emissions,
        },
    ];

    // Find top contributing factors.
    let mut sorted_contribs = contributions.clone();
    sorted_contribs.sort_by(|a, b| {
        b.weighted_contribution
            .partial_cmp(&a.weighted_contribution)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let top_factors: Vec<String> = sorted_contribs
        .iter()
        .take(3)
        .map(|c| {
            format!(
                "{} ({:.0}%)",
                c.name,
                c.weighted_contribution / composite.max(0.001) * 100.0
            )
        })
        .collect();

    let root = ExplanationNode {
        label: format!("Route: {}", candidate.label),
        score: composite,
        children: contributions
            .iter()
            .map(|c| ExplanationNode {
                label: c.name.clone(),
                score: c.weighted_contribution,
                children: Vec::new(),
            })
            .collect(),
    };

    ExplanationGraph {
        route_id: candidate.id,
        root,
        contributions,
        summary: format!(
            "Route '{}' scored {:.3}. Top factors: {}",
            candidate.label,
            composite,
            top_factors.join(", ")
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aurora_core::map::{RoadClass, SurfaceType};
    use aurora_core::types::GeoPosition;

    fn make_seg(length_m: f64, speed: f64) -> RoadSegment {
        RoadSegment {
            id: EntityId::new(),
            from_node: EntityId::new(),
            to_node: EntityId::new(),
            geometry: vec![
                GeoPosition {
                    latitude_deg: 32.08,
                    longitude_deg: 34.78,
                    altitude_m: None,
                },
                GeoPosition {
                    latitude_deg: 32.09,
                    longitude_deg: 34.78,
                    altitude_m: None,
                },
            ],
            road_class: RoadClass::Primary,
            one_way: false,
            speed_limit_kmh: Some(speed),
            lane_count: Some(2),
            surface_type: SurfaceType::Asphalt,
            bridge: false,
            tunnel: false,
            toll: false,
            weight_limit_kg: None,
            height_limit_m: None,
            hazmat_restricted: false,
            length_m,
            travel_time_s: Some(length_m / (speed / 3.6)),
        }
    }

    fn make_candidate(label: &str, segments: Vec<RoadSegment>) -> CandidateRoute {
        let histories = segments.iter().map(|_| SegmentHistory::default()).collect();
        CandidateRoute {
            id: EntityId::new(),
            label: label.into(),
            segments,
            segment_histories: histories,
        }
    }

    #[test]
    fn evaluate_single_route() {
        let router = ProbabilisticRouter::new();
        let candidate = make_candidate(
            "Highway A",
            vec![make_seg(5000.0, 90.0), make_seg(3000.0, 70.0)],
        );
        let env = EnvironmentConditions::default();

        let eval = router.evaluate(&candidate, &env);
        assert!(eval.composite_score > 0.0 && eval.composite_score <= 1.0);
        assert!(eval.eta.expected_s > 0.0);
        assert!(!eval.explanation.summary.is_empty());
    }

    #[test]
    fn rank_prefers_shorter_faster() {
        let router = ProbabilisticRouter::new();
        let fast = make_candidate("Fast", vec![make_seg(3000.0, 100.0)]);
        let slow = make_candidate("Slow", vec![make_seg(8000.0, 40.0)]);
        let env = EnvironmentConditions::default();

        let ranked = router.rank(&[slow, fast], &env);
        assert_eq!(ranked.len(), 2);
        assert_eq!(ranked[0].label, "Fast");
    }

    #[test]
    fn risk_affects_ranking() {
        let router = ProbabilisticRouter::with_weights(ObjectiveWeights {
            risk: 0.8,
            time: 0.1,
            distance: 0.1,
            ..Default::default()
        });

        let safe_seg = make_seg(5000.0, 60.0);
        let mut risky_seg = make_seg(4000.0, 80.0);
        risky_seg.tunnel = true;
        risky_seg.bridge = true;

        let safe = CandidateRoute {
            id: EntityId::new(),
            label: "Safe".into(),
            segments: vec![safe_seg],
            segment_histories: vec![SegmentHistory::default()],
        };
        let risky = CandidateRoute {
            id: EntityId::new(),
            label: "Risky".into(),
            segments: vec![risky_seg],
            segment_histories: vec![SegmentHistory {
                accident_count: 9,
                hard_brake_count: 45,
                avg_daily_traffic: 80000,
            }],
        };

        let env = EnvironmentConditions {
            weather_severity: 0.6,
            visibility: 0.4,
            lighting: 0.3,
            surface_wetness: 0.7,
        };

        let ranked = router.rank(&[risky, safe], &env);
        assert_eq!(ranked[0].label, "Safe");
    }

    #[test]
    fn explanation_graph_has_contributions() {
        let router = ProbabilisticRouter::new();
        let candidate = make_candidate("Test", vec![make_seg(2000.0, 60.0)]);
        let env = EnvironmentConditions::default();

        let eval = router.evaluate(&candidate, &env);
        assert_eq!(eval.explanation.contributions.len(), 7);
        assert!(!eval.explanation.root.children.is_empty());
    }

    #[test]
    fn empty_candidates_produces_empty_ranking() {
        let router = ProbabilisticRouter::new();
        let env = EnvironmentConditions::default();
        let ranked = router.rank(&[], &env);
        assert!(ranked.is_empty());
    }
}
