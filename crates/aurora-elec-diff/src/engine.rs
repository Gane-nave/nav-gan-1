/// Electronic differential: e-axle, torque vectoring, motor control
/// Phase 473

#[derive(Debug, Clone)]
pub struct ElecDiff {
    pub motor_ok: bool,
    pub inverter_ok: bool,
    pub sensor_ok: bool,
    pub vectoring_active: bool,
    pub temp_c: f64,
}

impl Default for ElecDiff {
    fn default() -> Self {
        Self::new()
    }
}

impl ElecDiff {
    pub fn new() -> Self {
        Self {
            motor_ok: true,
            inverter_ok: true,
            sensor_ok: true,
            vectoring_active: true,
            temp_c: 45.0,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.motor_ok && self.inverter_ok && self.sensor_ok
    }

    pub fn vectoring_ok(&self) -> bool {
        self.vectoring_active && self.all_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.motor_ok || !self.inverter_ok
    }

    pub fn temp_ok(&self) -> bool {
        self.temp_c < 80.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok {
            return 0.0;
        }
        if !self.inverter_ok {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ok() {
        let e = ElecDiff::new();
        assert!(e.all_ok());
    }

    #[test]
    fn test_vectoring() {
        let e = ElecDiff::new();
        assert!(e.vectoring_ok());
    }

    #[test]
    fn test_no_service() {
        let e = ElecDiff::new();
        assert!(!e.needs_service());
    }

    #[test]
    fn test_temp() {
        let e = ElecDiff::new();
        assert!(e.temp_ok());
    }

    #[test]
    fn test_bad_motor() {
        let mut e = ElecDiff::new();
        e.motor_ok = false;
        assert!(e.needs_service());
    }

    #[test]
    fn test_health() {
        let e = ElecDiff::new();
        assert!((e.health_score() - 100.0).abs() < 0.1);
    }
}
