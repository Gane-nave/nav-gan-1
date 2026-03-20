/// Hill start assist: incline detection, brake hold
/// Phase 491

#[derive(Debug, Clone)]
pub struct HillAssist {
    pub incline_deg: f64,
    pub brake_hold: bool,
    pub ha_active: bool,
    pub sensor_ok: bool,
    pub ecu_ok: bool,
}

impl Default for HillAssist {
    fn default() -> Self {
        Self::new()
    }
}

impl HillAssist {
    pub fn new() -> Self {
        Self {
            incline_deg: 5.0,
            brake_hold: true,
            ha_active: true,
            sensor_ok: true,
            ecu_ok: true,
        }
    }

    pub fn on_hill(&self) -> bool {
        self.incline_deg.abs() > 3.0
    }

    pub fn system_ok(&self) -> bool {
        self.sensor_ok && self.ecu_ok
    }

    pub fn all_ok(&self) -> bool {
        self.system_ok() && self.ha_active
    }

    pub fn needs_service(&self) -> bool {
        !self.sensor_ok || !self.ecu_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.ecu_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_on_hill() {
        let c = HillAssist::new();
        assert!(c.on_hill());
    }

    #[test]
    fn test_system() {
        let c = HillAssist::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HillAssist::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = HillAssist::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_ecu_fail() {
        let mut c = HillAssist::new();
        c.ecu_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = HillAssist::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
