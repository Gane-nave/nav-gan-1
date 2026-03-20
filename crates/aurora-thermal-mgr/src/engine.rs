/// thermal mgr: heat, cool, circulate, regulate, alert
/// Phase 1150

#[derive(Debug, Clone)]
pub struct ThermalMgr {
    pub heat_ok: bool,
    pub cool_ok: bool,
    pub circulate_ok: bool,
    pub regulate_ok: bool,
    pub alert_ok: bool,
}

impl Default for ThermalMgr {
    fn default() -> Self {
        Self::new()
    }
}

impl ThermalMgr {
    pub fn new() -> Self {
        Self {
            heat_ok: true,
            cool_ok: true,
            circulate_ok: true,
            regulate_ok: true,
            alert_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.heat_ok && self.cool_ok && self.circulate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.regulate_ok && self.alert_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.heat_ok || !self.cool_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.heat_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = ThermalMgr::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ThermalMgr::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ThermalMgr::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ThermalMgr::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ThermalMgr::new();
        c.heat_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ThermalMgr::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
