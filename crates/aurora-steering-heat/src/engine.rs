/// Steering wheel heater: heating element, temperature control
/// Phase 454

#[derive(Debug, Clone)]
pub struct SteeringHeat {
    pub active: bool,
    pub temp_c: f64,
    pub max_temp_c: f64,
    pub element_ok: bool,
    pub auto_off: bool,
}

impl Default for SteeringHeat {
    fn default() -> Self {
        Self::new()
    }
}

impl SteeringHeat {
    pub fn new() -> Self {
        Self {
            active: true,
            temp_c: 32.0,
            max_temp_c: 40.0,
            element_ok: true,
            auto_off: true,
        }
    }

    pub fn heating(&self) -> bool {
        self.active && self.element_ok
    }

    pub fn temp_ok(&self) -> bool {
        self.temp_c < self.max_temp_c
    }

    pub fn all_ok(&self) -> bool {
        self.element_ok && self.temp_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.element_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.element_ok {
            return 0.0;
        }
        if !self.temp_ok() {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heating() {
        let s = SteeringHeat::new();
        assert!(s.heating());
    }

    #[test]
    fn test_temp() {
        let s = SteeringHeat::new();
        assert!(s.temp_ok());
    }

    #[test]
    fn test_all_ok() {
        let s = SteeringHeat::new();
        assert!(s.all_ok());
    }

    #[test]
    fn test_no_service() {
        let s = SteeringHeat::new();
        assert!(!s.needs_service());
    }

    #[test]
    fn test_bad_element() {
        let mut s = SteeringHeat::new();
        s.element_ok = false;
        assert!(s.needs_service());
    }

    #[test]
    fn test_health() {
        let s = SteeringHeat::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
