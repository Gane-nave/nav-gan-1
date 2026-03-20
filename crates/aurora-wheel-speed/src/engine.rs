/// Wheel speed sensing: individual wheel RPM, slip detection, ABS input
/// Phase 204

#[derive(Debug, Clone)]
pub struct WheelSpeedSensor {
    pub position: String,
    pub rpm: f64,
    pub speed_kmh: f64,
    pub pulse_count: u64,
    pub signal_valid: bool,
}

impl Default for WheelSpeedSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl WheelSpeedSensor {
    pub fn new() -> Self {
        Self {
            position: "front_left".into(),
            rpm: 800.0,
            speed_kmh: 60.0,
            pulse_count: 0,
            signal_valid: true,
        }
    }

    pub fn is_moving(&self) -> bool {
        self.rpm > 5.0
    }

    pub fn slip_ratio(&self, reference_speed_kmh: f64) -> f64 {
        if reference_speed_kmh <= 0.0 {
            return 0.0;
        }
        ((reference_speed_kmh - self.speed_kmh) / reference_speed_kmh).abs()
    }

    pub fn wheel_locked(&self, reference_speed_kmh: f64) -> bool {
        reference_speed_kmh > 10.0 && self.speed_kmh < 1.0
    }

    pub fn spinning(&self, reference_speed_kmh: f64) -> bool {
        self.speed_kmh > reference_speed_kmh * 1.3 && reference_speed_kmh > 5.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moving() {
        let w = WheelSpeedSensor::new();
        assert!(w.is_moving());
    }

    #[test]
    fn test_slip_ratio() {
        let w = WheelSpeedSensor::new();
        assert!(w.slip_ratio(60.0) < 0.01);
    }

    #[test]
    fn test_not_locked() {
        let w = WheelSpeedSensor::new();
        assert!(!w.wheel_locked(60.0));
    }

    #[test]
    fn test_locked() {
        let mut w = WheelSpeedSensor::new();
        w.speed_kmh = 0.0;
        assert!(w.wheel_locked(60.0));
    }

    #[test]
    fn test_not_spinning() {
        let w = WheelSpeedSensor::new();
        assert!(!w.spinning(60.0));
    }

    #[test]
    fn test_spinning() {
        let mut w = WheelSpeedSensor::new();
        w.speed_kmh = 100.0;
        assert!(w.spinning(60.0));
    }
}
