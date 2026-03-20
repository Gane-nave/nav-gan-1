/// Throttle body: butterfly valve, position sensor, idle
/// Phase 501

#[derive(Debug, Clone)]
pub struct ThrottleBody {
    pub position_pct: f64,
    pub target_pct: f64,
    pub motor_ok: bool,
    pub sensor_ok: bool,
    pub carbon_free: bool,
}

impl Default for ThrottleBody {
    fn default() -> Self {
        Self::new()
    }
}

impl ThrottleBody {
    pub fn new() -> Self {
        Self {
            position_pct: 15.0,
            target_pct: 15.0,
            motor_ok: true,
            sensor_ok: true,
            carbon_free: true,
        }
    }

    pub fn at_target(&self) -> bool {
        (self.position_pct - self.target_pct).abs() < 2.0
    }

    pub fn is_idle(&self) -> bool {
        self.position_pct < 5.0
    }

    pub fn all_ok(&self) -> bool {
        self.at_target() && self.motor_ok && self.sensor_ok && self.carbon_free
    }

    pub fn needs_cleaning(&self) -> bool {
        !self.carbon_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at_target() {
        let c = ThrottleBody::new();
        assert!(c.at_target());
    }

    #[test]
    fn test_not_idle() {
        let c = ThrottleBody::new();
        assert!(!c.is_idle());
    }

    #[test]
    fn test_all_ok() {
        let c = ThrottleBody::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_clean() {
        let c = ThrottleBody::new();
        assert!(!c.needs_cleaning());
    }

    #[test]
    fn test_carbon() {
        let mut c = ThrottleBody::new();
        c.carbon_free = false;
        assert!(c.needs_cleaning());
    }

    #[test]
    fn test_health() {
        let c = ThrottleBody::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
