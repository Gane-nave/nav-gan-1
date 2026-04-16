/// water pump: circulate, pressurize, seal, drive, check
/// Phase 1249

#[derive(Debug, Clone)]
pub struct WaterPump2 {
    pub circulate_ok: bool,
    pub pressurize_ok: bool,
    pub seal_ok: bool,
    pub drive_ok: bool,
    pub check_ok: bool,
}

impl Default for WaterPump2 {
    fn default() -> Self {
        Self::new()
    }
}

impl WaterPump2 {
    pub fn new() -> Self {
        Self {
            circulate_ok: true,
            pressurize_ok: true,
            seal_ok: true,
            drive_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.circulate_ok && self.pressurize_ok && self.seal_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.drive_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.circulate_ok || !self.pressurize_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.circulate_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = WaterPump2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WaterPump2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WaterPump2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WaterPump2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WaterPump2::new();
        c.circulate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WaterPump2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
