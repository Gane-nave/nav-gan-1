//! Traffic demand and congestion forecasting — short-term traffic prediction,
//! delay propagation modelling, and network collapse early warning.

use chrono::{DateTime, Utc};
use gane_core::types::EntityId;
use std::collections::HashMap;
use tracing::{debug, warn};

/// A historical traffic sample used for forecasting.
#[derive(Debug, Clone)]
pub struct TrafficSample {
    pub segment_id: EntityId,
    pub timestamp: DateTime<Utc>,
    /// Observed speed in km/h.
    pub speed_kmh: f64,
    /// Observed flow in vehicles/hour.
    pub flow_veh_per_hour: f64,
    /// Free-flow speed for the segment.
    pub free_flow_speed_kmh: f64,
}

/// A short-term traffic prediction for a segment.
#[derive(Debug, Clone)]
pub struct TrafficPrediction {
    pub segment_id: EntityId,
    /// Predicted speed in km/h.
    pub predicted_speed_kmh: f64,
    /// Predicted congestion ratio [0, 1] (1 = fully congested).
    pub predicted_congestion: f64,
    /// Confidence in the prediction [0, 1].
    pub confidence: f64,
    /// How far ahead this prediction is (in seconds).
    pub horizon_s: u64,
    pub predicted_at: DateTime<Utc>,
}

/// Delay propagation estimate for a segment.
#[derive(Debug, Clone)]
pub struct DelayPropagation {
    pub source_segment: EntityId,
    pub affected_segments: Vec<EntityId>,
    /// Estimated additional delay in seconds for each affected segment.
    pub delay_s: Vec<f64>,
    pub propagation_probability: f64,
}

/// Network collapse warning level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollapseWarningLevel {
    /// Network operating normally.
    Normal,
    /// Early signs of stress — monitor closely.
    Watch,
    /// Significant congestion building — prepare mitigations.
    Warning,
    /// Network approaching gridlock — activate emergency measures.
    Critical,
}

/// Configuration for the traffic forecaster.
#[derive(Debug, Clone)]
pub struct ForecastConfig {
    /// Number of recent samples to use for prediction.
    pub lookback_samples: usize,
    /// Congestion ratio above which a segment contributes to collapse risk.
    pub collapse_congestion_threshold: f64,
    /// Fraction of segments congested that triggers collapse warning.
    pub collapse_fraction_watch: f64,
    pub collapse_fraction_warning: f64,
    pub collapse_fraction_critical: f64,
    /// Exponential smoothing alpha for speed prediction.
    pub smoothing_alpha: f64,
}

impl Default for ForecastConfig {
    fn default() -> Self {
        Self {
            lookback_samples: 10,
            collapse_congestion_threshold: 0.6,
            collapse_fraction_watch: 0.2,
            collapse_fraction_warning: 0.4,
            collapse_fraction_critical: 0.6,
            smoothing_alpha: 0.3,
        }
    }
}

/// Traffic forecaster — predicts near-future congestion and detects network collapse risk.
pub struct TrafficForecaster {
    config: ForecastConfig,
    /// Historical samples per segment (ring buffer of recent observations).
    history: HashMap<EntityId, Vec<TrafficSample>>,
    /// Segment adjacency for delay propagation (segment → downstream neighbours).
    adjacency: HashMap<EntityId, Vec<EntityId>>,
}

impl TrafficForecaster {
    pub fn new() -> Self {
        Self {
            config: ForecastConfig::default(),
            history: HashMap::new(),
            adjacency: HashMap::new(),
        }
    }

    pub fn with_config(config: ForecastConfig) -> Self {
        Self {
            config,
            ..Self::new()
        }
    }

    /// Register downstream neighbours for delay propagation.
    pub fn set_adjacency(&mut self, segment_id: EntityId, downstream: Vec<EntityId>) {
        self.adjacency.insert(segment_id, downstream);
    }

    /// Ingest a traffic sample.
    pub fn record(&mut self, sample: TrafficSample) {
        let history = self.history.entry(sample.segment_id).or_default();
        history.push(sample);

        // Keep only the most recent samples.
        let max = self.config.lookback_samples;
        if history.len() > max * 2 {
            let start = history.len() - max;
            *history = history[start..].to_vec();
        }
    }

    /// Predict short-term traffic for a segment.
    pub fn predict(&self, segment_id: &EntityId, horizon_s: u64) -> Option<TrafficPrediction> {
        let samples = self.history.get(segment_id)?;
        if samples.is_empty() {
            return None;
        }

        let alpha = self.config.smoothing_alpha;

        // Exponential moving average of speed.
        let mut ema_speed = samples[0].speed_kmh;
        for sample in &samples[1..] {
            ema_speed = alpha * sample.speed_kmh + (1.0 - alpha) * ema_speed;
        }

        // Trend: difference between last EMA and previous.
        let trend = if samples.len() >= 2 {
            let prev_speed = samples[samples.len() - 2].speed_kmh;
            let curr_speed = samples[samples.len() - 1].speed_kmh;
            curr_speed - prev_speed
        } else {
            0.0
        };

        // Simple linear extrapolation: speed + trend * (horizon / sample_interval).
        let sample_interval_s = if samples.len() >= 2 {
            let last = samples.last().unwrap().timestamp;
            let prev = samples[samples.len() - 2].timestamp;
            (last - prev).num_seconds().max(1) as f64
        } else {
            60.0
        };

        let steps = horizon_s as f64 / sample_interval_s;
        let predicted_speed = (ema_speed + trend * steps).max(0.0);

        // Free-flow from latest sample.
        let free_flow = samples.last().unwrap().free_flow_speed_kmh;
        let congestion = if free_flow > 0.0 {
            (1.0 - predicted_speed / free_flow).clamp(0.0, 1.0)
        } else {
            0.0
        };

        // Confidence decreases with horizon and increases with sample count.
        let sample_confidence =
            (samples.len() as f64 / self.config.lookback_samples as f64).min(1.0);
        let horizon_decay = (-0.001 * horizon_s as f64).exp();
        let confidence = (sample_confidence * horizon_decay).clamp(0.0, 1.0);

        debug!(
            segment = %segment_id,
            predicted_speed,
            congestion,
            confidence,
            "traffic prediction"
        );

        Some(TrafficPrediction {
            segment_id: *segment_id,
            predicted_speed_kmh: predicted_speed,
            predicted_congestion: congestion,
            confidence,
            horizon_s,
            predicted_at: Utc::now(),
        })
    }

    /// Estimate delay propagation from a congested source segment.
    pub fn estimate_delay_propagation(&self, source: &EntityId) -> Option<DelayPropagation> {
        let samples = self.history.get(source)?;
        let latest = samples.last()?;

        if latest.free_flow_speed_kmh <= 0.0 {
            return None;
        }

        let congestion = 1.0 - latest.speed_kmh / latest.free_flow_speed_kmh;
        if congestion < self.config.collapse_congestion_threshold {
            return None; // not congested enough to propagate
        }

        let downstream = self.adjacency.get(source)?;
        if downstream.is_empty() {
            return None;
        }

        // Each downstream segment gets a fraction of the delay, decaying with distance.
        let source_delay_s = (congestion * 120.0).max(0.0); // up to 2 min delay
        let delays: Vec<f64> = downstream
            .iter()
            .enumerate()
            .map(|(i, _)| source_delay_s * (0.7_f64).powi(i as i32 + 1))
            .collect();

        let probability = congestion.clamp(0.0, 1.0);

        Some(DelayPropagation {
            source_segment: *source,
            affected_segments: downstream.clone(),
            delay_s: delays,
            propagation_probability: probability,
        })
    }

    /// Assess network collapse risk based on fraction of congested segments.
    pub fn assess_collapse_risk(&self) -> CollapseWarningLevel {
        if self.history.is_empty() {
            return CollapseWarningLevel::Normal;
        }

        let total = self.history.len();
        let congested = self
            .history
            .values()
            .filter(|samples| {
                samples.last().map_or(false, |s| {
                    s.free_flow_speed_kmh > 0.0
                        && (1.0 - s.speed_kmh / s.free_flow_speed_kmh)
                            > self.config.collapse_congestion_threshold
                })
            })
            .count();

        let fraction = congested as f64 / total as f64;

        let level = if fraction >= self.config.collapse_fraction_critical {
            CollapseWarningLevel::Critical
        } else if fraction >= self.config.collapse_fraction_warning {
            CollapseWarningLevel::Warning
        } else if fraction >= self.config.collapse_fraction_watch {
            CollapseWarningLevel::Watch
        } else {
            CollapseWarningLevel::Normal
        };

        if level != CollapseWarningLevel::Normal {
            warn!(
                congested,
                total,
                fraction,
                level = ?level,
                "network collapse risk elevated"
            );
        }

        level
    }

    /// Number of segments with historical data.
    pub fn tracked_segments(&self) -> usize {
        self.history.len()
    }
}

impl Default for TrafficForecaster {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn sample(seg: EntityId, speed: f64, free_flow: f64, offset_s: i64) -> TrafficSample {
        TrafficSample {
            segment_id: seg,
            timestamp: Utc::now() + Duration::seconds(offset_s),
            speed_kmh: speed,
            flow_veh_per_hour: speed * 20.0, // rough approximation
            free_flow_speed_kmh: free_flow,
        }
    }

    #[test]
    fn predict_stable_traffic() {
        let mut forecaster = TrafficForecaster::new();
        let seg = EntityId::new();

        // Feed consistent 80 km/h samples.
        for i in 0..5 {
            forecaster.record(sample(seg, 80.0, 100.0, i * 60));
        }

        let pred = forecaster.predict(&seg, 300).unwrap();
        assert!((pred.predicted_speed_kmh - 80.0).abs() < 5.0);
        assert!(pred.predicted_congestion < 0.3);
        assert!(pred.confidence > 0.3);
    }

    #[test]
    fn predict_deteriorating_traffic() {
        let mut forecaster = TrafficForecaster::new();
        let seg = EntityId::new();

        // Speed decreasing over time.
        for i in 0..5 {
            let speed = 80.0 - i as f64 * 10.0;
            forecaster.record(sample(seg, speed, 100.0, i * 60));
        }

        let pred = forecaster.predict(&seg, 300).unwrap();
        // Should predict further slowdown.
        assert!(pred.predicted_speed_kmh < 50.0);
        assert!(pred.predicted_congestion > 0.3);
    }

    #[test]
    fn no_prediction_without_data() {
        let forecaster = TrafficForecaster::new();
        let seg = EntityId::new();
        assert!(forecaster.predict(&seg, 300).is_none());
    }

    #[test]
    fn collapse_risk_normal_when_no_congestion() {
        let mut forecaster = TrafficForecaster::new();

        for _ in 0..5 {
            let seg = EntityId::new();
            forecaster.record(sample(seg, 90.0, 100.0, 0));
        }

        assert_eq!(
            forecaster.assess_collapse_risk(),
            CollapseWarningLevel::Normal
        );
    }

    #[test]
    fn collapse_risk_escalates_with_congestion() {
        let config = ForecastConfig {
            collapse_congestion_threshold: 0.5,
            collapse_fraction_watch: 0.2,
            collapse_fraction_warning: 0.4,
            collapse_fraction_critical: 0.6,
            ..Default::default()
        };
        let mut forecaster = TrafficForecaster::with_config(config);

        // 10 segments: 7 congested, 3 free.
        for _ in 0..7 {
            let seg = EntityId::new();
            forecaster.record(sample(seg, 20.0, 100.0, 0)); // 80% congested
        }
        for _ in 0..3 {
            let seg = EntityId::new();
            forecaster.record(sample(seg, 90.0, 100.0, 0)); // free
        }

        assert_eq!(
            forecaster.assess_collapse_risk(),
            CollapseWarningLevel::Critical
        );
    }

    #[test]
    fn delay_propagation_from_congested_segment() {
        let mut forecaster = TrafficForecaster::new();
        let source = EntityId::new();
        let d1 = EntityId::new();
        let d2 = EntityId::new();

        forecaster.set_adjacency(source, vec![d1, d2]);
        forecaster.record(sample(source, 20.0, 100.0, 0)); // heavily congested

        let prop = forecaster.estimate_delay_propagation(&source).unwrap();
        assert_eq!(prop.source_segment, source);
        assert_eq!(prop.affected_segments.len(), 2);
        assert!(prop.delay_s[0] > prop.delay_s[1]); // decay with distance
        assert!(prop.propagation_probability > 0.5);
    }

    #[test]
    fn no_propagation_from_free_flowing_segment() {
        let mut forecaster = TrafficForecaster::new();
        let source = EntityId::new();
        let d1 = EntityId::new();

        forecaster.set_adjacency(source, vec![d1]);
        forecaster.record(sample(source, 90.0, 100.0, 0)); // free flowing

        assert!(forecaster.estimate_delay_propagation(&source).is_none());
    }

    #[test]
    fn sample_history_bounded() {
        let config = ForecastConfig {
            lookback_samples: 5,
            ..Default::default()
        };
        let mut forecaster = TrafficForecaster::with_config(config);
        let seg = EntityId::new();

        // Add 20 samples — should trim to ~5.
        for i in 0..20 {
            forecaster.record(sample(seg, 80.0, 100.0, i * 60));
        }

        let history = forecaster.history.get(&seg).unwrap();
        assert!(history.len() <= 10); // trimmed when > 2*lookback
    }
}
