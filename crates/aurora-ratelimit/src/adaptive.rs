//! Adaptive throttling — dynamically adjusts rate limits based on system load and error rates.

/// Throttle decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThrottleDecision {
    /// Allow the request.
    Allow,
    /// Throttle (delay) the request.
    Throttle,
    /// Reject the request.
    Reject,
}

/// Adaptive throttle configuration.
#[derive(Debug, Clone)]
pub struct AdaptiveConfig {
    /// Base requests per second.
    pub base_rps: f64,
    /// Minimum allowed RPS (floor).
    pub min_rps: f64,
    /// Maximum allowed RPS (ceiling).
    pub max_rps: f64,
    /// Error rate threshold to start throttling (0.0–1.0).
    pub throttle_threshold: f64,
    /// Error rate threshold to start rejecting (0.0–1.0).
    pub reject_threshold: f64,
    /// Smoothing factor for exponential moving average (0.0–1.0).
    pub ema_alpha: f64,
}

impl AdaptiveConfig {
    /// Create a new adaptive config with sensible defaults.
    pub fn new(base_rps: f64) -> Self {
        Self {
            base_rps,
            min_rps: base_rps * 0.1,
            max_rps: base_rps * 2.0,
            throttle_threshold: 0.05,
            reject_threshold: 0.20,
            ema_alpha: 0.3,
        }
    }
}

/// Adaptive throttle controller.
pub struct AdaptiveThrottle {
    config: AdaptiveConfig,
    /// Current effective RPS limit.
    current_rps: f64,
    /// Exponential moving average of error rate.
    error_rate_ema: f64,
    /// Requests in current measurement window.
    window_requests: u64,
    /// Errors in current measurement window.
    window_errors: u64,
    /// Total requests processed.
    total_requests: u64,
    /// Total errors recorded.
    total_errors: u64,
    /// Total throttled requests.
    total_throttled: u64,
    /// Total rejected requests.
    total_rejected: u64,
    /// Window start time (epoch millis).
    window_start_ms: u64,
    /// Measurement window duration (millis).
    window_duration_ms: u64,
}

impl AdaptiveThrottle {
    /// Create a new adaptive throttle.
    pub fn new(config: AdaptiveConfig, now_ms: u64) -> Self {
        let rps = config.base_rps;
        Self {
            config,
            current_rps: rps,
            error_rate_ema: 0.0,
            window_requests: 0,
            window_errors: 0,
            total_requests: 0,
            total_errors: 0,
            total_throttled: 0,
            total_rejected: 0,
            window_start_ms: now_ms,
            window_duration_ms: 1000,
        }
    }

    /// Evaluate whether a request should be allowed, throttled, or rejected.
    pub fn evaluate(&mut self, now_ms: u64) -> ThrottleDecision {
        self.maybe_rotate_window(now_ms);
        self.total_requests += 1;
        self.window_requests += 1;

        if self.error_rate_ema >= self.config.reject_threshold {
            self.total_rejected += 1;
            ThrottleDecision::Reject
        } else if self.error_rate_ema >= self.config.throttle_threshold {
            self.total_throttled += 1;
            ThrottleDecision::Throttle
        } else {
            ThrottleDecision::Allow
        }
    }

    /// Record a successful request completion.
    pub fn record_success(&mut self) {
        // Success doesn't change error counters
    }

    /// Record a request failure/error.
    pub fn record_error(&mut self) {
        self.window_errors += 1;
        self.total_errors += 1;
    }

    /// Rotate the measurement window and recalculate EMA.
    fn maybe_rotate_window(&mut self, now_ms: u64) {
        if now_ms < self.window_start_ms + self.window_duration_ms {
            return;
        }
        // Calculate how many windows have elapsed
        let elapsed = now_ms - self.window_start_ms;
        let windows_elapsed = (elapsed / self.window_duration_ms) as u32;

        // Process the first window with actual data
        let window_error_rate = if self.window_requests > 0 {
            self.window_errors as f64 / self.window_requests as f64
        } else {
            0.0
        };
        let alpha = self.config.ema_alpha;
        self.error_rate_ema = alpha * window_error_rate + (1.0 - alpha) * self.error_rate_ema;
        self.adjust_rps();

        // Decay EMA for intermediate empty windows (no requests)
        for _ in 1..windows_elapsed {
            self.error_rate_ema *= 1.0 - alpha;
            self.adjust_rps();
        }

        // Reset window
        self.window_requests = 0;
        self.window_errors = 0;
        self.window_start_ms = now_ms;
    }

    /// Adjust effective RPS based on error rate EMA.
    fn adjust_rps(&mut self) {
        if self.error_rate_ema >= self.config.reject_threshold {
            // Aggressively reduce
            self.current_rps = (self.current_rps * 0.5).max(self.config.min_rps);
        } else if self.error_rate_ema >= self.config.throttle_threshold {
            // Gently reduce
            self.current_rps = (self.current_rps * 0.9).max(self.config.min_rps);
        } else {
            // Gradually increase toward base
            self.current_rps = (self.current_rps * 1.1).min(self.config.max_rps);
        }
    }

    /// Current effective RPS limit.
    pub fn current_rps(&self) -> f64 {
        self.current_rps
    }

    /// Current error rate EMA.
    pub fn error_rate(&self) -> f64 {
        self.error_rate_ema
    }

    /// Total requests processed.
    pub fn total_requests(&self) -> u64 {
        self.total_requests
    }

    /// Total errors recorded.
    pub fn total_errors(&self) -> u64 {
        self.total_errors
    }

    /// Total throttled requests.
    pub fn throttled_count(&self) -> u64 {
        self.total_throttled
    }

    /// Total rejected requests.
    pub fn rejected_count(&self) -> u64 {
        self.total_rejected
    }

    /// Force update the error rate (for testing).
    pub fn set_error_rate(&mut self, rate: f64) {
        self.error_rate_ema = rate;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_allow_under_threshold() {
        let config = AdaptiveConfig::new(100.0);
        let mut throttle = AdaptiveThrottle::new(config, 0);

        let decision = throttle.evaluate(0);
        assert_eq!(decision, ThrottleDecision::Allow);
    }

    #[test]
    fn test_adaptive_throttle_above_threshold() {
        let config = AdaptiveConfig::new(100.0);
        let mut throttle = AdaptiveThrottle::new(config, 0);

        throttle.set_error_rate(0.10); // Above throttle_threshold (0.05)
        let decision = throttle.evaluate(0);
        assert_eq!(decision, ThrottleDecision::Throttle);
    }

    #[test]
    fn test_adaptive_reject_above_threshold() {
        let config = AdaptiveConfig::new(100.0);
        let mut throttle = AdaptiveThrottle::new(config, 0);

        throttle.set_error_rate(0.25); // Above reject_threshold (0.20)
        let decision = throttle.evaluate(0);
        assert_eq!(decision, ThrottleDecision::Reject);
    }

    #[test]
    fn test_adaptive_rps_reduction_on_errors() {
        let config = AdaptiveConfig::new(100.0);
        let mut throttle = AdaptiveThrottle::new(config, 0);

        // Simulate a window with high error rate
        for _ in 0..10 {
            throttle.evaluate(0);
            throttle.record_error();
        }

        // Rotate window
        throttle.evaluate(1100);

        // RPS should have decreased
        assert!(throttle.current_rps() < 100.0);
    }

    #[test]
    fn test_adaptive_rps_recovery() {
        let config = AdaptiveConfig::new(100.0);
        let mut throttle = AdaptiveThrottle::new(config, 0);

        // Good window
        for _ in 0..10 {
            throttle.evaluate(0);
            throttle.record_success();
        }

        // Rotate
        throttle.evaluate(1100);

        // RPS should increase (up to max)
        assert!(throttle.current_rps() >= 100.0);
    }

    #[test]
    fn test_adaptive_counters() {
        let config = AdaptiveConfig::new(100.0);
        let mut throttle = AdaptiveThrottle::new(config, 0);

        throttle.evaluate(0);
        throttle.evaluate(0);
        throttle.record_error();

        assert_eq!(throttle.total_requests(), 2);
        assert_eq!(throttle.total_errors(), 1);
    }

    #[test]
    fn test_adaptive_min_rps_floor() {
        let config = AdaptiveConfig::new(100.0);
        let mut throttle = AdaptiveThrottle::new(config, 0);

        // Force massive error rate
        throttle.set_error_rate(0.99);

        // Run many windows
        let mut t = 0u64;
        for _ in 0..100 {
            throttle.evaluate(t);
            t += 1100;
        }

        // Should not go below min_rps (10.0)
        assert!(throttle.current_rps() >= 10.0);
    }
}
