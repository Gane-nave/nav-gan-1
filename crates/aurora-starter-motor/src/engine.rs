/// Starter motor monitoring: cranking speed, battery draw, start reliability
/// Phase 174

#[derive(Debug, Clone)]
pub struct StarterMotor {
    pub cranking_rpm: f64,
    pub draw_amps: f64,
    pub start_time_ms: u64,
    pub attempts: u32,
    pub successful_starts: u32,
}

impl Default for StarterMotor {
    fn default() -> Self {
        Self::new()
    }
}

impl StarterMotor {
    pub fn new() -> Self {
        Self {
            cranking_rpm: 200.0,
            draw_amps: 150.0,
            start_time_ms: 800,
            attempts: 100,
            successful_starts: 100,
        }
    }

    pub fn cranking_ok(&self) -> bool {
        self.cranking_rpm > 150.0
    }

    pub fn draw_normal(&self) -> bool {
        self.draw_amps < 250.0
    }

    pub fn start_reliability_pct(&self) -> f64 {
        if self.attempts == 0 {
            return 100.0;
        }
        self.successful_starts as f64 / self.attempts as f64 * 100.0
    }

    pub fn quick_start(&self) -> bool {
        self.start_time_ms < 1500
    }

    pub fn health_score(&self) -> f64 {
        let crank_s = if self.cranking_ok() { 30.0 } else { 10.0 };
        let draw_s = if self.draw_normal() { 30.0 } else { 10.0 };
        let rel_s = self.start_reliability_pct() / 100.0 * 40.0;
        crank_s + draw_s + rel_s
    }

    pub fn needs_attention(&self) -> bool {
        !self.cranking_ok() || !self.draw_normal() || self.start_reliability_pct() < 90.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cranking_ok() {
        let s = StarterMotor::new();
        assert!(s.cranking_ok());
    }

    #[test]
    fn test_draw_normal() {
        let s = StarterMotor::new();
        assert!(s.draw_normal());
    }

    #[test]
    fn test_reliability() {
        let s = StarterMotor::new();
        assert!((s.start_reliability_pct() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_quick_start() {
        let s = StarterMotor::new();
        assert!(s.quick_start());
    }

    #[test]
    fn test_health() {
        let s = StarterMotor::new();
        assert!(s.health_score() > 90.0);
    }

    #[test]
    fn test_no_attention() {
        let s = StarterMotor::new();
        assert!(!s.needs_attention());
    }

    #[test]
    fn test_low_reliability() {
        let mut s = StarterMotor::new();
        s.successful_starts = 80;
        assert!(s.needs_attention());
    }
}
