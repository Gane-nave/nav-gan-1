/// Service interval: oil, filter, brake, coolant schedule
/// Phase 817

#[derive(Debug, Clone)]
pub struct ServiceInterval {
    pub oil_ok: bool,
    pub filter_ok: bool,
    pub brake_ok: bool,
    pub coolant_ok: bool,
    pub schedule_ok: bool,
}

impl Default for ServiceInterval {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceInterval {
    pub fn new() -> Self {
        Self {
            oil_ok: true,
            filter_ok: true,
            brake_ok: true,
            coolant_ok: true,
            schedule_ok: true,
        }
    }

    pub fn fluids_ok(&self) -> bool {
        self.oil_ok && self.coolant_ok
    }

    pub fn components_ok(&self) -> bool {
        self.filter_ok && self.brake_ok && self.schedule_ok
    }

    pub fn all_ok(&self) -> bool {
        self.fluids_ok() && self.components_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.oil_ok || !self.filter_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.oil_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fluids() {
        let c = ServiceInterval::new();
        assert!(c.fluids_ok());
    }

    #[test]
    fn test_components() {
        let c = ServiceInterval::new();
        assert!(c.components_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ServiceInterval::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = ServiceInterval::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_oil() {
        let mut c = ServiceInterval::new();
        c.oil_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = ServiceInterval::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
