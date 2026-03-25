/// Brake inspection: pad thickness, rotor runout, fluid
/// Phase 823

#[derive(Debug, Clone)]
pub struct BrakeInspect {
    pub pad_ok: bool,
    pub runout_ok: bool,
    pub fluid_ok: bool,
    pub hose_ok: bool,
    pub caliper_ok: bool,
}

impl Default for BrakeInspect {
    fn default() -> Self {
        Self::new()
    }
}

impl BrakeInspect {
    pub fn new() -> Self {
        Self {
            pad_ok: true,
            runout_ok: true,
            fluid_ok: true,
            hose_ok: true,
            caliper_ok: true,
        }
    }

    pub fn friction_ok(&self) -> bool {
        self.pad_ok && self.runout_ok && self.caliper_ok
    }

    pub fn hydraulic_ok(&self) -> bool {
        self.fluid_ok && self.hose_ok
    }

    pub fn all_ok(&self) -> bool {
        self.friction_ok() && self.hydraulic_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.pad_ok || !self.fluid_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.pad_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_friction() {
        let c = BrakeInspect::new();
        assert!(c.friction_ok());
    }

    #[test]
    fn test_hydraulic() {
        let c = BrakeInspect::new();
        assert!(c.hydraulic_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BrakeInspect::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = BrakeInspect::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_pad() {
        let mut c = BrakeInspect::new();
        c.pad_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = BrakeInspect::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
