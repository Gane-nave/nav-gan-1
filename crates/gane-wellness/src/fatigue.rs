//! Fatigue detection — monitors driving patterns for signs of drowsiness.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Fatigue severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FatigueLevel {
    /// Driver appears alert and focused.
    Alert,
    /// Mild signs of fatigue detected.
    Mild,
    /// Moderate fatigue — recommend a break soon.
    Moderate,
    /// Severe fatigue — immediate break recommended.
    Severe,
    /// Critical fatigue — safety intervention required.
    Critical,
}

/// A single driving behaviour sample for fatigue analysis.
#[derive(Debug, Clone)]
pub struct DrivingSample {
    /// Lateral deviation from lane centre (metres).
    pub lane_deviation: f64,
    /// Steering correction frequency (corrections per minute).
    pub steering_corrections_per_min: f64,
    /// Current speed (m/s).
    pub speed: f64,
    /// Time since last significant input (seconds).
    pub idle_duration_secs: f64,
    /// Timestamp of this sample.
    pub timestamp: Instant,
}

/// Configuration for fatigue detection.
#[derive(Debug, Clone)]
pub struct FatigueConfig {
    /// Lane deviation threshold for mild fatigue (metres).
    pub mild_deviation_threshold: f64,
    /// Lane deviation threshold for moderate fatigue (metres).
    pub moderate_deviation_threshold: f64,
    /// Lane deviation threshold for severe fatigue (metres).
    pub severe_deviation_threshold: f64,
    /// Steering corrections per minute threshold for fatigue.
    pub low_steering_threshold: f64,
    /// Idle duration (seconds) that suggests micro-sleep.
    pub micro_sleep_threshold_secs: f64,
    /// Window size for rolling analysis.
    pub analysis_window: Duration,
    /// Maximum samples to retain.
    pub max_samples: usize,
}

impl Default for FatigueConfig {
    fn default() -> Self {
        Self {
            mild_deviation_threshold: 0.3,
            moderate_deviation_threshold: 0.6,
            severe_deviation_threshold: 1.0,
            low_steering_threshold: 2.0,
            micro_sleep_threshold_secs: 3.0,
            analysis_window: Duration::from_secs(300), // 5 minutes
            max_samples: 500,
        }
    }
}

/// Fatigue detection engine.
pub struct FatigueDetector {
    config: FatigueConfig,
    samples: VecDeque<DrivingSample>,
    current_level: FatigueLevel,
    driving_start: Option<Instant>,
    total_driving_duration: Duration,
}

impl FatigueDetector {
    /// Create a new fatigue detector with the given config.
    pub fn new(config: FatigueConfig) -> Self {
        Self {
            config,
            samples: VecDeque::new(),
            current_level: FatigueLevel::Alert,
            driving_start: None,
            total_driving_duration: Duration::ZERO,
        }
    }

    /// Create with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(FatigueConfig::default())
    }

    /// Start a driving session.
    pub fn start_session(&mut self) {
        self.driving_start = Some(Instant::now());
        self.samples.clear();
        self.current_level = FatigueLevel::Alert;
    }

    /// Add a driving sample and update fatigue assessment.
    pub fn add_sample(&mut self, sample: DrivingSample) -> FatigueLevel {
        // Prune old samples
        let cutoff = sample.timestamp.checked_sub(self.config.analysis_window);
        if let Some(cutoff) = cutoff {
            while self.samples.front().is_some_and(|s| s.timestamp < cutoff) {
                self.samples.pop_front();
            }
        }

        self.samples.push_back(sample);
        if self.samples.len() > self.config.max_samples {
            self.samples.pop_front();
        }

        self.current_level = self.assess();
        self.current_level
    }

    /// Get the current fatigue level.
    pub fn current_level(&self) -> FatigueLevel {
        self.current_level
    }

    /// Get the total driving duration.
    pub fn driving_duration(&self) -> Duration {
        match self.driving_start {
            Some(start) => self.total_driving_duration + start.elapsed(),
            None => self.total_driving_duration,
        }
    }

    /// End the current driving session.
    pub fn end_session(&mut self) {
        if let Some(start) = self.driving_start.take() {
            self.total_driving_duration += start.elapsed();
        }
    }

    /// Get the number of samples in the analysis window.
    pub fn sample_count(&self) -> usize {
        self.samples.len()
    }

    /// Assess fatigue level from current samples.
    fn assess(&self) -> FatigueLevel {
        if self.samples.len() < 3 {
            return FatigueLevel::Alert;
        }

        let avg_deviation = self
            .samples
            .iter()
            .map(|s| s.lane_deviation.abs())
            .sum::<f64>()
            / self.samples.len() as f64;

        let avg_steering = self
            .samples
            .iter()
            .map(|s| s.steering_corrections_per_min)
            .sum::<f64>()
            / self.samples.len() as f64;

        let max_idle = self
            .samples
            .iter()
            .map(|s| s.idle_duration_secs)
            .fold(0.0f64, f64::max);

        // Critical: micro-sleep detected
        if max_idle >= self.config.micro_sleep_threshold_secs {
            return FatigueLevel::Critical;
        }

        // Severe: large lane deviation + low steering activity
        if avg_deviation >= self.config.severe_deviation_threshold {
            return FatigueLevel::Severe;
        }

        // Moderate: significant deviation or very low steering
        if avg_deviation >= self.config.moderate_deviation_threshold
            || avg_steering < self.config.low_steering_threshold * 0.5
        {
            return FatigueLevel::Moderate;
        }

        // Mild: minor deviation or somewhat low steering
        if avg_deviation >= self.config.mild_deviation_threshold
            || avg_steering < self.config.low_steering_threshold
        {
            return FatigueLevel::Mild;
        }

        FatigueLevel::Alert
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_sample(deviation: f64, steering: f64, idle: f64) -> DrivingSample {
        DrivingSample {
            lane_deviation: deviation,
            steering_corrections_per_min: steering,
            speed: 30.0,
            idle_duration_secs: idle,
            timestamp: Instant::now(),
        }
    }

    #[test]
    fn test_starts_alert() {
        let detector = FatigueDetector::with_defaults();
        assert_eq!(detector.current_level(), FatigueLevel::Alert);
    }

    #[test]
    fn test_alert_with_good_driving() {
        let mut detector = FatigueDetector::with_defaults();
        detector.start_session();
        for _ in 0..10 {
            let level = detector.add_sample(make_sample(0.1, 10.0, 0.5));
            assert_eq!(level, FatigueLevel::Alert);
        }
    }

    #[test]
    fn test_mild_fatigue_on_lane_deviation() {
        let mut detector = FatigueDetector::with_defaults();
        detector.start_session();
        // Deviation at 0.35 > mild threshold (0.3)
        for _ in 0..5 {
            detector.add_sample(make_sample(0.35, 10.0, 0.5));
        }
        assert_eq!(detector.current_level(), FatigueLevel::Mild);
    }

    #[test]
    fn test_moderate_fatigue_on_high_deviation() {
        let mut detector = FatigueDetector::with_defaults();
        detector.start_session();
        // Deviation at 0.7 > moderate threshold (0.6)
        for _ in 0..5 {
            detector.add_sample(make_sample(0.7, 10.0, 0.5));
        }
        assert_eq!(detector.current_level(), FatigueLevel::Moderate);
    }

    #[test]
    fn test_severe_fatigue_on_very_high_deviation() {
        let mut detector = FatigueDetector::with_defaults();
        detector.start_session();
        // Deviation at 1.2 > severe threshold (1.0)
        for _ in 0..5 {
            detector.add_sample(make_sample(1.2, 10.0, 0.5));
        }
        assert_eq!(detector.current_level(), FatigueLevel::Severe);
    }

    #[test]
    fn test_critical_on_micro_sleep() {
        let mut detector = FatigueDetector::with_defaults();
        detector.start_session();
        // Idle duration 4.0 > micro-sleep threshold (3.0)
        for _ in 0..5 {
            detector.add_sample(make_sample(0.1, 10.0, 4.0));
        }
        assert_eq!(detector.current_level(), FatigueLevel::Critical);
    }

    #[test]
    fn test_low_steering_triggers_moderate() {
        let mut detector = FatigueDetector::with_defaults();
        detector.start_session();
        // Very low steering corrections (0.5 < threshold * 0.5 = 1.0)
        for _ in 0..5 {
            detector.add_sample(make_sample(0.1, 0.5, 0.5));
        }
        assert_eq!(detector.current_level(), FatigueLevel::Moderate);
    }

    #[test]
    fn test_insufficient_samples_returns_alert() {
        let mut detector = FatigueDetector::with_defaults();
        detector.start_session();
        // Only 2 samples — below minimum of 3
        detector.add_sample(make_sample(1.5, 0.5, 4.0));
        detector.add_sample(make_sample(1.5, 0.5, 4.0));
        assert_eq!(detector.current_level(), FatigueLevel::Alert);
    }

    #[test]
    fn test_sample_count() {
        let mut detector = FatigueDetector::with_defaults();
        detector.start_session();
        for _ in 0..10 {
            detector.add_sample(make_sample(0.1, 10.0, 0.5));
        }
        assert_eq!(detector.sample_count(), 10);
    }

    #[test]
    fn test_fatigue_level_ordering() {
        assert!(FatigueLevel::Alert < FatigueLevel::Mild);
        assert!(FatigueLevel::Mild < FatigueLevel::Moderate);
        assert!(FatigueLevel::Moderate < FatigueLevel::Severe);
        assert!(FatigueLevel::Severe < FatigueLevel::Critical);
    }

    #[test]
    fn test_max_samples_enforced() {
        let config = FatigueConfig {
            max_samples: 5,
            ..Default::default()
        };
        let mut detector = FatigueDetector::new(config);
        detector.start_session();
        for _ in 0..20 {
            detector.add_sample(make_sample(0.1, 10.0, 0.5));
        }
        assert_eq!(detector.sample_count(), 5);
    }
}
