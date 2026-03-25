/// Rev limiter: maximum RPM protection, fuel cut, ignition retard
/// Phase 209

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LimiterType {
    FuelCut,
    IgnitionRetard,
    Combined,
}

#[derive(Debug, Clone)]
pub struct RevLimiter {
    pub limiter_type: LimiterType,
    pub rpm_limit: f64,
    pub current_rpm: f64,
    pub soft_limit_rpm: f64,
    pub active: bool,
    pub activations_count: u32,
}

impl Default for RevLimiter {
    fn default() -> Self {
        Self::new()
    }
}

impl RevLimiter {
    pub fn new() -> Self {
        Self {
            limiter_type: LimiterType::FuelCut,
            rpm_limit: 7000.0,
            current_rpm: 3000.0,
            soft_limit_rpm: 6500.0,
            active: false,
            activations_count: 0,
        }
    }

    pub fn in_soft_limit(&self) -> bool {
        self.current_rpm >= self.soft_limit_rpm && self.current_rpm < self.rpm_limit
    }

    pub fn at_limit(&self) -> bool {
        self.current_rpm >= self.rpm_limit
    }

    pub fn margin_rpm(&self) -> f64 {
        (self.rpm_limit - self.current_rpm).max(0.0)
    }

    pub fn pct_of_limit(&self) -> f64 {
        if self.rpm_limit <= 0.0 {
            return 0.0;
        }
        (self.current_rpm / self.rpm_limit * 100.0).min(100.0)
    }

    pub fn health_score(&self) -> f64 {
        if self.activations_count > 100 {
            return 70.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_at_limit() {
        let r = RevLimiter::new();
        assert!(!r.at_limit());
    }

    #[test]
    fn test_not_in_soft() {
        let r = RevLimiter::new();
        assert!(!r.in_soft_limit());
    }

    #[test]
    fn test_margin() {
        let r = RevLimiter::new();
        assert!((r.margin_rpm() - 4000.0).abs() < 0.1);
    }

    #[test]
    fn test_pct() {
        let r = RevLimiter::new();
        assert!(r.pct_of_limit() < 50.0);
    }

    #[test]
    fn test_at_limit() {
        let mut r = RevLimiter::new();
        r.current_rpm = 7500.0;
        assert!(r.at_limit());
    }

    #[test]
    fn test_health() {
        let r = RevLimiter::new();
        assert!((r.health_score() - 100.0).abs() < 0.1);
    }
}
