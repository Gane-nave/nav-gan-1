//! Risk engine — computes risk scores for segments, routes, and regions.

use chrono::Utc;
use gane_core::map::RoadSegment;
use gane_core::scoring::{RiskComponent, RiskEntityType, RiskFactor, RiskScore};
use gane_core::types::EntityId;
use tracing::debug;

/// Environmental conditions that affect risk calculations.
#[derive(Debug, Clone)]
pub struct EnvironmentConditions {
    /// Weather severity [0, 1] where 1 = severe storm.
    pub weather_severity: f64,
    /// Visibility [0, 1] where 1 = perfect, 0 = zero visibility.
    pub visibility: f64,
    /// Lighting [0, 1] where 1 = full daylight, 0 = no lighting.
    pub lighting: f64,
    /// Road surface wetness [0, 1].
    pub surface_wetness: f64,
}

impl Default for EnvironmentConditions {
    fn default() -> Self {
        Self {
            weather_severity: 0.0,
            visibility: 1.0,
            lighting: 1.0,
            surface_wetness: 0.0,
        }
    }
}

/// Historical data for a road segment used in risk computation.
#[derive(Debug, Clone, Default)]
pub struct SegmentHistory {
    /// Accident count in the last year.
    pub accident_count: u32,
    /// Hard brake event count in the last month.
    pub hard_brake_count: u32,
    /// Average daily traffic volume.
    pub avg_daily_traffic: u32,
}

/// Risk engine that scores road segments and routes.
pub struct RiskEngine {
    /// Factor weights for the risk model.
    weights: RiskWeights,
}

/// Configurable weights for each risk factor.
#[derive(Debug, Clone)]
pub struct RiskWeights {
    pub accident_history: f64,
    pub hard_brake_density: f64,
    pub weather: f64,
    pub visibility: f64,
    pub lighting: f64,
    pub infrastructure: f64,
    pub curvature: f64,
    pub speed_variance: f64,
    pub congestion: f64,
}

impl Default for RiskWeights {
    fn default() -> Self {
        Self {
            accident_history: 0.20,
            hard_brake_density: 0.15,
            weather: 0.15,
            visibility: 0.10,
            lighting: 0.05,
            infrastructure: 0.10,
            curvature: 0.10,
            speed_variance: 0.05,
            congestion: 0.10,
        }
    }
}

impl RiskEngine {
    pub fn new() -> Self {
        Self {
            weights: RiskWeights::default(),
        }
    }

    pub fn with_weights(weights: RiskWeights) -> Self {
        Self { weights }
    }

    /// Compute risk score for a single road segment.
    pub fn score_segment(
        &self,
        segment: &RoadSegment,
        env: &EnvironmentConditions,
        history: &SegmentHistory,
    ) -> RiskScore {
        let mut components = Vec::new();

        // Accident history factor.
        let accident_score = (history.accident_count as f64 / 10.0).min(1.0);
        components.push(RiskComponent {
            factor: RiskFactor::AccidentHistory,
            score: accident_score,
            weight: self.weights.accident_history,
        });

        // Hard brake density factor.
        let brake_score = (history.hard_brake_count as f64 / 50.0).min(1.0);
        components.push(RiskComponent {
            factor: RiskFactor::HardBrakeDensity,
            score: brake_score,
            weight: self.weights.hard_brake_density,
        });

        // Weather risk.
        components.push(RiskComponent {
            factor: RiskFactor::Weather,
            score: env.weather_severity,
            weight: self.weights.weather,
        });

        // Visibility risk (inverted: low visibility = high risk).
        components.push(RiskComponent {
            factor: RiskFactor::Visibility,
            score: 1.0 - env.visibility,
            weight: self.weights.visibility,
        });

        // Lighting risk (inverted: low lighting = high risk).
        components.push(RiskComponent {
            factor: RiskFactor::Lighting,
            score: 1.0 - env.lighting,
            weight: self.weights.lighting,
        });

        // Infrastructure condition (tunnels, bridges add risk).
        let infra_score = infra_risk(segment);
        components.push(RiskComponent {
            factor: RiskFactor::InfrastructureCondition,
            score: infra_score,
            weight: self.weights.infrastructure,
        });

        // Curvature complexity (estimated from geometry point density).
        let curvature_score = curvature_risk(segment);
        components.push(RiskComponent {
            factor: RiskFactor::CurvatureComplexity,
            score: curvature_score,
            weight: self.weights.curvature,
        });

        // Speed variance risk (higher speed limits = higher risk).
        let speed_score = speed_risk(segment);
        components.push(RiskComponent {
            factor: RiskFactor::SpeedVariance,
            score: speed_score,
            weight: self.weights.speed_variance,
        });

        // Congestion risk (based on surface wetness as proxy for conditions).
        components.push(RiskComponent {
            factor: RiskFactor::Congestion,
            score: env.surface_wetness,
            weight: self.weights.congestion,
        });

        // Weighted sum.
        let total_weight: f64 = components.iter().map(|c| c.weight).sum();
        let weighted_sum: f64 = components.iter().map(|c| c.score * c.weight).sum();
        let score = if total_weight > 0.0 {
            (weighted_sum / total_weight).clamp(0.0, 1.0)
        } else {
            0.0
        };

        debug!(
            segment_id = %segment.id,
            score,
            components = components.len(),
            "segment risk scored"
        );

        RiskScore {
            id: EntityId::new(),
            entity_type: RiskEntityType::Segment,
            entity_id: segment.id,
            score,
            components,
            computed_at: Utc::now(),
        }
    }

    /// Compute aggregate risk score for a route (list of segments).
    pub fn score_route(
        &self,
        segments: &[&RoadSegment],
        env: &EnvironmentConditions,
        histories: &[SegmentHistory],
    ) -> RiskScore {
        if segments.is_empty() {
            return RiskScore {
                id: EntityId::new(),
                entity_type: RiskEntityType::Route,
                entity_id: EntityId::new(),
                score: 0.0,
                components: Vec::new(),
                computed_at: Utc::now(),
            };
        }

        let segment_scores: Vec<RiskScore> = segments
            .iter()
            .zip(histories.iter())
            .map(|(seg, hist)| self.score_segment(seg, env, hist))
            .collect();

        // Route risk = length-weighted average of segment risks.
        let total_length: f64 = segments.iter().map(|s| s.length_m).sum();
        let weighted_risk: f64 = segment_scores
            .iter()
            .zip(segments.iter())
            .map(|(score, seg)| score.score * seg.length_m)
            .sum();

        let route_score = if total_length > 0.0 {
            weighted_risk / total_length
        } else {
            0.0
        };

        // Also find the max segment risk (worst point on route).
        let max_segment_risk = segment_scores
            .iter()
            .map(|s| s.score)
            .fold(0.0_f64, f64::max);

        // Final route risk is blend of average and worst-case.
        let blended = 0.7 * route_score + 0.3 * max_segment_risk;

        debug!(
            route_score,
            max_segment_risk,
            blended,
            segment_count = segments.len(),
            "route risk scored"
        );

        RiskScore {
            id: EntityId::new(),
            entity_type: RiskEntityType::Route,
            entity_id: EntityId::new(),
            score: blended.clamp(0.0, 1.0),
            components: Vec::new(),
            computed_at: Utc::now(),
        }
    }

    /// Classify risk into human-readable level.
    pub fn classify(score: f64) -> RiskLevel {
        match score {
            s if s < 0.2 => RiskLevel::Low,
            s if s < 0.4 => RiskLevel::Moderate,
            s if s < 0.6 => RiskLevel::Elevated,
            s if s < 0.8 => RiskLevel::High,
            _ => RiskLevel::Extreme,
        }
    }
}

impl Default for RiskEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Human-readable risk level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Moderate,
    Elevated,
    High,
    Extreme,
}

/// Infrastructure risk based on segment properties.
fn infra_risk(seg: &RoadSegment) -> f64 {
    let mut risk: f64 = 0.0;
    if seg.bridge {
        risk += 0.3;
    }
    if seg.tunnel {
        risk += 0.4;
    }
    if seg.surface_type == gane_core::map::SurfaceType::Gravel
        || seg.surface_type == gane_core::map::SurfaceType::Dirt
    {
        risk += 0.3;
    }
    risk.min(1.0)
}

/// Curvature risk estimated from geometry density.
fn curvature_risk(seg: &RoadSegment) -> f64 {
    if seg.length_m <= 0.0 {
        return 0.0;
    }
    let points_per_km = (seg.geometry.len() as f64 / seg.length_m) * 1000.0;
    (points_per_km / 10.0).min(1.0)
}

/// Speed-related risk.
fn speed_risk(seg: &RoadSegment) -> f64 {
    let limit = seg.speed_limit_kmh.unwrap_or(50.0);
    (limit / 130.0).min(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gane_core::map::{RoadClass, SurfaceType};
    use gane_core::types::GeoPosition;

    fn make_segment(length_m: f64, speed: f64, tunnel: bool, bridge: bool) -> RoadSegment {
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
            bridge,
            tunnel,
            toll: false,
            weight_limit_kg: None,
            height_limit_m: None,
            hazmat_restricted: false,
            length_m,
            travel_time_s: Some(length_m / (speed / 3.6)),
        }
    }

    #[test]
    fn low_risk_segment_in_good_conditions() {
        let engine = RiskEngine::new();
        let seg = make_segment(500.0, 50.0, false, false);
        let env = EnvironmentConditions::default();
        let history = SegmentHistory::default();

        let score = engine.score_segment(&seg, &env, &history);
        assert!(score.score < 0.3, "expected low risk, got {}", score.score);
        assert_eq!(RiskEngine::classify(score.score), RiskLevel::Low);
    }

    #[test]
    fn high_risk_in_bad_conditions() {
        let engine = RiskEngine::new();
        let seg = make_segment(500.0, 120.0, true, false);
        let env = EnvironmentConditions {
            weather_severity: 0.8,
            visibility: 0.2,
            lighting: 0.1,
            surface_wetness: 0.9,
        };
        let history = SegmentHistory {
            accident_count: 8,
            hard_brake_count: 40,
            avg_daily_traffic: 50000,
        };

        let score = engine.score_segment(&seg, &env, &history);
        assert!(score.score > 0.5, "expected high risk, got {}", score.score);
    }

    #[test]
    fn route_risk_aggregates_segments() {
        let engine = RiskEngine::new();
        let seg1 = make_segment(1000.0, 50.0, false, false);
        let seg2 = make_segment(500.0, 90.0, true, false);
        let env = EnvironmentConditions::default();
        let histories = vec![SegmentHistory::default(), SegmentHistory::default()];

        let route_score = engine.score_route(&[&seg1, &seg2], &env, &histories);
        assert!((0.0..=1.0).contains(&route_score.score));
    }

    #[test]
    fn empty_route_scores_zero() {
        let engine = RiskEngine::new();
        let env = EnvironmentConditions::default();
        let score = engine.score_route(&[], &env, &[]);
        assert_eq!(score.score, 0.0);
    }

    #[test]
    fn risk_classification_boundaries() {
        assert_eq!(RiskEngine::classify(0.0), RiskLevel::Low);
        assert_eq!(RiskEngine::classify(0.19), RiskLevel::Low);
        assert_eq!(RiskEngine::classify(0.2), RiskLevel::Moderate);
        assert_eq!(RiskEngine::classify(0.5), RiskLevel::Elevated);
        assert_eq!(RiskEngine::classify(0.7), RiskLevel::High);
        assert_eq!(RiskEngine::classify(0.9), RiskLevel::Extreme);
    }
}
