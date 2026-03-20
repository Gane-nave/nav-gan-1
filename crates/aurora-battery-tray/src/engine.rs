/// Battery tray: bracket, hold-down, insulator, vent
/// Phase 804

#[derive(Debug, Clone)]
pub struct BatteryTray {
    pub bracket_ok: bool,
    pub hold_down_ok: bool,
    pub insulator_ok: bool,
    pub vent_ok: bool,
    pub corrosion_free: bool,
}

impl Default for BatteryTray {
    fn default() -> Self {
        Self::new()
    }
}

impl BatteryTray {
    pub fn new() -> Self {
        Self {
            bracket_ok: true,
            hold_down_ok: true,
            insulator_ok: true,
            vent_ok: true,
            corrosion_free: true,
        }
    }

    pub fn mounting_ok(&self) -> bool {
        self.bracket_ok && self.hold_down_ok
    }

    pub fn protection_ok(&self) -> bool {
        self.insulator_ok && self.vent_ok && self.corrosion_free
    }

    pub fn all_ok(&self) -> bool {
        self.mounting_ok() && self.protection_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.bracket_ok || !self.corrosion_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.corrosion_free {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mounting() {
        let c = BatteryTray::new();
        assert!(c.mounting_ok());
    }

    #[test]
    fn test_protection() {
        let c = BatteryTray::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BatteryTray::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = BatteryTray::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_corrosion() {
        let mut c = BatteryTray::new();
        c.corrosion_free = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = BatteryTray::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
