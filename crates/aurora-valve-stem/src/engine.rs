/// Valve stem: core, cap, seal, TPMS sensor
/// Phase 807

#[derive(Debug, Clone)]
pub struct ValveStem {
    pub core_ok: bool,
    pub cap_ok: bool,
    pub seal_ok: bool,
    pub tpms_ok: bool,
    pub torque_ok: bool,
}

impl Default for ValveStem {
    fn default() -> Self {
        Self::new()
    }
}

impl ValveStem {
    pub fn new() -> Self {
        Self {
            core_ok: true,
            cap_ok: true,
            seal_ok: true,
            tpms_ok: true,
            torque_ok: true,
        }
    }

    pub fn sealing_ok(&self) -> bool {
        self.core_ok && self.seal_ok && self.cap_ok
    }

    pub fn monitoring_ok(&self) -> bool {
        self.tpms_ok && self.torque_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sealing_ok() && self.monitoring_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.core_ok || !self.seal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.core_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sealing() {
        let c = ValveStem::new();
        assert!(c.sealing_ok());
    }

    #[test]
    fn test_monitoring() {
        let c = ValveStem::new();
        assert!(c.monitoring_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ValveStem::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = ValveStem::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_core() {
        let mut c = ValveStem::new();
        c.core_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = ValveStem::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
