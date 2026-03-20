/// Airbag controller: crash sensor, squib circuit, readiness
/// Phase 528

#[derive(Debug, Clone)]
pub struct AirbagController {
    pub crash_sensor_ok: bool,
    pub squib_ok: bool,
    pub clock_spring_ok: bool,
    pub armed: bool,
    pub fault_count: u32,
}

impl Default for AirbagController {
    fn default() -> Self {
        Self::new()
    }
}

impl AirbagController {
    pub fn new() -> Self {
        Self {
            crash_sensor_ok: true,
            squib_ok: true,
            clock_spring_ok: true,
            armed: true,
            fault_count: 0,
        }
    }

    pub fn sensors_ok(&self) -> bool {
        self.crash_sensor_ok && self.clock_spring_ok
    }

    pub fn system_ready(&self) -> bool {
        self.sensors_ok() && self.squib_ok && self.armed
    }

    pub fn all_ok(&self) -> bool {
        self.system_ready() && self.fault_count == 0
    }

    pub fn needs_service(&self) -> bool {
        self.fault_count > 0 || !self.squib_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.squib_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensors() {
        let c = AirbagController::new();
        assert!(c.sensors_ok());
    }

    #[test]
    fn test_ready() {
        let c = AirbagController::new();
        assert!(c.system_ready());
    }

    #[test]
    fn test_all_ok() {
        let c = AirbagController::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = AirbagController::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_squib_fail() {
        let mut c = AirbagController::new();
        c.squib_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = AirbagController::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
