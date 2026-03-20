/// Brake booster: vacuum, diaphragm, check valve, pushrod
/// Phase 662

#[derive(Debug, Clone)]
pub struct BrakeBooster {
    pub vacuum_ok: bool,
    pub diaphragm_ok: bool,
    pub check_valve_ok: bool,
    pub pushrod_ok: bool,
    pub assist_ok: bool,
}

impl Default for BrakeBooster {
    fn default() -> Self {
        Self::new()
    }
}

impl BrakeBooster {
    pub fn new() -> Self {
        Self {
            vacuum_ok: true,
            diaphragm_ok: true,
            check_valve_ok: true,
            pushrod_ok: true,
            assist_ok: true,
        }
    }

    pub fn vacuum_system_ok(&self) -> bool {
        self.vacuum_ok && self.check_valve_ok
    }

    pub fn mechanical_ok(&self) -> bool {
        self.diaphragm_ok && self.pushrod_ok
    }

    pub fn all_ok(&self) -> bool {
        self.vacuum_system_ok() && self.mechanical_ok() && self.assist_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.diaphragm_ok || !self.vacuum_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.diaphragm_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vacuum() {
        let c = BrakeBooster::new();
        assert!(c.vacuum_system_ok());
    }

    #[test]
    fn test_mechanical() {
        let c = BrakeBooster::new();
        assert!(c.mechanical_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BrakeBooster::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = BrakeBooster::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_diaphragm() {
        let mut c = BrakeBooster::new();
        c.diaphragm_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = BrakeBooster::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
