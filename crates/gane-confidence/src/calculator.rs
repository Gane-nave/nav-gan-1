//! Confidence calculator — computes ETA distributions and confidence/volatility indices.

use gane_core::route::EtaDistribution;
use gane_core::types::EntityId;
use tracing::debug;

/// Input data for a single route segment's travel time estimate.
#[derive(Debug, Clone)]
pub struct SegmentEstimate {
    /// Expected travel time in seconds.
    pub expected_s: f64,
    /// Standard deviation of travel time in seconds.
    pub std_dev_s: f64,
    /// Risk score [0, 1] for this segment.
    pub risk_score: f64,
    /// Historical reliability [0, 1] for this segment.
    pub reliability: f64,
}

/// Calculator for confidence metrics.
pub struct ConfidenceCalculator {
    /// Z-score for 5th percentile (~-1.645).
    z_p5: f64,
    /// Z-score for 95th percentile (~1.645).
    z_p95: f64,
    /// Delay threshold in seconds (default 300 = 5 min).
    delay_threshold_s: f64,
}

impl ConfidenceCalculator {
    pub fn new() -> Self {
        Self {
            z_p5: -1.645,
            z_p95: 1.645,
            delay_threshold_s: 300.0,
        }
    }

    /// Set the delay threshold for probability calculations.
    pub fn with_delay_threshold(mut self, threshold_s: f64) -> Self {
        self.delay_threshold_s = threshold_s;
        self
    }

    /// Compute ETA distribution for a route from segment estimates.
    ///
    /// Assumes segment travel times are independent (sum of normals).
    pub fn compute_eta(&self, route_id: EntityId, segments: &[SegmentEstimate]) -> EtaDistribution {
        if segments.is_empty() {
            return EtaDistribution {
                route_id,
                expected_s: 0.0,
                std_dev_s: 0.0,
                p5_s: 0.0,
                p50_s: 0.0,
                p95_s: 0.0,
                on_time_probability: 1.0,
                delay_probability: 0.0,
                blockage_probability: 0.0,
                confidence: 1.0,
                volatility: 0.0,
            };
        }

        // Sum of independent normal distributions.
        let expected_s: f64 = segments.iter().map(|s| s.expected_s).sum();
        // Variance of sum of independent vars = sum of variances.
        let total_variance: f64 = segments.iter().map(|s| s.std_dev_s * s.std_dev_s).sum();
        let std_dev_s = total_variance.sqrt();

        // Percentiles assuming normal distribution.
        let p5_s = (expected_s + self.z_p5 * std_dev_s).max(0.0);
        let p50_s = expected_s; // median = mean for normal
        let p95_s = expected_s + self.z_p95 * std_dev_s;

        // Probability of delay (arrival > expected + threshold).
        let delay_probability = if std_dev_s > 0.0 {
            let z = self.delay_threshold_s / std_dev_s;
            approx_normal_cdf_complement(z)
        } else {
            0.0
        };

        // Blockage probability based on max segment risk.
        let max_risk = segments
            .iter()
            .map(|s| s.risk_score)
            .fold(0.0_f64, f64::max);
        let blockage_probability = (max_risk * 0.3).min(1.0);

        // On-time probability (arrival within p95 window).
        let on_time_probability = 1.0 - delay_probability;

        // Confidence index: based on segment reliability and std_dev relative to expected.
        let confidence = self.compute_confidence(segments, expected_s, std_dev_s);

        // Volatility index: how much the ETA might swing.
        let volatility = self.compute_volatility(segments, expected_s, std_dev_s);

        debug!(
            route_id = %route_id,
            expected_s,
            std_dev_s,
            confidence,
            volatility,
            "ETA distribution computed"
        );

        EtaDistribution {
            route_id,
            expected_s,
            std_dev_s,
            p5_s,
            p50_s,
            p95_s,
            on_time_probability,
            delay_probability,
            blockage_probability,
            confidence,
            volatility,
        }
    }

    /// Confidence index [0, 1] — how much we trust the ETA estimate.
    fn compute_confidence(
        &self,
        segments: &[SegmentEstimate],
        expected_s: f64,
        std_dev_s: f64,
    ) -> f64 {
        if expected_s <= 0.0 {
            return 1.0;
        }

        // Factor 1: coefficient of variation (lower = more confident).
        let cv = std_dev_s / expected_s;
        let cv_confidence = (1.0 - cv * 2.0).clamp(0.0, 1.0);

        // Factor 2: average segment reliability.
        let avg_reliability =
            segments.iter().map(|s| s.reliability).sum::<f64>() / segments.len() as f64;

        // Factor 3: segment count (more segments = slightly less confident due to accumulation).
        let count_factor = (1.0 - (segments.len() as f64 - 1.0) * 0.01).clamp(0.5, 1.0);

        // Weighted combination.
        let confidence = 0.4 * cv_confidence + 0.4 * avg_reliability + 0.2 * count_factor;
        confidence.clamp(0.0, 1.0)
    }

    /// Volatility index [0, 1] — how much the ETA might change.
    fn compute_volatility(
        &self,
        segments: &[SegmentEstimate],
        expected_s: f64,
        std_dev_s: f64,
    ) -> f64 {
        if expected_s <= 0.0 {
            return 0.0;
        }

        // Coefficient of variation as primary volatility measure.
        let cv = std_dev_s / expected_s;

        // Risk contribution to volatility.
        let avg_risk = segments.iter().map(|s| s.risk_score).sum::<f64>() / segments.len() as f64;

        // Blend CV and risk.
        let volatility = 0.6 * (cv * 3.0).min(1.0) + 0.4 * avg_risk;
        volatility.clamp(0.0, 1.0)
    }

    /// Compare two routes and return which has better confidence-adjusted ETA.
    pub fn compare_routes(a: &EtaDistribution, b: &EtaDistribution) -> RouteComparison {
        let a_adjusted = a.expected_s / a.confidence.max(0.01);
        let b_adjusted = b.expected_s / b.confidence.max(0.01);

        if a_adjusted < b_adjusted {
            RouteComparison::PreferA
        } else if b_adjusted < a_adjusted {
            RouteComparison::PreferB
        } else {
            RouteComparison::Equivalent
        }
    }
}

impl Default for ConfidenceCalculator {
    fn default() -> Self {
        Self::new()
    }
}

/// Route comparison result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteComparison {
    PreferA,
    PreferB,
    Equivalent,
}

/// Approximate P(Z > z) for standard normal distribution.
/// Uses the Abramowitz and Stegun approximation.
fn approx_normal_cdf_complement(z: f64) -> f64 {
    if z < -6.0 {
        return 1.0;
    }
    if z > 6.0 {
        return 0.0;
    }

    let t = 1.0 / (1.0 + 0.2316419 * z.abs());
    let d = 0.3989422804014327; // 1/sqrt(2*pi)
    let p = d * (-z * z / 2.0).exp();

    let poly = t
        * (0.319381530
            + t * (-0.356563782 + t * (1.781477937 + t * (-1.821255978 + t * 1.330274429))));

    if z >= 0.0 {
        p * poly
    } else {
        1.0 - p * poly
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_segment(expected: f64, std_dev: f64, risk: f64) -> SegmentEstimate {
        SegmentEstimate {
            expected_s: expected,
            std_dev_s: std_dev,
            risk_score: risk,
            reliability: 0.8,
        }
    }

    #[test]
    fn empty_route_eta() {
        let calc = ConfidenceCalculator::new();
        let eta = calc.compute_eta(EntityId::new(), &[]);
        assert_eq!(eta.expected_s, 0.0);
        assert_eq!(eta.confidence, 1.0);
        assert_eq!(eta.volatility, 0.0);
    }

    #[test]
    fn single_segment_eta() {
        let calc = ConfidenceCalculator::new();
        let segs = vec![make_segment(120.0, 15.0, 0.1)];
        let eta = calc.compute_eta(EntityId::new(), &segs);

        assert!((eta.expected_s - 120.0).abs() < 0.01);
        assert!((eta.std_dev_s - 15.0).abs() < 0.01);
        assert!(eta.p5_s < eta.expected_s);
        assert!(eta.p95_s > eta.expected_s);
        assert!(eta.confidence > 0.5);
        assert!((0.0..=1.0).contains(&eta.volatility));
    }

    #[test]
    fn multi_segment_variance_propagation() {
        let calc = ConfidenceCalculator::new();
        let segs = vec![
            make_segment(100.0, 10.0, 0.1),
            make_segment(200.0, 20.0, 0.2),
            make_segment(150.0, 15.0, 0.15),
        ];
        let eta = calc.compute_eta(EntityId::new(), &segs);

        assert!((eta.expected_s - 450.0).abs() < 0.01);
        assert!((eta.std_dev_s - 725.0_f64.sqrt()).abs() < 0.01);
    }

    #[test]
    fn high_risk_increases_blockage_probability() {
        let calc = ConfidenceCalculator::new();
        let low_risk = vec![make_segment(100.0, 10.0, 0.1)];
        let high_risk = vec![make_segment(100.0, 10.0, 0.9)];

        let eta_low = calc.compute_eta(EntityId::new(), &low_risk);
        let eta_high = calc.compute_eta(EntityId::new(), &high_risk);

        assert!(eta_high.blockage_probability > eta_low.blockage_probability);
    }

    #[test]
    fn route_comparison() {
        let fast_confident = EtaDistribution {
            route_id: EntityId::new(),
            expected_s: 300.0,
            std_dev_s: 20.0,
            p5_s: 267.0,
            p50_s: 300.0,
            p95_s: 333.0,
            on_time_probability: 0.95,
            delay_probability: 0.05,
            blockage_probability: 0.01,
            confidence: 0.9,
            volatility: 0.1,
        };

        let slow_uncertain = EtaDistribution {
            route_id: EntityId::new(),
            expected_s: 350.0,
            std_dev_s: 80.0,
            p5_s: 218.0,
            p50_s: 350.0,
            p95_s: 482.0,
            on_time_probability: 0.7,
            delay_probability: 0.3,
            blockage_probability: 0.15,
            confidence: 0.5,
            volatility: 0.6,
        };

        assert_eq!(
            ConfidenceCalculator::compare_routes(&fast_confident, &slow_uncertain),
            RouteComparison::PreferA
        );
    }

    #[test]
    fn normal_cdf_complement_sanity() {
        let p0 = approx_normal_cdf_complement(0.0);
        assert!((p0 - 0.5).abs() < 0.01);

        let p2 = approx_normal_cdf_complement(2.0);
        assert!((p2 - 0.0228).abs() < 0.01);

        let pn2 = approx_normal_cdf_complement(-2.0);
        assert!((pn2 - 0.9772).abs() < 0.01);
    }
}
