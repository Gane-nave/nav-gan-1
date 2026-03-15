//! Data reduction — algorithms for reducing data volume at the edge.
//!
//! Implements sampling, deduplication, delta encoding, and spatial
//! aggregation to minimize bandwidth usage.

use aurora_core::types::EntityId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::debug;

/// Data reduction strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReductionStrategy {
    /// Time-based sampling: keep every Nth sample.
    TimeSampling,
    /// Threshold-based: only transmit when value changes by > threshold.
    ChangeThreshold,
    /// Spatial aggregation: combine nearby points.
    SpatialAggregation,
    /// Delta encoding: transmit only differences.
    DeltaEncoding,
    /// Min/max/avg summary over a window.
    WindowSummary,
}

/// A data sample to be reduced.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSample {
    pub id: EntityId,
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub latitude: f64,
    pub longitude: f64,
    pub source: String,
}

/// Configuration for a reduction strategy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReductionConfig {
    pub strategy: ReductionStrategy,
    /// Sampling interval (for TimeSampling): keep 1 out of N.
    pub sampling_rate: u32,
    /// Change threshold (for ChangeThreshold): minimum delta to transmit.
    pub change_threshold: f64,
    /// Spatial radius in meters (for SpatialAggregation).
    pub spatial_radius_m: f64,
    /// Window size in samples (for WindowSummary).
    pub window_size: usize,
}

impl Default for ReductionConfig {
    fn default() -> Self {
        Self {
            strategy: ReductionStrategy::ChangeThreshold,
            sampling_rate: 10,
            change_threshold: 0.01,
            spatial_radius_m: 50.0,
            window_size: 60,
        }
    }
}

/// Data reducer — applies reduction strategies to incoming data streams.
pub struct DataReducer {
    config: ReductionConfig,
    /// Last transmitted value (for ChangeThreshold).
    last_value: Option<f64>,
    /// Sample counter (for TimeSampling).
    sample_counter: u64,
    /// Window buffer (for WindowSummary).
    window_buffer: Vec<f64>,
    /// Total samples received.
    total_received: u64,
    /// Total samples emitted (after reduction).
    total_emitted: u64,
}

impl DataReducer {
    pub fn new(config: ReductionConfig) -> Self {
        Self {
            config,
            last_value: None,
            sample_counter: 0,
            window_buffer: Vec::new(),
            total_received: 0,
            total_emitted: 0,
        }
    }

    /// Process a sample. Returns Some(output) if the sample should be transmitted.
    pub fn process(&mut self, sample: &DataSample) -> Option<ReducedOutput> {
        self.total_received += 1;
        self.sample_counter += 1;

        match self.config.strategy {
            ReductionStrategy::TimeSampling => self.time_sampling(sample),
            ReductionStrategy::ChangeThreshold => self.change_threshold(sample),
            ReductionStrategy::DeltaEncoding => self.delta_encoding(sample),
            ReductionStrategy::WindowSummary => self.window_summary(sample),
            ReductionStrategy::SpatialAggregation => {
                // Spatial aggregation is a batch operation; pass-through here.
                self.total_emitted += 1;
                Some(ReducedOutput {
                    value: sample.value,
                    timestamp: sample.timestamp,
                    reduction_type: ReductionStrategy::SpatialAggregation,
                    original_samples: 1,
                })
            }
        }
    }

    fn time_sampling(&mut self, sample: &DataSample) -> Option<ReducedOutput> {
        if self.sample_counter % self.config.sampling_rate as u64 == 0 {
            self.total_emitted += 1;
            Some(ReducedOutput {
                value: sample.value,
                timestamp: sample.timestamp,
                reduction_type: ReductionStrategy::TimeSampling,
                original_samples: self.config.sampling_rate as u64,
            })
        } else {
            None
        }
    }

    fn change_threshold(&mut self, sample: &DataSample) -> Option<ReducedOutput> {
        let should_emit = match self.last_value {
            Some(last) => (sample.value - last).abs() > self.config.change_threshold,
            None => true, // Always emit the first sample.
        };

        if should_emit {
            self.last_value = Some(sample.value);
            self.total_emitted += 1;
            Some(ReducedOutput {
                value: sample.value,
                timestamp: sample.timestamp,
                reduction_type: ReductionStrategy::ChangeThreshold,
                original_samples: 1,
            })
        } else {
            None
        }
    }

    fn delta_encoding(&mut self, sample: &DataSample) -> Option<ReducedOutput> {
        let delta = match self.last_value {
            Some(last) => sample.value - last,
            None => sample.value,
        };
        self.last_value = Some(sample.value);
        self.total_emitted += 1;

        Some(ReducedOutput {
            value: delta,
            timestamp: sample.timestamp,
            reduction_type: ReductionStrategy::DeltaEncoding,
            original_samples: 1,
        })
    }

    fn window_summary(&mut self, sample: &DataSample) -> Option<ReducedOutput> {
        self.window_buffer.push(sample.value);

        if self.window_buffer.len() >= self.config.window_size {
            let count = self.window_buffer.len() as f64;
            let avg = self.window_buffer.iter().sum::<f64>() / count;
            let original = self.window_buffer.len() as u64;
            self.window_buffer.clear();
            self.total_emitted += 1;

            Some(ReducedOutput {
                value: avg,
                timestamp: sample.timestamp,
                reduction_type: ReductionStrategy::WindowSummary,
                original_samples: original,
            })
        } else {
            None
        }
    }

    /// Get the reduction ratio (emitted / received). Lower = more reduction.
    pub fn reduction_ratio(&self) -> f64 {
        if self.total_received == 0 {
            return 1.0;
        }
        self.total_emitted as f64 / self.total_received as f64
    }

    /// Total samples received.
    pub fn total_received(&self) -> u64 {
        self.total_received
    }

    /// Total samples emitted.
    pub fn total_emitted(&self) -> u64 {
        self.total_emitted
    }

    /// Get current strategy.
    pub fn strategy(&self) -> ReductionStrategy {
        self.config.strategy
    }

    /// Reset internal state.
    pub fn reset(&mut self) {
        self.last_value = None;
        self.sample_counter = 0;
        self.window_buffer.clear();
    }
}

/// Output of a reduction operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReducedOutput {
    pub value: f64,
    pub timestamp: DateTime<Utc>,
    pub reduction_type: ReductionStrategy,
    pub original_samples: u64,
}

/// Batch spatial aggregation — groups nearby samples.
pub fn spatial_aggregate(samples: &[DataSample], radius_m: f64) -> Vec<ReducedOutput> {
    if samples.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut used = vec![false; samples.len()];

    for i in 0..samples.len() {
        if used[i] {
            continue;
        }
        used[i] = true;

        let mut group_sum = samples[i].value;
        let mut group_count = 1u64;

        for j in (i + 1)..samples.len() {
            if used[j] {
                continue;
            }
            let dist = haversine_m(
                samples[i].latitude,
                samples[i].longitude,
                samples[j].latitude,
                samples[j].longitude,
            );
            if dist <= radius_m {
                used[j] = true;
                group_sum += samples[j].value;
                group_count += 1;
            }
        }

        result.push(ReducedOutput {
            value: group_sum / group_count as f64,
            timestamp: samples[i].timestamp,
            reduction_type: ReductionStrategy::SpatialAggregation,
            original_samples: group_count,
        });
    }

    debug!(
        input = samples.len(),
        output = result.len(),
        "spatial aggregation complete"
    );
    result
}

fn haversine_m(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6_371_000.0;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    2.0 * r * a.sqrt().asin()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_sample(value: f64) -> DataSample {
        DataSample {
            id: EntityId::new(),
            timestamp: Utc::now(),
            value,
            latitude: 32.0,
            longitude: 34.0,
            source: "test".to_string(),
        }
    }

    fn make_geo_sample(value: f64, lat: f64, lon: f64) -> DataSample {
        DataSample {
            id: EntityId::new(),
            timestamp: Utc::now(),
            value,
            latitude: lat,
            longitude: lon,
            source: "test".to_string(),
        }
    }

    #[test]
    fn time_sampling_keeps_every_nth() {
        let config = ReductionConfig {
            strategy: ReductionStrategy::TimeSampling,
            sampling_rate: 5,
            ..Default::default()
        };
        let mut reducer = DataReducer::new(config);

        let mut emitted = 0;
        for i in 0..20 {
            if reducer.process(&make_sample(i as f64)).is_some() {
                emitted += 1;
            }
        }
        assert_eq!(emitted, 4); // 5, 10, 15, 20
        assert!((reducer.reduction_ratio() - 0.2).abs() < 0.01);
    }

    #[test]
    fn change_threshold_filters_small_changes() {
        let config = ReductionConfig {
            strategy: ReductionStrategy::ChangeThreshold,
            change_threshold: 1.0,
            ..Default::default()
        };
        let mut reducer = DataReducer::new(config);

        assert!(reducer.process(&make_sample(10.0)).is_some()); // First always emitted.
        assert!(reducer.process(&make_sample(10.5)).is_none()); // Change < 1.0.
        assert!(reducer.process(&make_sample(10.9)).is_none()); // Still < 1.0.
        assert!(reducer.process(&make_sample(11.5)).is_some()); // Change > 1.0.
    }

    #[test]
    fn delta_encoding_emits_differences() {
        let config = ReductionConfig {
            strategy: ReductionStrategy::DeltaEncoding,
            ..Default::default()
        };
        let mut reducer = DataReducer::new(config);

        let r1 = reducer.process(&make_sample(100.0)).unwrap();
        assert!((r1.value - 100.0).abs() < f64::EPSILON); // First = full value.

        let r2 = reducer.process(&make_sample(103.0)).unwrap();
        assert!((r2.value - 3.0).abs() < f64::EPSILON); // Delta = 3.

        let r3 = reducer.process(&make_sample(101.0)).unwrap();
        assert!((r3.value - (-2.0)).abs() < f64::EPSILON); // Delta = -2.
    }

    #[test]
    fn window_summary_emits_average() {
        let config = ReductionConfig {
            strategy: ReductionStrategy::WindowSummary,
            window_size: 4,
            ..Default::default()
        };
        let mut reducer = DataReducer::new(config);

        assert!(reducer.process(&make_sample(10.0)).is_none());
        assert!(reducer.process(&make_sample(20.0)).is_none());
        assert!(reducer.process(&make_sample(30.0)).is_none());
        let result = reducer.process(&make_sample(40.0)).unwrap();
        assert!((result.value - 25.0).abs() < f64::EPSILON); // avg(10,20,30,40) = 25
        assert_eq!(result.original_samples, 4);
    }

    #[test]
    fn spatial_aggregation_groups_nearby() {
        let samples = vec![
            make_geo_sample(10.0, 32.0000, 34.0000),
            make_geo_sample(20.0, 32.0001, 34.0001), // ~14m away
            make_geo_sample(30.0, 32.0100, 34.0100), // ~1.4km away
        ];

        let result = spatial_aggregate(&samples, 100.0);
        assert_eq!(result.len(), 2); // First two grouped, third separate.

        // First group average.
        assert!((result[0].value - 15.0).abs() < f64::EPSILON);
        assert_eq!(result[0].original_samples, 2);

        // Second group is just the distant point.
        assert!((result[1].value - 30.0).abs() < f64::EPSILON);
        assert_eq!(result[1].original_samples, 1);
    }

    #[test]
    fn spatial_aggregation_empty_input() {
        let result = spatial_aggregate(&[], 100.0);
        assert!(result.is_empty());
    }

    #[test]
    fn reduction_ratio_no_samples() {
        let reducer = DataReducer::new(ReductionConfig::default());
        assert!((reducer.reduction_ratio() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn reset_clears_state() {
        let config = ReductionConfig {
            strategy: ReductionStrategy::ChangeThreshold,
            change_threshold: 1.0,
            ..Default::default()
        };
        let mut reducer = DataReducer::new(config);
        reducer.process(&make_sample(10.0));
        reducer.process(&make_sample(10.5)); // Filtered.

        reducer.reset();
        // After reset, first sample should be emitted again.
        assert!(reducer.process(&make_sample(10.5)).is_some());
    }

    #[test]
    fn all_samples_in_one_spatial_group() {
        let samples = vec![
            make_geo_sample(10.0, 32.0000, 34.0000),
            make_geo_sample(20.0, 32.0000, 34.0000),
            make_geo_sample(30.0, 32.0000, 34.0000),
        ];
        let result = spatial_aggregate(&samples, 100.0);
        assert_eq!(result.len(), 1);
        assert!((result[0].value - 20.0).abs() < f64::EPSILON);
    }
}
