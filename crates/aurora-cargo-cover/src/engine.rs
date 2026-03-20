/// Cargo cover: retractable, motor, sensor, lock
/// Phase 757

#[derive(Debug, Clone)]
pub struct CargoCover {
    pub retract_ok: bool,
    pub motor_ok: bool,
    pub sensor_ok: bool,
    pub lock_ok: bool,
    pub track_ok: bool,
}

impl Default for CargoCover {
    fn default() -> Self {
        Self::new()
    }
}

impl CargoCover {
    pub fn new() -> Self {
        Self {
            retract_ok: true,
            motor_ok: true,
            sensor_ok: true,
            lock_ok: true,
            track_ok: true,
        }
    }

    pub fn movement_ok(&self) -> bool {
        self.retract_ok && self.motor_ok && self.track_ok
    }

    pub fn security_ok(&self) -> bool {
        self.sensor_ok && self.lock_ok
    }

    pub fn all_ok(&self) -> bool {
        self.movement_ok() && self.security_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.motor_ok || !self.track_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movement() {
        let c = CargoCover::new();
        assert!(c.movement_ok());
    }

    #[test]
    fn test_security() {
        let c = CargoCover::new();
        assert!(c.security_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CargoCover::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = CargoCover::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_motor() {
        let mut c = CargoCover::new();
        c.motor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = CargoCover::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
