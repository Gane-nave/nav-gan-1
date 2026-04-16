/// Onboard charger: PFC, LLC, isolation, cooling
/// Phase 719

#[derive(Debug, Clone)]
pub struct OnboardCharger {
    pub pfc_ok: bool,
    pub llc_ok: bool,
    pub isolation_ok: bool,
    pub cooling_ok: bool,
    pub comm_ok: bool,
}

impl Default for OnboardCharger {
    fn default() -> Self {
        Self::new()
    }
}

impl OnboardCharger {
    pub fn new() -> Self {
        Self {
            pfc_ok: true,
            llc_ok: true,
            isolation_ok: true,
            cooling_ok: true,
            comm_ok: true,
        }
    }

    pub fn power_ok(&self) -> bool {
        self.pfc_ok && self.llc_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.isolation_ok && self.cooling_ok && self.comm_ok
    }

    pub fn all_ok(&self) -> bool {
        self.power_ok() && self.safety_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.pfc_ok || !self.isolation_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.pfc_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power() {
        let c = OnboardCharger::new();
        assert!(c.power_ok());
    }

    #[test]
    fn test_safety() {
        let c = OnboardCharger::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OnboardCharger::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = OnboardCharger::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_pfc() {
        let mut c = OnboardCharger::new();
        c.pfc_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = OnboardCharger::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
