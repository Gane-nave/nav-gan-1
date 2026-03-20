/// ac refrigerant: charge, pressure, temp, leak, check
/// Phase 1257

#[derive(Debug, Clone)]
pub struct AcRefrig {
    pub charge_ok: bool,
    pub pressure_ok: bool,
    pub temp_ok: bool,
    pub leak_ok: bool,
    pub check_ok: bool,
}

impl Default for AcRefrig {
    fn default() -> Self {
        Self::new()
    }
}

impl AcRefrig {
    pub fn new() -> Self {
        Self {
            charge_ok: true,
            pressure_ok: true,
            temp_ok: true,
            leak_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.charge_ok && self.pressure_ok && self.temp_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.leak_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.charge_ok || !self.pressure_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.charge_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = AcRefrig::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AcRefrig::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AcRefrig::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AcRefrig::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AcRefrig::new();
        c.charge_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AcRefrig::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
