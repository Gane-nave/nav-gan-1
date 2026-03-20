/// Light sensor: ambient, auto headlight, twilight
/// Phase 562

#[derive(Debug, Clone)]
pub struct LightSensor {
    pub lux_reading: f64,
    pub sensor_ok: bool,
    pub auto_mode: bool,
    pub twilight_ok: bool,
    pub calibrated: bool,
}

impl Default for LightSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl LightSensor {
    pub fn new() -> Self {
        Self {
            lux_reading: 5000.0,
            sensor_ok: true,
            auto_mode: true,
            twilight_ok: true,
            calibrated: true,
        }
    }

    pub fn reading_valid(&self) -> bool {
        self.sensor_ok
    }

    pub fn auto_ok(&self) -> bool {
        self.auto_mode && self.calibrated
    }

    pub fn all_ok(&self) -> bool {
        self.reading_valid() && self.auto_ok() && self.twilight_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.sensor_ok || !self.calibrated
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let c = LightSensor::new();
        assert!(c.reading_valid());
    }

    #[test]
    fn test_auto() {
        let c = LightSensor::new();
        assert!(c.auto_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LightSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = LightSensor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_sensor() {
        let mut c = LightSensor::new();
        c.sensor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = LightSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
