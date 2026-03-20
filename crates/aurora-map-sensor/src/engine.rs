/// MAP sensor: manifold pressure, vacuum, barometric
/// Phase 584

#[derive(Debug, Clone)]
pub struct MapSensor {
    pub pressure_kpa: f64,
    pub vacuum_ok: bool,
    pub baro_ok: bool,
    pub signal_ok: bool,
    pub calibrated: bool,
}

impl Default for MapSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl MapSensor {
    pub fn new() -> Self {
        Self {
            pressure_kpa: 40.0,
            vacuum_ok: true,
            baro_ok: true,
            signal_ok: true,
            calibrated: true,
        }
    }

    pub fn pressure_valid(&self) -> bool {
        self.signal_ok && self.pressure_kpa > 0.0
    }

    pub fn system_ok(&self) -> bool {
        self.vacuum_ok && self.baro_ok
    }

    pub fn all_ok(&self) -> bool {
        self.pressure_valid() && self.system_ok() && self.calibrated
    }

    pub fn needs_service(&self) -> bool {
        !self.signal_ok || !self.calibrated
    }

    pub fn health_score(&self) -> f64 {
        if !self.signal_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure() {
        let c = MapSensor::new();
        assert!(c.pressure_valid());
    }

    #[test]
    fn test_system() {
        let c = MapSensor::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MapSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = MapSensor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_signal() {
        let mut c = MapSensor::new();
        c.signal_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = MapSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
