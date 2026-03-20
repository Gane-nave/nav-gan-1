/// Heat shield: exhaust thermal barrier, component protection
/// Phase 362

#[derive(Debug, Clone)]
pub struct HeatShield {
    pub surface_temp_c: f64,
    pub max_temp_c: f64,
    pub intact: bool,
    pub fasteners_ok: bool,
    pub rattle_detected: bool,
}

impl Default for HeatShield {
    fn default() -> Self {
        Self::new()
    }
}

impl HeatShield {
    pub fn new() -> Self {
        Self {
            surface_temp_c: 150.0,
            max_temp_c: 500.0,
            intact: true,
            fasteners_ok: true,
            rattle_detected: false,
        }
    }

    pub fn temp_ok(&self) -> bool {
        self.surface_temp_c < self.max_temp_c
    }

    pub fn effective(&self) -> bool {
        self.intact && self.temp_ok()
    }

    pub fn needs_repair(&self) -> bool {
        !self.intact || !self.fasteners_ok || self.rattle_detected
    }

    pub fn protection_ok(&self) -> bool {
        self.intact && self.fasteners_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.intact {
            return 0.0;
        }
        if self.rattle_detected {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp() {
        let h = HeatShield::new();
        assert!(h.temp_ok());
    }

    #[test]
    fn test_effective() {
        let h = HeatShield::new();
        assert!(h.effective());
    }

    #[test]
    fn test_no_repair() {
        let h = HeatShield::new();
        assert!(!h.needs_repair());
    }

    #[test]
    fn test_protection() {
        let h = HeatShield::new();
        assert!(h.protection_ok());
    }

    #[test]
    fn test_rattle() {
        let mut h = HeatShield::new();
        h.rattle_detected = true;
        assert!(h.needs_repair());
    }

    #[test]
    fn test_health() {
        let h = HeatShield::new();
        assert!((h.health_score() - 100.0).abs() < 0.1);
    }
}
