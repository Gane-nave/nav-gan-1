/// Tire temperature: sensor, zone, optimal, warning
/// Phase 951

#[derive(Debug, Clone)]
pub struct TireTemp {
    pub sensor_ok: bool,
    pub zone_ok: bool,
    pub optimal_ok: bool,
    pub warning_ok: bool,
    pub display_ok: bool,
}

impl Default for TireTemp {
    fn default() -> Self {
        Self::new()
    }
}

impl TireTemp {
    pub fn new() -> Self {
        Self {
            sensor_ok: true,
            zone_ok: true,
            optimal_ok: true,
            warning_ok: true,
            display_ok: true,
        }
    }

    pub fn monitoring_ok(&self) -> bool {
        self.sensor_ok && self.zone_ok && self.display_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.optimal_ok && self.warning_ok
    }

    pub fn all_ok(&self) -> bool {
        self.monitoring_ok() && self.safety_ok()
    }

    pub fn needs_calibration(&self) -> bool {
        !self.sensor_ok || !self.zone_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitoring() {
        let c = TireTemp::new();
        assert!(c.monitoring_ok());
    }

    #[test]
    fn test_safety() {
        let c = TireTemp::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TireTemp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cal() {
        let c = TireTemp::new();
        assert!(!c.needs_calibration());
    }

    #[test]
    fn test_sensor() {
        let mut c = TireTemp::new();
        c.sensor_ok = false;
        assert!(c.needs_calibration());
    }

    #[test]
    fn test_health() {
        let c = TireTemp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
