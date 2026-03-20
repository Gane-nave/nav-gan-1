//! Explanation graph — provides human-readable explanations for route ranking decisions.

use aurora_core::types::EntityId;
use serde::{Deserialize, Serialize};

/// Explanation graph for a route evaluation decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationGraph {
    pub route_id: EntityId,
    /// Root node of the explanation tree.
    pub root: ExplanationNode,
    /// Per-objective contribution breakdown.
    pub contributions: Vec<ObjectiveContribution>,
    /// Human-readable summary.
    pub summary: String,
}

/// A node in the explanation tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationNode {
    pub label: String,
    pub score: f64,
    pub children: Vec<ExplanationNode>,
}

/// Contribution of a single objective to the composite score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveContribution {
    pub name: String,
    /// Raw score [0, 1] for this objective.
    pub raw_score: f64,
    /// Weight assigned to this objective.
    pub weight: f64,
    /// raw_score * weight.
    pub weighted_contribution: f64,
}

impl ExplanationGraph {
    /// Get the dominant factor (highest weighted contribution).
    pub fn dominant_factor(&self) -> Option<&ObjectiveContribution> {
        self.contributions.iter().max_by(|a, b| {
            a.weighted_contribution
                .partial_cmp(&b.weighted_contribution)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Get factors above a contribution threshold.
    pub fn significant_factors(&self, threshold: f64) -> Vec<&ObjectiveContribution> {
        self.contributions
            .iter()
            .filter(|c| c.weighted_contribution >= threshold)
            .collect()
    }

    /// Generate a short natural-language explanation.
    pub fn short_explanation(&self) -> String {
        match self.dominant_factor() {
            Some(factor) => format!(
                "Route chosen primarily for {} (contributes {:.0}% of score)",
                factor.name,
                if self.root.score > 0.0 {
                    factor.weighted_contribution / self.root.score * 100.0
                } else {
                    0.0
                }
            ),
            None => "No dominant factor identified".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_graph() -> ExplanationGraph {
        ExplanationGraph {
            route_id: EntityId::new(),
            root: ExplanationNode {
                label: "Route A".into(),
                score: 0.45,
                children: vec![
                    ExplanationNode {
                        label: "Time".into(),
                        score: 0.15,
                        children: Vec::new(),
                    },
                    ExplanationNode {
                        label: "Risk".into(),
                        score: 0.20,
                        children: Vec::new(),
                    },
                ],
            },
            contributions: vec![
                ObjectiveContribution {
                    name: "Travel Time".into(),
                    raw_score: 0.3,
                    weight: 0.35,
                    weighted_contribution: 0.105,
                },
                ObjectiveContribution {
                    name: "Risk".into(),
                    raw_score: 0.5,
                    weight: 0.20,
                    weighted_contribution: 0.10,
                },
                ObjectiveContribution {
                    name: "Distance".into(),
                    raw_score: 0.2,
                    weight: 0.15,
                    weighted_contribution: 0.03,
                },
            ],
            summary: "Route A scored 0.45".into(),
        }
    }

    #[test]
    fn dominant_factor_is_highest_contribution() {
        let graph = make_graph();
        let dominant = graph.dominant_factor().unwrap();
        assert_eq!(dominant.name, "Travel Time");
    }

    #[test]
    fn significant_factors_above_threshold() {
        let graph = make_graph();
        let significant = graph.significant_factors(0.05);
        assert_eq!(significant.len(), 2); // Time (0.105) and Risk (0.10)
    }

    #[test]
    fn short_explanation_format() {
        let graph = make_graph();
        let explanation = graph.short_explanation();
        assert!(explanation.contains("Travel Time"));
        assert!(explanation.contains("primarily"));
    }
}
