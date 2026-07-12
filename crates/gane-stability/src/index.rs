//! Network stability index — computes a holistic stability score for the road
//! network using congestion entropy, flow balance, and fairness constraints.

use chrono::Utc;
use gane_core::scoring::{StabilityEntityType, StabilityScore};
use gane_core::types::EntityId;
use tracing::{debug, warn};

/// Flow state for a single segment used in stability computation.
#[derive(Debug, Clone)]
pub struct SegmentFlowState {
    pub segment_id: EntityId,
    /// Current speed in km/h.
    pub speed_kmh: f64,
    /// Free-flow speed in km/h.
    pub free_flow_speed_kmh: f64,
    /// Current flow in vehicles per hour.
    pub flow_veh_per_hour: f64,
    /// Segment capacity in vehicles per hour.
    pub capacity_veh_per_hour: f64,
    /// Whether this segment is in a residential zone.
    pub is_residential: bool,
}

/// Configuration for the stability index calculator.
#[derive(Debug, Clone)]
pub struct StabilityConfig {
    /// Weight for speed ratio component.
    pub weight_speed_ratio: f64,
    /// Weight for congestion entropy component.
    pub weight_entropy: f64,
    /// Weight for flow balance component.
    pub weight_balance: f64,
    /// Weight for fairness component.
    pub weight_fairness: f64,
    /// Maximum acceptable flow/capacity ratio for residential segments.
    pub residential_max_utilisation: f64,
}

impl Default for StabilityConfig {
    fn default() -> Self {
        Self {
            weight_speed_ratio: 0.3,
            weight_entropy: 0.25,
            weight_balance: 0.25,
            weight_fairness: 0.2,
            residential_max_utilisation: 0.5,
        }
    }
}

/// Network stability index calculator.
///
/// Computes a composite stability score [0, 1] where 1 = perfectly stable.
/// The score integrates:
/// - Average speed ratio (actual / free-flow)
/// - Congestion entropy (how evenly congestion is distributed)
/// - Flow balance (how evenly flow is distributed relative to capacity)
/// - Fairness (residential zone protection compliance)
pub struct StabilityIndex {
    config: StabilityConfig,
    /// Historical stability scores for trend analysis.
    history: Vec<StabilityScore>,
}

impl StabilityIndex {
    pub fn new() -> Self {
        Self {
            config: StabilityConfig::default(),
            history: Vec::new(),
        }
    }

    pub fn with_config(config: StabilityConfig) -> Self {
        Self {
            config,
            history: Vec::new(),
        }
    }

    /// Compute the network stability score from current segment flow states.
    pub fn compute(&mut self, segments: &[SegmentFlowState]) -> StabilityScore {
        if segments.is_empty() {
            let score = StabilityScore {
                id: EntityId::new(),
                entity_type: StabilityEntityType::Network,
                entity_id: EntityId::new(),
                score: 1.0,
                congestion_entropy: 0.0,
                computed_at: Utc::now(),
            };
            self.history.push(score.clone());
            return score;
        }

        let speed_ratio = self.compute_speed_ratio(segments);
        let entropy = self.compute_congestion_entropy(segments);
        let balance = self.compute_flow_balance(segments);
        let fairness = self.compute_fairness(segments);

        let score = self.config.weight_speed_ratio * speed_ratio
            + self.config.weight_entropy * entropy
            + self.config.weight_balance * balance
            + self.config.weight_fairness * fairness;

        let score = score.clamp(0.0, 1.0);

        debug!(
            speed_ratio,
            entropy, balance, fairness, score, "stability index computed"
        );

        let stability = StabilityScore {
            id: EntityId::new(),
            entity_type: StabilityEntityType::Network,
            entity_id: EntityId::new(),
            score,
            congestion_entropy: entropy,
            computed_at: Utc::now(),
        };

        self.history.push(stability.clone());
        stability
    }

    /// Average speed ratio across all segments.
    /// Returns [0, 1] where 1 = all segments at free-flow speed.
    fn compute_speed_ratio(&self, segments: &[SegmentFlowState]) -> f64 {
        let ratios: Vec<f64> = segments
            .iter()
            .filter(|s| s.free_flow_speed_kmh > 0.0)
            .map(|s| (s.speed_kmh / s.free_flow_speed_kmh).clamp(0.0, 1.0))
            .collect();

        if ratios.is_empty() {
            return 1.0;
        }

        ratios.iter().sum::<f64>() / ratios.len() as f64
    }

    /// Congestion entropy — measures how evenly congestion is distributed.
    /// Returns [0, 1] where 1 = perfectly even distribution (best case).
    ///
    /// Uses normalised Shannon entropy: H / H_max.
    fn compute_congestion_entropy(&self, segments: &[SegmentFlowState]) -> f64 {
        let congestion_levels: Vec<f64> = segments
            .iter()
            .filter(|s| s.free_flow_speed_kmh > 0.0)
            .map(|s| (1.0 - s.speed_kmh / s.free_flow_speed_kmh).clamp(0.0, 1.0))
            .collect();

        if congestion_levels.is_empty() {
            return 1.0;
        }

        let total: f64 = congestion_levels.iter().sum();
        if total <= 0.0 {
            return 1.0; // no congestion = maximum stability
        }

        // Shannon entropy.
        let n = congestion_levels.len() as f64;
        let entropy: f64 = congestion_levels
            .iter()
            .filter(|&&c| c > 0.0)
            .map(|&c| {
                let p = c / total;
                -p * p.ln()
            })
            .sum();

        let max_entropy = n.ln();
        if max_entropy <= 0.0 {
            return 1.0;
        }

        (entropy / max_entropy).clamp(0.0, 1.0)
    }

    /// Flow balance — how evenly flow is distributed relative to capacity.
    /// Returns [0, 1] where 1 = all segments equally utilised.
    fn compute_flow_balance(&self, segments: &[SegmentFlowState]) -> f64 {
        let utilisations: Vec<f64> = segments
            .iter()
            .filter(|s| s.capacity_veh_per_hour > 0.0)
            .map(|s| (s.flow_veh_per_hour / s.capacity_veh_per_hour).clamp(0.0, 1.0))
            .collect();

        if utilisations.is_empty() {
            return 1.0;
        }

        let mean = utilisations.iter().sum::<f64>() / utilisations.len() as f64;
        if mean <= 0.0 {
            return 1.0;
        }

        // Coefficient of variation (lower = more balanced).
        let variance = utilisations
            .iter()
            .map(|&u| (u - mean).powi(2))
            .sum::<f64>()
            / utilisations.len() as f64;
        let cv = variance.sqrt() / mean;

        // Invert: low CV = high balance score.
        (1.0 - cv).clamp(0.0, 1.0)
    }

    /// Fairness — how well residential zones are protected.
    /// Returns [0, 1] where 1 = all residential zones within limits.
    fn compute_fairness(&self, segments: &[SegmentFlowState]) -> f64 {
        let residential: Vec<&SegmentFlowState> =
            segments.iter().filter(|s| s.is_residential).collect();

        if residential.is_empty() {
            return 1.0; // no residential zones to protect
        }

        let compliant = residential
            .iter()
            .filter(|s| {
                if s.capacity_veh_per_hour <= 0.0 {
                    return true;
                }
                let utilisation = s.flow_veh_per_hour / s.capacity_veh_per_hour;
                utilisation <= self.config.residential_max_utilisation
            })
            .count();

        compliant as f64 / residential.len() as f64
    }

    /// Compute stability for a single corridor (group of segments).
    pub fn compute_corridor(
        &mut self,
        corridor_id: EntityId,
        segments: &[SegmentFlowState],
    ) -> StabilityScore {
        if segments.is_empty() {
            return StabilityScore {
                id: EntityId::new(),
                entity_type: StabilityEntityType::Corridor,
                entity_id: corridor_id,
                score: 1.0,
                congestion_entropy: 0.0,
                computed_at: Utc::now(),
            };
        }

        let speed_ratio = self.compute_speed_ratio(segments);
        let entropy = self.compute_congestion_entropy(segments);
        let balance = self.compute_flow_balance(segments);

        let score = (0.4 * speed_ratio + 0.3 * entropy + 0.3 * balance).clamp(0.0, 1.0);

        debug!(
            corridor = %corridor_id,
            score,
            "corridor stability computed"
        );

        StabilityScore {
            id: EntityId::new(),
            entity_type: StabilityEntityType::Corridor,
            entity_id: corridor_id,
            score,
            congestion_entropy: entropy,
            computed_at: Utc::now(),
        }
    }

    /// Get the stability trend (positive = improving, negative = degrading).
    pub fn trend(&self, lookback: usize) -> f64 {
        if self.history.len() < 2 {
            return 0.0;
        }

        let start = if self.history.len() > lookback {
            self.history.len() - lookback
        } else {
            0
        };
        let window = &self.history[start..];

        if window.len() < 2 {
            return 0.0;
        }

        // Simple linear regression slope.
        let n = window.len() as f64;
        let sum_x: f64 = (0..window.len()).map(|i| i as f64).sum();
        let sum_y: f64 = window.iter().map(|s| s.score).sum();
        let sum_xy: f64 = window
            .iter()
            .enumerate()
            .map(|(i, s)| i as f64 * s.score)
            .sum();
        let sum_xx: f64 = (0..window.len()).map(|i| (i as f64).powi(2)).sum();

        let denom = n * sum_xx - sum_x * sum_x;
        if denom.abs() < f64::EPSILON {
            return 0.0;
        }

        (n * sum_xy - sum_x * sum_y) / denom
    }

    /// Classify the stability level.
    pub fn classify(score: f64) -> StabilityLevel {
        match score {
            s if s >= 0.8 => StabilityLevel::Stable,
            s if s >= 0.6 => StabilityLevel::Adequate,
            s if s >= 0.4 => StabilityLevel::Stressed,
            s if s >= 0.2 => StabilityLevel::Degraded,
            _ => StabilityLevel::Critical,
        }
    }

    /// Number of historical stability computations.
    pub fn history_count(&self) -> usize {
        self.history.len()
    }

    /// Get the latest stability score.
    pub fn latest(&self) -> Option<&StabilityScore> {
        self.history.last()
    }

    /// Get all historical scores.
    pub fn history(&self) -> &[StabilityScore] {
        &self.history
    }

    /// Check if stability is degrading and emit a warning.
    pub fn check_degradation(&self, lookback: usize) -> Option<DegradationWarning> {
        let trend = self.trend(lookback);
        let latest = self.latest()?;

        if trend < -0.05 && latest.score < 0.6 {
            warn!(score = latest.score, trend, "network stability degrading");
            Some(DegradationWarning {
                current_score: latest.score,
                trend,
                level: Self::classify(latest.score),
            })
        } else {
            None
        }
    }
}

impl Default for StabilityIndex {
    fn default() -> Self {
        Self::new()
    }
}

/// Human-readable stability level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StabilityLevel {
    Stable,
    Adequate,
    Stressed,
    Degraded,
    Critical,
}

/// Warning issued when stability is degrading.
#[derive(Debug, Clone)]
pub struct DegradationWarning {
    pub current_score: f64,
    pub trend: f64,
    pub level: StabilityLevel,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seg(
        speed: f64,
        free_flow: f64,
        flow: f64,
        capacity: f64,
        residential: bool,
    ) -> SegmentFlowState {
        SegmentFlowState {
            segment_id: EntityId::new(),
            speed_kmh: speed,
            free_flow_speed_kmh: free_flow,
            flow_veh_per_hour: flow,
            capacity_veh_per_hour: capacity,
            is_residential: residential,
        }
    }

    #[test]
    fn perfect_conditions_high_stability() {
        let mut index = StabilityIndex::new();
        let segments = vec![
            seg(100.0, 100.0, 500.0, 2000.0, false),
            seg(80.0, 80.0, 400.0, 1500.0, false),
            seg(60.0, 60.0, 300.0, 1000.0, false),
        ];

        let score = index.compute(&segments);
        assert!(
            score.score > 0.7,
            "expected high stability, got {}",
            score.score
        );
        assert_eq!(
            StabilityIndex::classify(score.score),
            StabilityLevel::Stable
        );
    }

    #[test]
    fn congested_network_lower_than_free_flow() {
        let mut index = StabilityIndex::new();

        let free_segments = vec![
            seg(100.0, 100.0, 500.0, 2000.0, false),
            seg(80.0, 80.0, 400.0, 1500.0, false),
            seg(60.0, 60.0, 300.0, 1000.0, false),
        ];
        let free_score = index.compute(&free_segments);

        let congested_segments = vec![
            seg(20.0, 100.0, 1800.0, 2000.0, false),
            seg(15.0, 80.0, 1400.0, 1500.0, false),
            seg(10.0, 60.0, 950.0, 1000.0, false),
        ];
        let congested_score = index.compute(&congested_segments);

        assert!(
            congested_score.score < free_score.score,
            "congested {} should be < free {}",
            congested_score.score,
            free_score.score
        );
    }

    #[test]
    fn residential_violation_reduces_fairness() {
        let mut index = StabilityIndex::new();

        // Residential segment over the utilisation limit.
        let segments = vec![
            seg(80.0, 100.0, 500.0, 2000.0, false),
            seg(40.0, 60.0, 800.0, 1000.0, true), // utilisation 0.8 > 0.5 limit
        ];

        let score = index.compute(&segments);
        // With fairness violated, score should be lower than if compliant.
        let compliant_segments = vec![
            seg(80.0, 100.0, 500.0, 2000.0, false),
            seg(40.0, 60.0, 300.0, 1000.0, true), // utilisation 0.3 < 0.5 limit
        ];

        let compliant_score = index.compute(&compliant_segments);
        assert!(score.score < compliant_score.score);
    }

    #[test]
    fn empty_segments_full_stability() {
        let mut index = StabilityIndex::new();
        let score = index.compute(&[]);
        assert!((score.score - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn corridor_stability() {
        let mut index = StabilityIndex::new();
        let corridor_id = EntityId::new();

        let segments = vec![
            seg(90.0, 100.0, 800.0, 2000.0, false),
            seg(85.0, 100.0, 900.0, 2000.0, false),
        ];

        let score = index.compute_corridor(corridor_id, &segments);
        assert_eq!(score.entity_type, StabilityEntityType::Corridor);
        assert_eq!(score.entity_id, corridor_id);
        assert!(score.score > 0.5);
    }

    #[test]
    fn stability_trend_detection() {
        let mut index = StabilityIndex::new();

        // Simulate declining stability.
        for i in 0..5 {
            let speed = 100.0 - (i as f64 * 20.0);
            let segments = vec![seg(speed, 100.0, 500.0, 2000.0, false)];
            index.compute(&segments);
        }

        let trend = index.trend(5);
        assert!(trend < 0.0, "expected negative trend, got {}", trend);
    }

    #[test]
    fn stability_trend_improving() {
        let mut index = StabilityIndex::new();

        // Simulate improving stability.
        for i in 0..5 {
            let speed = 20.0 + (i as f64 * 20.0);
            let segments = vec![seg(speed, 100.0, 500.0, 2000.0, false)];
            index.compute(&segments);
        }

        let trend = index.trend(5);
        assert!(trend > 0.0, "expected positive trend, got {}", trend);
    }

    #[test]
    fn classification_boundaries() {
        assert_eq!(StabilityIndex::classify(0.9), StabilityLevel::Stable);
        assert_eq!(StabilityIndex::classify(0.7), StabilityLevel::Adequate);
        assert_eq!(StabilityIndex::classify(0.5), StabilityLevel::Stressed);
        assert_eq!(StabilityIndex::classify(0.3), StabilityLevel::Degraded);
        assert_eq!(StabilityIndex::classify(0.1), StabilityLevel::Critical);
    }

    #[test]
    fn degradation_warning_when_declining() {
        let mut index = StabilityIndex::new();

        // Start with good conditions, then degrade rapidly.
        // This creates both a negative trend AND a low final score.
        let scenarios: Vec<(f64, f64, f64, bool)> = vec![
            // (speed, free_flow, flow, residential)
            (95.0, 100.0, 400.0, false), // good
            (85.0, 100.0, 600.0, false), // slight decline
            (60.0, 100.0, 900.0, true),  // moderate with residential
            (40.0, 100.0, 1200.0, true), // worse
            (20.0, 100.0, 1600.0, true), // bad — residential violated
            (10.0, 100.0, 1900.0, true), // very bad
            (5.0, 100.0, 1950.0, true),  // critical
            (2.0, 100.0, 2000.0, true),  // near gridlock
        ];

        for (speed, free_flow, flow, residential) in &scenarios {
            let segments = vec![
                seg(*speed, *free_flow, *flow, 2000.0, *residential),
                seg(*speed * 0.5, *free_flow, *flow * 0.9, 2000.0, *residential),
            ];
            index.compute(&segments);
        }

        // Verify the trend is negative.
        let trend = index.trend(8);
        assert!(trend < -0.01, "expected negative trend, got {}", trend);

        // Verify the latest score is below warning threshold.
        let latest = index.latest().unwrap();
        assert!(
            latest.score < 0.6,
            "expected score < 0.6, got {}",
            latest.score
        );

        let warning = index.check_degradation(8);
        assert!(warning.is_some());
    }

    #[test]
    fn uneven_congestion_reduces_entropy() {
        let mut index = StabilityIndex::new();

        // Uneven congestion: one segment congested, others free.
        let uneven = vec![
            seg(10.0, 100.0, 500.0, 2000.0, false), // heavily congested
            seg(95.0, 100.0, 500.0, 2000.0, false), // free
            seg(98.0, 100.0, 500.0, 2000.0, false), // free
        ];
        let uneven_score = index.compute(&uneven);

        // Even congestion: all segments moderately congested.
        let even = vec![
            seg(60.0, 100.0, 500.0, 2000.0, false),
            seg(55.0, 100.0, 500.0, 2000.0, false),
            seg(58.0, 100.0, 500.0, 2000.0, false),
        ];
        let even_score = index.compute(&even);

        // Even distribution has higher entropy score.
        assert!(
            even_score.congestion_entropy > uneven_score.congestion_entropy,
            "even entropy {} should be > uneven entropy {}",
            even_score.congestion_entropy,
            uneven_score.congestion_entropy
        );
    }
}
