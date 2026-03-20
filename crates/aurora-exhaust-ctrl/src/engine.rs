/// Exhaust control: valve, flap, backpressure, sound management
/// Phase 311

#[derive(Debug, Clone)]
pub struct ExhaustController {
    pub valve_open_pct: f64,
    pub backpressure_kpa: f64,
    pub temp_c: f64,
    pub flap_ok: bool,
    pub sport_mode: bool,
}

impl Default for ExhaustController {
    fn default() -> Self {
        Self::new()
    }
}

impl ExhaustController {
    pub fn new() -> Self {
        Self {
            valve_open_pct: 50.0,
            backpressure_kpa: 8.0,
            temp_c: 300.0,
            flap_ok: true,
            sport_mode: false,
        }
    }

    pub fn fully_open(&self) -> bool {
        self.valve_open_pct > 95.0
    }

    pub fn restricted(&self) -> bool {
        self.backpressure_kpa > 20.0
    }

    pub fn overheating(&self) -> bool {
        self.temp_c > 800.0
    }

    pub fn needs_service(&self) -> bool {
        !self.flap_ok || self.restricted()
    }

    pub fn health_score(&self) -> f64 {
        if !self.flap_ok {
            return 0.0;
        }
        if self.restricted() {
            return 40.0;
        }
        if self.overheating() {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_open() {
        let e = ExhaustController::new();
        assert!(!e.fully_open());
    }

    #[test]
    fn test_not_restricted() {
        let e = ExhaustController::new();
        assert!(!e.restricted());
    }

    #[test]
    fn test_not_hot() {
        let e = ExhaustController::new();
        assert!(!e.overheating());
    }

    #[test]
    fn test_no_service() {
        let e = ExhaustController::new();
        assert!(!e.needs_service());
    }

    #[test]
    fn test_restricted() {
        let mut e = ExhaustController::new();
        e.backpressure_kpa = 25.0;
        assert!(e.restricted());
    }

    #[test]
    fn test_health() {
        let e = ExhaustController::new();
        assert!((e.health_score() - 100.0).abs() < 0.1);
    }
}
