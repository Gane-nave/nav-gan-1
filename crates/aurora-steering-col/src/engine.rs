/// Steering column: tilt, telescope, lock
/// Phase 481

#[derive(Debug, Clone)]
pub struct SteeringColumn {
    pub tilt_deg: f64,
    pub telescope_mm: f64,
    pub locked: bool,
    pub motor_ok: bool,
    pub sensor_ok: bool,
}

impl Default for SteeringColumn {
    fn default() -> Self {
        Self::new()
    }
}

impl SteeringColumn {
    pub fn new() -> Self {
        Self {
            tilt_deg: 5.0,
            telescope_mm: 20.0,
            locked: false,
            motor_ok: true,
            sensor_ok: true,
        }
    }

    pub fn is_adjustable(&self) -> bool {
        self.motor_ok && self.sensor_ok
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }

    pub fn all_ok(&self) -> bool {
        self.motor_ok && self.sensor_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.motor_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok { return 30.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjustable() {
        let c = SteeringColumn::new();
        assert!(c.is_adjustable());
    }

    #[test]
    fn test_not_locked() {
        let c = SteeringColumn::new();
        assert!(!c.is_locked());
    }

    #[test]
    fn test_all_ok() {
        let c = SteeringColumn::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = SteeringColumn::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_motor_fail() {
        let mut c = SteeringColumn::new();
        c.motor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = SteeringColumn::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
