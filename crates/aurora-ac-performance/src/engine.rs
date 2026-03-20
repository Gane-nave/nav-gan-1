/// AC performance: vent temp, pressure, charge, leak
/// Phase 825

#[derive(Debug, Clone)]
pub struct AcPerformance {
    pub vent_ok: bool,
    pub pressure_ok: bool,
    pub charge_ok: bool,
    pub leak_free: bool,
    pub clutch_ok: bool,
}

impl Default for AcPerformance {
    fn default() -> Self {
        Self::new()
    }
}

impl AcPerformance {
    pub fn new() -> Self {
        Self {
            vent_ok: true,
            pressure_ok: true,
            charge_ok: true,
            leak_free: true,
            clutch_ok: true,
        }
    }

    pub fn cooling_ok(&self) -> bool {
        self.vent_ok && self.charge_ok && self.clutch_ok
    }

    pub fn system_ok(&self) -> bool {
        self.pressure_ok && self.leak_free
    }

    pub fn all_ok(&self) -> bool {
        self.cooling_ok() && self.system_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.charge_ok || !self.leak_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.charge_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cooling() {
        let c = AcPerformance::new();
        assert!(c.cooling_ok());
    }

    #[test]
    fn test_system() {
        let c = AcPerformance::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AcPerformance::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = AcPerformance::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_charge() {
        let mut c = AcPerformance::new();
        c.charge_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = AcPerformance::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
