/// bms core: voltage, current, temperature, soc, soh
/// Phase 1141

#[derive(Debug, Clone)]
pub struct BmsCore {
    pub voltage_ok: bool,
    pub current_ok: bool,
    pub temperature_ok: bool,
    pub soc_ok: bool,
    pub soh_ok: bool,
}

impl Default for BmsCore {
    fn default() -> Self {
        Self::new()
    }
}

impl BmsCore {
    pub fn new() -> Self {
        Self {
            voltage_ok: true,
            current_ok: true,
            temperature_ok: true,
            soc_ok: true,
            soh_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.voltage_ok && self.current_ok && self.temperature_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.soc_ok && self.soh_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.voltage_ok || !self.current_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.voltage_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = BmsCore::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = BmsCore::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BmsCore::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = BmsCore::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = BmsCore::new();
        c.voltage_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = BmsCore::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
