/// Throttle position: butterfly valve angle, electronic throttle body
/// Phase 260

#[derive(Debug, Clone)]
pub struct ThrottlePosition {
    pub position_pct: f64,
    pub target_pct: f64,
    pub voltage: f64,
    pub motor_ok: bool,
    pub spring_ok: bool,
}

impl Default for ThrottlePosition {
    fn default() -> Self {
        Self::new()
    }
}

impl ThrottlePosition {
    pub fn new() -> Self {
        Self {
            position_pct: 0.0,
            target_pct: 0.0,
            voltage: 0.5,
            motor_ok: true,
            spring_ok: true,
        }
    }

    pub fn at_target(&self) -> bool {
        (self.position_pct - self.target_pct).abs() < 2.0
    }

    pub fn wide_open(&self) -> bool {
        self.position_pct > 95.0
    }

    pub fn closed(&self) -> bool {
        self.position_pct < 3.0
    }

    pub fn position_error(&self) -> f64 {
        (self.position_pct - self.target_pct).abs()
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok {
            return 0.0;
        }
        if !self.spring_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at_target() {
        let t = ThrottlePosition::new();
        assert!(t.at_target());
    }

    #[test]
    fn test_not_open() {
        let t = ThrottlePosition::new();
        assert!(!t.wide_open());
    }

    #[test]
    fn test_closed() {
        let t = ThrottlePosition::new();
        assert!(t.closed());
    }

    #[test]
    fn test_no_error() {
        let t = ThrottlePosition::new();
        assert!(t.position_error() < 0.1);
    }

    #[test]
    fn test_open() {
        let mut t = ThrottlePosition::new();
        t.position_pct = 100.0;
        assert!(t.wide_open());
    }

    #[test]
    fn test_health() {
        let t = ThrottlePosition::new();
        assert!((t.health_score() - 100.0).abs() < 0.1);
    }
}
