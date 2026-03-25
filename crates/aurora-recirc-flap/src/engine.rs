/// Recirculation flap: fresh/recirculated air, actuator, mode control
/// Phase 444

#[derive(Debug, Clone)]
pub struct RecircFlap {
    pub recirculating: bool,
    pub actuator_ok: bool,
    pub position_pct: f64,
    pub seal_ok: bool,
    pub auto_mode: bool,
}

impl Default for RecircFlap {
    fn default() -> Self {
        Self::new()
    }
}

impl RecircFlap {
    pub fn new() -> Self {
        Self {
            recirculating: false,
            actuator_ok: true,
            position_pct: 0.0,
            seal_ok: true,
            auto_mode: true,
        }
    }

    pub fn functional(&self) -> bool {
        self.actuator_ok
    }

    pub fn all_ok(&self) -> bool {
        self.actuator_ok && self.seal_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.actuator_ok || !self.seal_ok
    }

    pub fn fresh_air(&self) -> bool {
        !self.recirculating
    }

    pub fn health_score(&self) -> f64 {
        if !self.actuator_ok {
            return 20.0;
        }
        if !self.seal_ok {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functional() {
        let r = RecircFlap::new();
        assert!(r.functional());
    }

    #[test]
    fn test_all_ok() {
        let r = RecircFlap::new();
        assert!(r.all_ok());
    }

    #[test]
    fn test_no_service() {
        let r = RecircFlap::new();
        assert!(!r.needs_service());
    }

    #[test]
    fn test_fresh() {
        let r = RecircFlap::new();
        assert!(r.fresh_air());
    }

    #[test]
    fn test_bad_actuator() {
        let mut r = RecircFlap::new();
        r.actuator_ok = false;
        assert!(r.needs_service());
    }

    #[test]
    fn test_health() {
        let r = RecircFlap::new();
        assert!((r.health_score() - 100.0).abs() < 0.1);
    }
}
