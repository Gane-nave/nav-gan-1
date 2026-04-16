/// touch sensor: detect, position, gesture, pressure, log
/// Phase 1301

#[derive(Debug, Clone)]
pub struct TouchSensor {
    pub detect_ok: bool,
    pub position_ok: bool,
    pub gesture_ok: bool,
    pub pressure_ok: bool,
    pub log_ok: bool,
}

impl Default for TouchSensor {
    fn default() -> Self {
        Self::new()
    }
}

impl TouchSensor {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            position_ok: true,
            gesture_ok: true,
            pressure_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.position_ok && self.gesture_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.pressure_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.position_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = TouchSensor::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TouchSensor::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TouchSensor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TouchSensor::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TouchSensor::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TouchSensor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
