/// Engine ECU: injection, ignition, emission, boost
/// Phase 709

#[derive(Debug, Clone)]
pub struct EngineEcu {
    pub injection_ok: bool,
    pub ignition_ok: bool,
    pub emission_ok: bool,
    pub boost_ok: bool,
    pub comm_ok: bool,
}

impl Default for EngineEcu {
    fn default() -> Self {
        Self::new()
    }
}

impl EngineEcu {
    pub fn new() -> Self {
        Self {
            injection_ok: true,
            ignition_ok: true,
            emission_ok: true,
            boost_ok: true,
            comm_ok: true,
        }
    }

    pub fn combustion_ok(&self) -> bool {
        self.injection_ok && self.ignition_ok
    }

    pub fn compliance_ok(&self) -> bool {
        self.emission_ok && self.boost_ok
    }

    pub fn all_ok(&self) -> bool {
        self.combustion_ok() && self.compliance_ok() && self.comm_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.injection_ok || !self.ignition_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.injection_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combustion() {
        let c = EngineEcu::new();
        assert!(c.combustion_ok());
    }

    #[test]
    fn test_compliance() {
        let c = EngineEcu::new();
        assert!(c.compliance_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EngineEcu::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = EngineEcu::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_injection() {
        let mut c = EngineEcu::new();
        c.injection_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = EngineEcu::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
