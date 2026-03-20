//! Adaptive flow controller — adjusts throughput based on system load.

/// Load level classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadLevel {
    /// System is idle or lightly loaded.
    Low,
    /// System is moderately loaded.
    Medium,
    /// System is heavily loaded.
    High,
    /// System is critically overloaded.
    Critical,
}

impl LoadLevel {
    /// Numeric severity (0 = low, 3 = critical).
    pub fn severity(&self) -> u8 {
        match self {
            LoadLevel::Low => 0,
            LoadLevel::Medium => 1,
            LoadLevel::High => 2,
            LoadLevel::Critical => 3,
        }
    }

    /// Display name.
    pub fn as_str(&self) -> &'static str {
        match self {
            LoadLevel::Low => "low",
            LoadLevel::Medium => "medium",
            LoadLevel::High => "high",
            LoadLevel::Critical => "critical",
        }
    }
}

/// Configuration for the flow controller.
#[derive(Debug, Clone)]
pub struct FlowControlConfig {
    /// Threshold for medium load (0.0 – 1.0).
    pub medium_threshold: f64,
    /// Threshold for high load (0.0 – 1.0).
    pub high_threshold: f64,
    /// Threshold for critical load (0.0 – 1.0).
    pub critical_threshold: f64,
    /// Minimum throughput ratio (0.0 – 1.0) when under critical load.
    pub min_throughput_ratio: f64,
}

impl Default for FlowControlConfig {
    fn default() -> Self {
        Self {
            medium_threshold: 0.5,
            high_threshold: 0.75,
            critical_threshold: 0.9,
            min_throughput_ratio: 0.1,
        }
    }
}

/// Adaptive flow controller.
pub struct FlowController {
    config: FlowControlConfig,
    current_load: f64,
    load_level: LoadLevel,
    throughput_ratio: f64,
    samples: Vec<f64>,
    max_samples: usize,
}

impl FlowController {
    /// Create a new flow controller.
    pub fn new(config: FlowControlConfig) -> Self {
        Self {
            config,
            current_load: 0.0,
            load_level: LoadLevel::Low,
            throughput_ratio: 1.0,
            samples: Vec::new(),
            max_samples: 100,
        }
    }

    /// Update the current load measurement (0.0 – 1.0).
    pub fn update_load(&mut self, load: f64) {
        let load = load.clamp(0.0, 1.0);
        self.current_load = load;

        self.samples.push(load);
        if self.samples.len() > self.max_samples {
            self.samples.remove(0);
        }

        self.load_level = if load >= self.config.critical_threshold {
            LoadLevel::Critical
        } else if load >= self.config.high_threshold {
            LoadLevel::High
        } else if load >= self.config.medium_threshold {
            LoadLevel::Medium
        } else {
            LoadLevel::Low
        };

        self.throughput_ratio = match self.load_level {
            LoadLevel::Low => 1.0,
            LoadLevel::Medium => 0.75,
            LoadLevel::High => 0.5,
            LoadLevel::Critical => self.config.min_throughput_ratio,
        };
    }

    /// Current load level.
    pub fn load_level(&self) -> LoadLevel {
        self.load_level
    }

    /// Current raw load value.
    pub fn current_load(&self) -> f64 {
        self.current_load
    }

    /// Current throughput ratio (1.0 = full speed, lower = throttled).
    pub fn throughput_ratio(&self) -> f64 {
        self.throughput_ratio
    }

    /// Whether the system should accept new work.
    pub fn should_accept(&self) -> bool {
        self.load_level != LoadLevel::Critical
    }

    /// Average load over the sample window.
    pub fn average_load(&self) -> f64 {
        if self.samples.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.samples.iter().sum();
        sum / self.samples.len() as f64
    }

    /// Peak load observed in the sample window.
    pub fn peak_load(&self) -> f64 {
        self.samples.iter().copied().fold(0.0f64, f64::max)
    }

    /// Number of samples collected.
    pub fn sample_count(&self) -> usize {
        self.samples.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let ctrl = FlowController::new(FlowControlConfig::default());
        assert_eq!(ctrl.load_level(), LoadLevel::Low);
        assert!((ctrl.throughput_ratio() - 1.0).abs() < f64::EPSILON);
        assert!(ctrl.should_accept());
    }

    #[test]
    fn test_low_load() {
        let mut ctrl = FlowController::new(FlowControlConfig::default());
        ctrl.update_load(0.3);
        assert_eq!(ctrl.load_level(), LoadLevel::Low);
        assert!((ctrl.throughput_ratio() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_medium_load() {
        let mut ctrl = FlowController::new(FlowControlConfig::default());
        ctrl.update_load(0.6);
        assert_eq!(ctrl.load_level(), LoadLevel::Medium);
        assert!((ctrl.throughput_ratio() - 0.75).abs() < f64::EPSILON);
    }

    #[test]
    fn test_high_load() {
        let mut ctrl = FlowController::new(FlowControlConfig::default());
        ctrl.update_load(0.8);
        assert_eq!(ctrl.load_level(), LoadLevel::High);
        assert!((ctrl.throughput_ratio() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_critical_load() {
        let mut ctrl = FlowController::new(FlowControlConfig::default());
        ctrl.update_load(0.95);
        assert_eq!(ctrl.load_level(), LoadLevel::Critical);
        assert!((ctrl.throughput_ratio() - 0.1).abs() < f64::EPSILON);
        assert!(!ctrl.should_accept());
    }

    #[test]
    fn test_load_clamped() {
        let mut ctrl = FlowController::new(FlowControlConfig::default());
        ctrl.update_load(1.5); // should clamp to 1.0
        assert!((ctrl.current_load() - 1.0).abs() < f64::EPSILON);
        ctrl.update_load(-0.5); // should clamp to 0.0
        assert!((ctrl.current_load() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_average_load() {
        let mut ctrl = FlowController::new(FlowControlConfig::default());
        ctrl.update_load(0.2);
        ctrl.update_load(0.4);
        ctrl.update_load(0.6);
        assert!((ctrl.average_load() - 0.4).abs() < f64::EPSILON);
    }

    #[test]
    fn test_peak_load() {
        let mut ctrl = FlowController::new(FlowControlConfig::default());
        ctrl.update_load(0.3);
        ctrl.update_load(0.9);
        ctrl.update_load(0.5);
        assert!((ctrl.peak_load() - 0.9).abs() < f64::EPSILON);
    }

    #[test]
    fn test_recovery_from_critical() {
        let mut ctrl = FlowController::new(FlowControlConfig::default());
        ctrl.update_load(0.95);
        assert_eq!(ctrl.load_level(), LoadLevel::Critical);
        ctrl.update_load(0.3);
        assert_eq!(ctrl.load_level(), LoadLevel::Low);
        assert!(ctrl.should_accept());
    }

    #[test]
    fn test_load_level_display() {
        assert_eq!(LoadLevel::Low.as_str(), "low");
        assert_eq!(LoadLevel::Critical.as_str(), "critical");
        assert!(LoadLevel::Low.severity() < LoadLevel::Critical.severity());
    }
}
