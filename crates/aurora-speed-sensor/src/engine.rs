/// Vehicle speed sensor: ABS ring, magnetic pickup
/// Phase 597

#[derive(Debug, Clone)]
pub struct SpeedSensor {
    pub speed_kmh: f64,
    pub abs_ring_ok: bool,
    pub pickup_ok: bool,
    pub signal_ok: bool,
    pub calibrated: bool,
}

impl Default for SpeedSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl SpeedSensor {
    pub fn new() -> Self {
        Self {
            speed_kmh: 60.0,
            abs_ring_ok: true,
            pickup_ok: true,
            signal_ok: true,
            calibrated: true,
        }
    }

    pub fn reading_ok(&self) -> bool {
        self.signal_ok && self.speed_kmh >= 0.0
    }

    pub fn hardware_ok(&self) -> bool {
        self.abs_ring_ok && self.pickup_ok
    }

    pub fn all_ok(&self) -> bool {
        self.reading_ok() && self.hardware_ok() && self.calibrated
    }

    pub fn needs_service(&self) -> bool {
        !self.abs_ring_ok || !self.pickup_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.abs_ring_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading() {
        let c = SpeedSensor::new();
        assert!(c.reading_ok());
    }

    #[test]
    fn test_hardware() {
        let c = SpeedSensor::new();
        assert!(c.hardware_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SpeedSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = SpeedSensor::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_ring() {
        let mut c = SpeedSensor::new();
        c.abs_ring_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = SpeedSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
