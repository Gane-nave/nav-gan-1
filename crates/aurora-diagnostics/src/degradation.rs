//! Degradation detection — monitors system performance trends and predicts failures.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Degradation severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DegradationLevel {
    /// No degradation detected.
    Normal,
    /// Minor degradation — monitoring closely.
    Minor,
    /// Moderate degradation — performance noticeably affected.
    Moderate,
    /// Severe degradation — system at risk of failure.
    Severe,
    /// Critical — imminent failure.
    Critical,
}

/// A performance metric sample.
#[derive(Debug, Clone)]
pub struct MetricSample {
    /// Metric name.
    pub name: String,
    /// Metric value.
    pub value: f64,
    /// Timestamp.
    pub timestamp: Instant,
}

/// Degradation trend direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrendDirection {
    /// Improving.
    Improving,
    /// Stable.
    Stable,
    /// Degrading.
    Degrading,
}

/// Configuration for degradation detection.
#[derive(Debug, Clone)]
pub struct DegradationConfig {
    /// Maximum samples to retain per metric.
    pub max_samples: usize,
    /// Analysis window duration.
    pub analysis_window: Duration,
    /// Minor degradation threshold (% performance loss).
    pub minor_threshold: f64,
    /// Moderate degradation threshold.
    pub moderate_threshold: f64,
    /// Severe degradation threshold.
    pub severe_threshold: f64,
    /// Critical degradation threshold.
    pub critical_threshold: f64,
}

impl Default for DegradationConfig {
    fn default() -> Self {
        Self {
            max_samples: 1000,
            analysis_window: Duration::from_secs(3600), // 1 hour
            minor_threshold: 0.10,                      // 10% loss
            moderate_threshold: 0.25,                   // 25% loss
            severe_threshold: 0.50,                     // 50% loss
            critical_threshold: 0.75,                   // 75% loss
        }
    }
}

/// Degradation detector — monitors metric trends and classifies degradation level.
pub struct DegradationDetector {
    config: DegradationConfig,
    metrics: std::collections::HashMap<String, VecDeque<(Instant, f64)>>,
    baselines: std::collections::HashMap<String, f64>,
}

impl DegradationDetector {
    /// Create a new degradation detector.
    pub fn new(config: DegradationConfig) -> Self {
        Self {
            config,
            metrics: std::collections::HashMap::new(),
            baselines: std::collections::HashMap::new(),
        }
    }

    /// Create with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(DegradationConfig::default())
    }

    /// Set baseline value for a metric (expected healthy value).
    pub fn set_baseline(&mut self, metric: &str, value: f64) {
        self.baselines.insert(metric.to_string(), value);
    }

    /// Record a metric sample.
    pub fn record(&mut self, sample: MetricSample) {
        let queue = self.metrics.entry(sample.name.clone()).or_default();
        queue.push_back((sample.timestamp, sample.value));

        // Prune old samples
        while queue.len() > self.config.max_samples {
            queue.pop_front();
        }

        // Set baseline if not already set (first sample)
        self.baselines.entry(sample.name).or_insert(sample.value);
    }

    /// Get the degradation level for a metric.
    pub fn degradation_level(&self, metric: &str) -> DegradationLevel {
        let baseline = match self.baselines.get(metric) {
            Some(b) if *b > 0.0 => *b,
            _ => return DegradationLevel::Normal,
        };

        let current = match self.current_value(metric) {
            Some(v) => v,
            None => return DegradationLevel::Normal,
        };

        let loss = (baseline - current) / baseline;
        if loss < self.config.minor_threshold {
            DegradationLevel::Normal
        } else if loss < self.config.moderate_threshold {
            DegradationLevel::Minor
        } else if loss < self.config.severe_threshold {
            DegradationLevel::Moderate
        } else if loss < self.config.critical_threshold {
            DegradationLevel::Severe
        } else {
            DegradationLevel::Critical
        }
    }

    /// Get the current (most recent) value for a metric.
    pub fn current_value(&self, metric: &str) -> Option<f64> {
        self.metrics
            .get(metric)
            .and_then(|q| q.back())
            .map(|(_, v)| *v)
    }

    /// Get the average value over recent samples.
    pub fn average(&self, metric: &str) -> Option<f64> {
        let queue = self.metrics.get(metric)?;
        if queue.is_empty() {
            return None;
        }
        let sum: f64 = queue.iter().map(|(_, v)| v).sum();
        Some(sum / queue.len() as f64)
    }

    /// Detect trend direction for a metric.
    pub fn trend(&self, metric: &str) -> TrendDirection {
        let queue = match self.metrics.get(metric) {
            Some(q) if q.len() >= 4 => q,
            _ => return TrendDirection::Stable,
        };

        let len = queue.len();
        let first_half: f64 =
            queue.iter().take(len / 2).map(|(_, v)| v).sum::<f64>() / (len / 2) as f64;
        let second_half: f64 =
            queue.iter().skip(len / 2).map(|(_, v)| v).sum::<f64>() / (len - len / 2) as f64;

        let change = (second_half - first_half) / first_half.abs().max(1e-10);
        if change > 0.05 {
            TrendDirection::Improving
        } else if change < -0.05 {
            TrendDirection::Degrading
        } else {
            TrendDirection::Stable
        }
    }

    /// Get the number of tracked metrics.
    pub fn metric_count(&self) -> usize {
        self.metrics.len()
    }

    /// Get sample count for a specific metric.
    pub fn sample_count(&self, metric: &str) -> usize {
        self.metrics.get(metric).map(|q| q.len()).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(name: &str, value: f64) -> MetricSample {
        MetricSample {
            name: name.to_string(),
            value,
            timestamp: Instant::now(),
        }
    }

    #[test]
    fn test_normal_when_at_baseline() {
        let mut detector = DegradationDetector::with_defaults();
        detector.set_baseline("fps", 60.0);
        detector.record(sample("fps", 60.0));
        assert_eq!(detector.degradation_level("fps"), DegradationLevel::Normal);
    }

    #[test]
    fn test_normal_when_above_baseline() {
        let mut detector = DegradationDetector::with_defaults();
        detector.set_baseline("fps", 60.0);
        detector.record(sample("fps", 65.0));
        assert_eq!(detector.degradation_level("fps"), DegradationLevel::Normal);
    }

    #[test]
    fn test_minor_degradation() {
        let mut detector = DegradationDetector::with_defaults();
        detector.set_baseline("fps", 60.0);
        detector.record(sample("fps", 52.0)); // 13% loss
        assert_eq!(detector.degradation_level("fps"), DegradationLevel::Minor);
    }

    #[test]
    fn test_moderate_degradation() {
        let mut detector = DegradationDetector::with_defaults();
        detector.set_baseline("fps", 60.0);
        detector.record(sample("fps", 40.0)); // 33% loss
        assert_eq!(
            detector.degradation_level("fps"),
            DegradationLevel::Moderate
        );
    }

    #[test]
    fn test_severe_degradation() {
        let mut detector = DegradationDetector::with_defaults();
        detector.set_baseline("fps", 60.0);
        detector.record(sample("fps", 25.0)); // 58% loss
        assert_eq!(detector.degradation_level("fps"), DegradationLevel::Severe);
    }

    #[test]
    fn test_critical_degradation() {
        let mut detector = DegradationDetector::with_defaults();
        detector.set_baseline("fps", 60.0);
        detector.record(sample("fps", 10.0)); // 83% loss
        assert_eq!(
            detector.degradation_level("fps"),
            DegradationLevel::Critical
        );
    }

    #[test]
    fn test_unknown_metric_is_normal() {
        let detector = DegradationDetector::with_defaults();
        assert_eq!(
            detector.degradation_level("nonexistent"),
            DegradationLevel::Normal
        );
    }

    #[test]
    fn test_auto_baseline_from_first_sample() {
        let mut detector = DegradationDetector::with_defaults();
        detector.record(sample("latency", 100.0));
        detector.record(sample("latency", 75.0)); // 25% loss from auto-baseline
        assert_eq!(
            detector.degradation_level("latency"),
            DegradationLevel::Moderate
        );
    }

    #[test]
    fn test_average() {
        let mut detector = DegradationDetector::with_defaults();
        detector.record(sample("fps", 60.0));
        detector.record(sample("fps", 40.0));
        detector.record(sample("fps", 50.0));
        let avg = detector.average("fps").unwrap();
        assert!((avg - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_trend_degrading() {
        let mut detector = DegradationDetector::with_defaults();
        // Declining values
        for v in [60.0, 58.0, 55.0, 50.0, 45.0, 40.0] {
            detector.record(sample("fps", v));
        }
        assert_eq!(detector.trend("fps"), TrendDirection::Degrading);
    }

    #[test]
    fn test_trend_improving() {
        let mut detector = DegradationDetector::with_defaults();
        // Increasing values
        for v in [40.0, 45.0, 50.0, 55.0, 58.0, 60.0] {
            detector.record(sample("fps", v));
        }
        assert_eq!(detector.trend("fps"), TrendDirection::Improving);
    }

    #[test]
    fn test_trend_stable() {
        let mut detector = DegradationDetector::with_defaults();
        for _ in 0..6 {
            detector.record(sample("fps", 60.0));
        }
        assert_eq!(detector.trend("fps"), TrendDirection::Stable);
    }

    #[test]
    fn test_trend_insufficient_data() {
        let mut detector = DegradationDetector::with_defaults();
        detector.record(sample("fps", 60.0));
        assert_eq!(detector.trend("fps"), TrendDirection::Stable);
    }

    #[test]
    fn test_metric_count() {
        let mut detector = DegradationDetector::with_defaults();
        detector.record(sample("fps", 60.0));
        detector.record(sample("latency", 10.0));
        assert_eq!(detector.metric_count(), 2);
    }

    #[test]
    fn test_sample_count() {
        let mut detector = DegradationDetector::with_defaults();
        for _ in 0..5 {
            detector.record(sample("fps", 60.0));
        }
        assert_eq!(detector.sample_count("fps"), 5);
        assert_eq!(detector.sample_count("nonexistent"), 0);
    }

    #[test]
    fn test_degradation_ordering() {
        assert!(DegradationLevel::Normal < DegradationLevel::Minor);
        assert!(DegradationLevel::Minor < DegradationLevel::Moderate);
        assert!(DegradationLevel::Moderate < DegradationLevel::Severe);
        assert!(DegradationLevel::Severe < DegradationLevel::Critical);
    }
}
