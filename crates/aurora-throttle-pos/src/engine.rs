/// Throttle position sensor: angle, linearity, idle
/// Phase 586

#[derive(Debug, Clone)]
pub struct ThrottlePos {
    pub angle_deg: f64,
    pub linearity_ok: bool,
    pub idle_ok: bool,
    pub signal_ok: bool,
    pub calibrated: bool,
}

impl Default for ThrottlePos {
    fn default() -> Self {
        Self::new()
    }
}

impl ThrottlePos {
    pub fn new() -> Self {
        Self {
            angle_deg: 15.0,
            linearity_ok: true,
            idle_ok: true,
            signal_ok: true,
            calibrated: true,
        }
    }

    pub fn position_valid(&self) -> bool {
        self.signal_ok && self.angle_deg >= 0.0
    }

    pub fn response_ok(&self) -> bool {
        self.linearity_ok && self.idle_ok
    }

    pub fn all_ok(&self) -> bool {
        self.position_valid() && self.response_ok() && self.calibrated
    }

    pub fn needs_service(&self) -> bool {
        !self.signal_ok || !self.linearity_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.signal_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        let c = ThrottlePos::new();
        assert!(c.position_valid());
    }

    #[test]
    fn test_response() {
        let c = ThrottlePos::new();
        assert!(c.response_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ThrottlePos::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = ThrottlePos::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_signal() {
        let mut c = ThrottlePos::new();
        c.signal_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = ThrottlePos::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
