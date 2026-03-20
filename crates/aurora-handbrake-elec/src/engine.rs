/// Electric handbrake: motor, switch, auto-hold, hill
/// Phase 753

#[derive(Debug, Clone)]
pub struct HandbrakeElec {
    pub motor_ok: bool,
    pub switch_ok: bool,
    pub auto_hold_ok: bool,
    pub hill_ok: bool,
    pub indicator_ok: bool,
}

impl Default for HandbrakeElec {
    fn default() -> Self {
        Self::new()
    }
}

impl HandbrakeElec {
    pub fn new() -> Self {
        Self {
            motor_ok: true,
            switch_ok: true,
            auto_hold_ok: true,
            hill_ok: true,
            indicator_ok: true,
        }
    }

    pub fn actuation_ok(&self) -> bool {
        self.motor_ok && self.switch_ok
    }

    pub fn features_ok(&self) -> bool {
        self.auto_hold_ok && self.hill_ok && self.indicator_ok
    }

    pub fn all_ok(&self) -> bool {
        self.actuation_ok() && self.features_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.motor_ok || !self.switch_ok
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
    fn test_actuation() {
        let c = HandbrakeElec::new();
        assert!(c.actuation_ok());
    }

    #[test]
    fn test_features() {
        let c = HandbrakeElec::new();
        assert!(c.features_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HandbrakeElec::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = HandbrakeElec::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_motor() {
        let mut c = HandbrakeElec::new();
        c.motor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = HandbrakeElec::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
