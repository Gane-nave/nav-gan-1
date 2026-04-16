/// Pedal sensor: accelerator, brake, clutch position and force
/// Phase 259

#[derive(Debug, Clone)]
pub struct PedalSensor {
    pub accelerator_pct: f64,
    pub brake_pct: f64,
    pub clutch_pct: f64,
    pub brake_force_n: f64,
    pub sensor_ok: bool,
}

impl Default for PedalSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl PedalSensor {
    pub fn new() -> Self {
        Self {
            accelerator_pct: 0.0,
            brake_pct: 0.0,
            clutch_pct: 0.0,
            brake_force_n: 0.0,
            sensor_ok: true,
        }
    }

    pub fn throttle_applied(&self) -> bool {
        self.accelerator_pct > 2.0
    }

    pub fn braking(&self) -> bool {
        self.brake_pct > 2.0
    }

    pub fn both_pedals(&self) -> bool {
        self.throttle_applied() && self.braking()
    }

    pub fn emergency_brake(&self) -> bool {
        self.brake_pct > 90.0 && self.brake_force_n > 500.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok {
            return 0.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_throttle() {
        let p = PedalSensor::new();
        assert!(!p.throttle_applied());
    }

    #[test]
    fn test_no_braking() {
        let p = PedalSensor::new();
        assert!(!p.braking());
    }

    #[test]
    fn test_no_both() {
        let p = PedalSensor::new();
        assert!(!p.both_pedals());
    }

    #[test]
    fn test_no_emergency() {
        let p = PedalSensor::new();
        assert!(!p.emergency_brake());
    }

    #[test]
    fn test_throttle() {
        let mut p = PedalSensor::new();
        p.accelerator_pct = 50.0;
        assert!(p.throttle_applied());
    }

    #[test]
    fn test_health() {
        let p = PedalSensor::new();
        assert!((p.health_score() - 100.0).abs() < 0.1);
    }
}
