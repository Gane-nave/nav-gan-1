/// Steering wheel: heated rim, controls, paddle shift
/// Phase 750

#[derive(Debug, Clone)]
pub struct SteeringWheel {
    pub heated_ok: bool,
    pub controls_ok: bool,
    pub paddle_ok: bool,
    pub airbag_ok: bool,
    pub clock_spring_ok: bool,
}

impl Default for SteeringWheel {
    fn default() -> Self {
        Self::new()
    }
}

impl SteeringWheel {
    pub fn new() -> Self {
        Self {
            heated_ok: true,
            controls_ok: true,
            paddle_ok: true,
            airbag_ok: true,
            clock_spring_ok: true,
        }
    }

    pub fn comfort_ok(&self) -> bool {
        self.heated_ok && self.controls_ok && self.paddle_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.airbag_ok && self.clock_spring_ok
    }

    pub fn all_ok(&self) -> bool {
        self.comfort_ok() && self.safety_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.clock_spring_ok || !self.airbag_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.clock_spring_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comfort() {
        let c = SteeringWheel::new();
        assert!(c.comfort_ok());
    }

    #[test]
    fn test_safety() {
        let c = SteeringWheel::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SteeringWheel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = SteeringWheel::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_clockspring() {
        let mut c = SteeringWheel::new();
        c.clock_spring_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = SteeringWheel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
