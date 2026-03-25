/// Condenser fan: AC condenser cooling, electric fan, thermal switch
/// Phase 441

#[derive(Debug, Clone)]
pub struct CondenserFan {
    pub speed_rpm: f64,
    pub motor_ok: bool,
    pub relay_ok: bool,
    pub temp_c: f64,
    pub max_temp_c: f64,
}

impl Default for CondenserFan {
    fn default() -> Self {
        Self::new()
    }
}

impl CondenserFan {
    pub fn new() -> Self {
        Self {
            speed_rpm: 2000.0,
            motor_ok: true,
            relay_ok: true,
            temp_c: 55.0,
            max_temp_c: 80.0,
        }
    }

    pub fn running(&self) -> bool {
        self.speed_rpm > 0.0 && self.motor_ok
    }

    pub fn temp_ok(&self) -> bool {
        self.temp_c < self.max_temp_c
    }

    pub fn all_ok(&self) -> bool {
        self.running() && self.temp_ok() && self.relay_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.motor_ok || !self.relay_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.motor_ok {
            return 0.0;
        }
        if !self.relay_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running() {
        let c = CondenserFan::new();
        assert!(c.running());
    }

    #[test]
    fn test_temp() {
        let c = CondenserFan::new();
        assert!(c.temp_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CondenserFan::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = CondenserFan::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_dead_motor() {
        let mut c = CondenserFan::new();
        c.motor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = CondenserFan::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
