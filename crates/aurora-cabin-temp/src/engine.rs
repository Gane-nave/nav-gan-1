/// Cabin temperature sensor: NTC, duct, sunload
/// Phase 628

#[derive(Debug, Clone)]
pub struct CabinTemp {
    pub cabin_temp_c: f64,
    pub ntc_ok: bool,
    pub duct_ok: bool,
    pub sunload_ok: bool,
    pub calibrated: bool,
}

impl Default for CabinTemp {
    fn default() -> Self {
        Self::new()
    }
}

impl CabinTemp {
    pub fn new() -> Self {
        Self {
            cabin_temp_c: 22.0,
            ntc_ok: true,
            duct_ok: true,
            sunload_ok: true,
            calibrated: true,
        }
    }

    pub fn reading_ok(&self) -> bool {
        self.ntc_ok && self.calibrated
    }

    pub fn system_ok(&self) -> bool {
        self.reading_ok() && self.duct_ok && self.sunload_ok
    }

    pub fn all_ok(&self) -> bool {
        self.system_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.ntc_ok || !self.calibrated
    }

    pub fn health_score(&self) -> f64 {
        if !self.ntc_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let c = CabinTemp::new();
        assert!(c.reading_ok());
    }

    #[test]
    fn test_system() {
        let c = CabinTemp::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CabinTemp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = CabinTemp::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_ntc() {
        let mut c = CabinTemp::new();
        c.ntc_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = CabinTemp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
