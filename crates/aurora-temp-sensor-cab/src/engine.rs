/// Cabin temperature sensor: multi-zone, aspirated, sun-load compensated
/// Phase 447

#[derive(Debug, Clone)]
pub struct TempSensorCab {
    pub temp_c: f64,
    pub target_c: f64,
    pub sensor_ok: bool,
    pub aspirator_ok: bool,
    pub zone_count: u8,
}

impl Default for TempSensorCab {
    fn default() -> Self {
        Self::new()
    }
}

impl TempSensorCab {
    pub fn new() -> Self {
        Self {
            temp_c: 22.0,
            target_c: 22.0,
            sensor_ok: true,
            aspirator_ok: true,
            zone_count: 2,
        }
    }

    pub fn at_target(&self) -> bool {
        (self.temp_c - self.target_c).abs() < 2.0
    }

    pub fn all_ok(&self) -> bool {
        self.sensor_ok && self.aspirator_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.sensor_ok
    }

    pub fn multi_zone(&self) -> bool {
        self.zone_count > 1
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok {
            return 0.0;
        }
        if !self.aspirator_ok {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target() {
        let t = TempSensorCab::new();
        assert!(t.at_target());
    }

    #[test]
    fn test_all_ok() {
        let t = TempSensorCab::new();
        assert!(t.all_ok());
    }

    #[test]
    fn test_no_service() {
        let t = TempSensorCab::new();
        assert!(!t.needs_service());
    }

    #[test]
    fn test_multi_zone() {
        let t = TempSensorCab::new();
        assert!(t.multi_zone());
    }

    #[test]
    fn test_bad_sensor() {
        let mut t = TempSensorCab::new();
        t.sensor_ok = false;
        assert!(t.needs_service());
    }

    #[test]
    fn test_health() {
        let t = TempSensorCab::new();
        assert!((t.health_score() - 100.0).abs() < 0.1);
    }
}
