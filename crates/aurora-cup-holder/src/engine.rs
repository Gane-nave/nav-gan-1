/// Cup holder: heater, cooler, sensor, light
/// Phase 754

#[derive(Debug, Clone)]
pub struct CupHolder {
    pub heater_ok: bool,
    pub cooler_ok: bool,
    pub sensor_ok: bool,
    pub light_ok: bool,
    pub latch_ok: bool,
}

impl Default for CupHolder {
    fn default() -> Self {
        Self::new()
    }
}

impl CupHolder {
    pub fn new() -> Self {
        Self {
            heater_ok: true,
            cooler_ok: true,
            sensor_ok: true,
            light_ok: true,
            latch_ok: true,
        }
    }

    pub fn thermal_ok(&self) -> bool {
        self.heater_ok && self.cooler_ok
    }

    pub fn convenience_ok(&self) -> bool {
        self.sensor_ok && self.light_ok && self.latch_ok
    }

    pub fn all_ok(&self) -> bool {
        self.thermal_ok() && self.convenience_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.heater_ok || !self.latch_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.latch_ok { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermal() {
        let c = CupHolder::new();
        assert!(c.thermal_ok());
    }

    #[test]
    fn test_convenience() {
        let c = CupHolder::new();
        assert!(c.convenience_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CupHolder::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = CupHolder::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_latch() {
        let mut c = CupHolder::new();
        c.latch_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = CupHolder::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
