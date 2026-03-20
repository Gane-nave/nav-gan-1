/// Crankcase: ventilation, pressure monitoring, PCV valve
/// Phase 316

#[derive(Debug, Clone)]
pub struct Crankcase {
    pub pressure_kpa: f64,
    pub ventilation_ok: bool,
    pub pcv_ok: bool,
    pub oil_level_ok: bool,
    pub blow_by_lpm: f64,
}

impl Default for Crankcase {
    fn default() -> Self {
        Self::new()
    }
}

impl Crankcase {
    pub fn new() -> Self {
        Self {
            pressure_kpa: -0.5,
            ventilation_ok: true,
            pcv_ok: true,
            oil_level_ok: true,
            blow_by_lpm: 3.0,
        }
    }

    pub fn pressure_ok(&self) -> bool {
        self.pressure_kpa > -2.0 && self.pressure_kpa < 1.0
    }

    pub fn all_ok(&self) -> bool {
        self.ventilation_ok && self.pcv_ok && self.oil_level_ok && self.pressure_ok()
    }

    pub fn blow_by_excessive(&self) -> bool {
        self.blow_by_lpm > 15.0
    }

    pub fn needs_service(&self) -> bool {
        !self.pcv_ok || !self.ventilation_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.oil_level_ok {
            return 10.0;
        }
        if !self.pcv_ok {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure() {
        let c = Crankcase::new();
        assert!(c.pressure_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Crankcase::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_blow_by() {
        let c = Crankcase::new();
        assert!(!c.blow_by_excessive());
    }

    #[test]
    fn test_no_service() {
        let c = Crankcase::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_bad_pcv() {
        let mut c = Crankcase::new();
        c.pcv_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Crankcase::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
