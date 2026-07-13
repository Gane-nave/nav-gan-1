//! Stress testing — load generation and performance boundary verification.

use std::collections::VecDeque;

/// Load profile for stress testing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadProfile {
    /// Constant load.
    Constant,
    /// Linearly increasing load.
    Ramp,
    /// Sudden spike in load.
    Spike,
    /// Alternating high/low load.
    Wave,
}

/// A single stress test sample.
#[derive(Debug, Clone)]
pub struct StressSample {
    /// Request count in this interval.
    pub request_count: u64,
    /// Average response time (ms).
    pub avg_response_ms: f64,
    /// Error count.
    pub error_count: u64,
    /// Timestamp offset (seconds from start).
    pub time_offset_secs: f64,
}

/// Stress test configuration.
#[derive(Debug, Clone)]
pub struct StressConfig {
    /// Maximum allowed response time (ms).
    pub max_response_ms: f64,
    /// Maximum allowed error rate (0.0 - 1.0).
    pub max_error_rate: f64,
    /// Number of intervals to keep in sliding window.
    pub window_size: usize,
}

impl Default for StressConfig {
    fn default() -> Self {
        Self {
            max_response_ms: 100.0,
            max_error_rate: 0.05,
            window_size: 10,
        }
    }
}

/// Stress test runner — generates load and tracks performance metrics.
pub struct StressRunner {
    config: StressConfig,
    samples: VecDeque<StressSample>,
    total_requests: u64,
    total_errors: u64,
    peak_response_ms: f64,
}

impl StressRunner {
    /// Create a new stress runner.
    pub fn new(config: StressConfig) -> Self {
        Self {
            config,
            samples: VecDeque::new(),
            total_requests: 0,
            total_errors: 0,
            peak_response_ms: 0.0,
        }
    }

    /// Create with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(StressConfig::default())
    }

    /// Record a stress test sample.
    pub fn record(&mut self, sample: StressSample) {
        self.total_requests += sample.request_count;
        self.total_errors += sample.error_count;
        if sample.avg_response_ms > self.peak_response_ms {
            self.peak_response_ms = sample.avg_response_ms;
        }
        self.samples.push_back(sample);
        while self.samples.len() > self.config.window_size {
            self.samples.pop_front();
        }
    }

    /// Get current error rate (windowed).
    pub fn error_rate(&self) -> f64 {
        let total_req: u64 = self.samples.iter().map(|s| s.request_count).sum();
        let total_err: u64 = self.samples.iter().map(|s| s.error_count).sum();
        if total_req == 0 {
            return 0.0;
        }
        total_err as f64 / total_req as f64
    }

    /// Get current average response time (windowed).
    pub fn avg_response_ms(&self) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }
        let total: f64 = self.samples.iter().map(|s| s.avg_response_ms).sum();
        total / self.samples.len() as f64
    }

    /// Check if the system is within acceptable bounds.
    pub fn within_bounds(&self) -> bool {
        self.avg_response_ms() <= self.config.max_response_ms
            && self.error_rate() <= self.config.max_error_rate
    }

    /// Get peak response time observed.
    pub fn peak_response_ms(&self) -> f64 {
        self.peak_response_ms
    }

    /// Get total requests processed.
    pub fn total_requests(&self) -> u64 {
        self.total_requests
    }

    /// Get total errors.
    pub fn total_errors(&self) -> u64 {
        self.total_errors
    }

    /// Get sample count in current window.
    pub fn window_sample_count(&self) -> usize {
        self.samples.len()
    }

    /// Get throughput (requests per second) based on time range.
    pub fn throughput_rps(&self) -> f64 {
        if self.samples.len() < 2 {
            return 0.0;
        }
        let first = self.samples.front().unwrap().time_offset_secs;
        let last = self.samples.back().unwrap().time_offset_secs;
        let duration = last - first;
        if duration <= 0.0 {
            return 0.0;
        }
        let total_req: u64 = self.samples.iter().map(|s| s.request_count).sum();
        total_req as f64 / duration
    }

    /// Generate a summary report.
    pub fn summary(&self) -> String {
        format!(
            "Requests: {}, Errors: {}, Avg: {:.1}ms, Peak: {:.1}ms, Error rate: {:.2}%, Within bounds: {}",
            self.total_requests,
            self.total_errors,
            self.avg_response_ms(),
            self.peak_response_ms,
            self.error_rate() * 100.0,
            self.within_bounds()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stress_within_bounds() {
        let mut runner = StressRunner::with_defaults();
        runner.record(StressSample {
            request_count: 100,
            avg_response_ms: 50.0,
            error_count: 1,
            time_offset_secs: 1.0,
        });
        assert!(runner.within_bounds()); // 50ms < 100ms, 1% < 5%
    }

    #[test]
    fn test_stress_exceeds_response_time() {
        let mut runner = StressRunner::with_defaults();
        runner.record(StressSample {
            request_count: 100,
            avg_response_ms: 150.0,
            error_count: 0,
            time_offset_secs: 1.0,
        });
        assert!(!runner.within_bounds()); // 150ms > 100ms
    }

    #[test]
    fn test_stress_exceeds_error_rate() {
        let mut runner = StressRunner::with_defaults();
        runner.record(StressSample {
            request_count: 100,
            avg_response_ms: 10.0,
            error_count: 10,
            time_offset_secs: 1.0,
        });
        assert!(!runner.within_bounds()); // 10% > 5%
    }

    #[test]
    fn test_window_eviction() {
        let config = StressConfig {
            window_size: 3,
            ..Default::default()
        };
        let mut runner = StressRunner::new(config);
        for i in 0..5 {
            runner.record(StressSample {
                request_count: 10,
                avg_response_ms: 20.0,
                error_count: 0,
                time_offset_secs: i as f64,
            });
        }
        assert_eq!(runner.window_sample_count(), 3); // only last 3 kept
        assert_eq!(runner.total_requests(), 50); // total is cumulative
    }

    #[test]
    fn test_peak_response() {
        let mut runner = StressRunner::with_defaults();
        runner.record(StressSample {
            request_count: 10,
            avg_response_ms: 50.0,
            error_count: 0,
            time_offset_secs: 1.0,
        });
        runner.record(StressSample {
            request_count: 10,
            avg_response_ms: 200.0,
            error_count: 0,
            time_offset_secs: 2.0,
        });
        runner.record(StressSample {
            request_count: 10,
            avg_response_ms: 30.0,
            error_count: 0,
            time_offset_secs: 3.0,
        });
        assert_eq!(runner.peak_response_ms(), 200.0);
    }

    #[test]
    fn test_throughput() {
        let mut runner = StressRunner::with_defaults();
        runner.record(StressSample {
            request_count: 100,
            avg_response_ms: 10.0,
            error_count: 0,
            time_offset_secs: 0.0,
        });
        runner.record(StressSample {
            request_count: 100,
            avg_response_ms: 10.0,
            error_count: 0,
            time_offset_secs: 1.0,
        });
        let rps = runner.throughput_rps();
        assert!((rps - 200.0).abs() < 0.01); // 200 requests / 1 second
    }

    #[test]
    fn test_empty_runner() {
        let runner = StressRunner::with_defaults();
        assert_eq!(runner.avg_response_ms(), 0.0);
        assert_eq!(runner.error_rate(), 0.0);
        assert!(runner.within_bounds());
        assert_eq!(runner.throughput_rps(), 0.0);
    }

    #[test]
    fn test_summary_format() {
        let mut runner = StressRunner::with_defaults();
        runner.record(StressSample {
            request_count: 50,
            avg_response_ms: 25.0,
            error_count: 2,
            time_offset_secs: 1.0,
        });
        let s = runner.summary();
        assert!(s.contains("Requests: 50"));
        assert!(s.contains("Errors: 2"));
        assert!(s.contains("Within bounds: true"));
    }

    #[test]
    fn test_load_profile_variants() {
        // Verify all variants exist and are distinct
        let profiles = [
            LoadProfile::Constant,
            LoadProfile::Ramp,
            LoadProfile::Spike,
            LoadProfile::Wave,
        ];
        for (i, p) in profiles.iter().enumerate() {
            for (j, q) in profiles.iter().enumerate() {
                if i == j {
                    assert_eq!(p, q);
                } else {
                    assert_ne!(p, q);
                }
            }
        }
    }
}
