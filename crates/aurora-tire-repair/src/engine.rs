/// Tire repair: plug, patch, sealant, pressure check
/// Phase 813

#[derive(Debug, Clone)]
pub struct TireRepair {
    pub plug_ok: bool,
    pub patch_ok: bool,
    pub sealant_ok: bool,
    pub pressure_ok: bool,
    pub location_ok: bool,
}

impl Default for TireRepair {
    fn default() -> Self {
        Self::new()
    }
}

impl TireRepair {
    pub fn new() -> Self {
        Self {
            plug_ok: true,
            patch_ok: true,
            sealant_ok: true,
            pressure_ok: true,
            location_ok: true,
        }
    }

    pub fn repair_ok(&self) -> bool {
        self.plug_ok && self.patch_ok && self.location_ok
    }

    pub fn verification_ok(&self) -> bool {
        self.sealant_ok && self.pressure_ok
    }

    pub fn all_ok(&self) -> bool {
        self.repair_ok() && self.verification_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.location_ok || !self.pressure_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.location_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repair() {
        let c = TireRepair::new();
        assert!(c.repair_ok());
    }

    #[test]
    fn test_verification() {
        let c = TireRepair::new();
        assert!(c.verification_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TireRepair::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = TireRepair::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_location() {
        let mut c = TireRepair::new();
        c.location_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = TireRepair::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
