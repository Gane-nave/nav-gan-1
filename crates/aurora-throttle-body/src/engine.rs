/// Throttle body: bore, butterfly, motor, TPS
/// Phase 608

#[derive(Debug, Clone)]
pub struct ThrottleBody {
    pub bore_ok: bool,
    pub butterfly_ok: bool,
    pub motor_ok: bool,
    pub tps_ok: bool,
    pub clean: bool,
}

impl Default for ThrottleBody {
    fn default() -> Self {
        Self::new()
    }
}

impl ThrottleBody {
    pub fn new() -> Self {
        Self {
            bore_ok: true,
            butterfly_ok: true,
            motor_ok: true,
            tps_ok: true,
            clean: true,
        }
    }

    pub fn mechanical_ok(&self) -> bool {
        self.bore_ok && self.butterfly_ok
    }

    pub fn electronic_ok(&self) -> bool {
        self.motor_ok && self.tps_ok
    }

    pub fn all_ok(&self) -> bool {
        self.mechanical_ok() && self.electronic_ok() && self.clean
    }

    pub fn needs_cleaning(&self) -> bool {
        !self.clean || !self.bore_ok
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
    fn test_mechanical() {
        let c = ThrottleBody::new();
        assert!(c.mechanical_ok());
    }

    #[test]
    fn test_electronic() {
        let c = ThrottleBody::new();
        assert!(c.electronic_ok());
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
    fn test_dirty() {
        let mut c = ThrottleBody::new();
        c.clean = false;
        assert!(c.needs_cleaning());
    }

    #[test]
    fn test_health() {
        let c = ThrottleBody::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
