//! Watermark monitoring for ring buffers — track high/low water levels.

/// Monitors fill levels and triggers alerts at configurable thresholds.
#[derive(Debug, Clone)]
pub struct WatermarkMonitor {
    high_threshold: f64,
    low_threshold: f64,
    current_fill: f64,
    high_breaches: u64,
    low_breaches: u64,
    samples: u64,
    peak_fill: f64,
    total_fill_sum: f64,
    state: WatermarkState,
}

/// The current watermark state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WatermarkState {
    /// Fill is between low and high thresholds.
    Normal,
    /// Fill exceeded high threshold.
    High,
    /// Fill dropped below low threshold.
    Low,
}

impl WatermarkMonitor {
    /// Create a new monitor with high and low thresholds (0.0 to 1.0).
    pub fn new(high_threshold: f64, low_threshold: f64) -> Self {
        let high = high_threshold.clamp(0.0, 1.0);
        let low = low_threshold.clamp(0.0, high);
        Self {
            high_threshold: high,
            low_threshold: low,
            current_fill: 0.0,
            high_breaches: 0,
            low_breaches: 0,
            samples: 0,
            peak_fill: 0.0,
            total_fill_sum: 0.0,
            state: WatermarkState::Normal,
        }
    }

    /// Update the monitor with a new fill ratio sample.
    /// Returns the new state.
    pub fn sample(&mut self, fill_ratio: f64) -> WatermarkState {
        let fill = fill_ratio.clamp(0.0, 1.0);
        self.current_fill = fill;
        self.samples = self.samples.saturating_add(1);
        self.total_fill_sum += fill;

        if fill > self.peak_fill {
            self.peak_fill = fill;
        }

        let old_state = self.state;
        if fill >= self.high_threshold {
            self.state = WatermarkState::High;
            if old_state != WatermarkState::High {
                self.high_breaches = self.high_breaches.saturating_add(1);
            }
        } else if fill <= self.low_threshold {
            self.state = WatermarkState::Low;
            if old_state != WatermarkState::Low {
                self.low_breaches = self.low_breaches.saturating_add(1);
            }
        } else {
            self.state = WatermarkState::Normal;
        }
        self.state
    }

    /// Current fill ratio.
    pub fn current_fill(&self) -> f64 {
        self.current_fill
    }

    /// Current state.
    pub fn state(&self) -> WatermarkState {
        self.state
    }

    /// Number of times high threshold was breached.
    pub fn high_breaches(&self) -> u64 {
        self.high_breaches
    }

    /// Number of times low threshold was breached.
    pub fn low_breaches(&self) -> u64 {
        self.low_breaches
    }

    /// Total samples taken.
    pub fn samples(&self) -> u64 {
        self.samples
    }

    /// Peak fill ratio observed.
    pub fn peak_fill(&self) -> f64 {
        self.peak_fill
    }

    /// Average fill ratio across all samples.
    pub fn avg_fill(&self) -> f64 {
        if self.samples == 0 {
            return 0.0;
        }
        self.total_fill_sum / self.samples as f64
    }

    /// High threshold setting.
    pub fn high_threshold(&self) -> f64 {
        self.high_threshold
    }

    /// Low threshold setting.
    pub fn low_threshold(&self) -> f64 {
        self.low_threshold
    }

    /// Whether the monitor is currently in a breach state.
    pub fn is_breached(&self) -> bool {
        self.state != WatermarkState::Normal
    }

    /// Reset all counters but keep thresholds.
    pub fn reset(&mut self) {
        self.current_fill = 0.0;
        self.high_breaches = 0;
        self.low_breaches = 0;
        self.samples = 0;
        self.peak_fill = 0.0;
        self.total_fill_sum = 0.0;
        self.state = WatermarkState::Normal;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_monitor() {
        let m = WatermarkMonitor::new(0.8, 0.2);
        assert!((m.high_threshold() - 0.8).abs() < f64::EPSILON);
        assert!((m.low_threshold() - 0.2).abs() < f64::EPSILON);
        assert_eq!(m.state(), WatermarkState::Normal);
    }

    #[test]
    fn test_high_breach() {
        let mut m = WatermarkMonitor::new(0.8, 0.2);
        let state = m.sample(0.9);
        assert_eq!(state, WatermarkState::High);
        assert_eq!(m.high_breaches(), 1);
        assert!(m.is_breached());
    }

    #[test]
    fn test_low_breach() {
        let mut m = WatermarkMonitor::new(0.8, 0.2);
        let state = m.sample(0.1);
        assert_eq!(state, WatermarkState::Low);
        assert_eq!(m.low_breaches(), 1);
    }

    #[test]
    fn test_normal_no_breach() {
        let mut m = WatermarkMonitor::new(0.8, 0.2);
        let state = m.sample(0.5);
        assert_eq!(state, WatermarkState::Normal);
        assert_eq!(m.high_breaches(), 0);
        assert_eq!(m.low_breaches(), 0);
        assert!(!m.is_breached());
    }

    #[test]
    fn test_repeated_high_counts_once() {
        let mut m = WatermarkMonitor::new(0.8, 0.2);
        m.sample(0.85);
        m.sample(0.90);
        m.sample(0.95);
        assert_eq!(m.high_breaches(), 1); // only counted on transition
    }

    #[test]
    fn test_transition_back_and_forth() {
        let mut m = WatermarkMonitor::new(0.8, 0.2);
        m.sample(0.9); // high
        m.sample(0.5); // normal
        m.sample(0.9); // high again
        assert_eq!(m.high_breaches(), 2);
    }

    #[test]
    fn test_peak_fill() {
        let mut m = WatermarkMonitor::new(0.8, 0.2);
        m.sample(0.3);
        m.sample(0.7);
        m.sample(0.5);
        assert!((m.peak_fill() - 0.7).abs() < f64::EPSILON);
    }

    #[test]
    fn test_avg_fill() {
        let mut m = WatermarkMonitor::new(0.8, 0.2);
        m.sample(0.2);
        m.sample(0.4);
        m.sample(0.6);
        // avg = (0.2 + 0.4 + 0.6) / 3 = 0.4
        assert!((m.avg_fill() - 0.4).abs() < 1e-10);
    }

    #[test]
    fn test_reset() {
        let mut m = WatermarkMonitor::new(0.8, 0.2);
        m.sample(0.9);
        m.reset();
        assert_eq!(m.state(), WatermarkState::Normal);
        assert_eq!(m.high_breaches(), 0);
        assert_eq!(m.samples(), 0);
    }

    #[test]
    fn test_threshold_clamping() {
        let m = WatermarkMonitor::new(1.5, -0.1);
        assert!((m.high_threshold() - 1.0).abs() < f64::EPSILON);
        assert!((m.low_threshold() - 0.0).abs() < f64::EPSILON);
    }
}
