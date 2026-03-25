/// Spring damper: coilover, spring rate, damping coefficient
/// Phase 330

#[derive(Debug, Clone)]
pub struct SpringDamper {
    pub spring_rate_n_mm: f64,
    pub damping_coeff: f64,
    pub travel_mm: f64,
    pub max_travel_mm: f64,
    pub condition_ok: bool,
}

impl Default for SpringDamper {
    fn default() -> Self {
        Self::new()
    }
}

impl SpringDamper {
    pub fn new() -> Self {
        Self {
            spring_rate_n_mm: 30.0,
            damping_coeff: 2500.0,
            travel_mm: 50.0,
            max_travel_mm: 120.0,
            condition_ok: true,
        }
    }

    pub fn travel_ok(&self) -> bool {
        self.travel_mm < self.max_travel_mm * 0.9
    }

    pub fn bottomed_out(&self) -> bool {
        self.travel_mm >= self.max_travel_mm
    }

    pub fn needs_replacement(&self) -> bool {
        !self.condition_ok
    }

    pub fn travel_pct(&self) -> f64 {
        if self.max_travel_mm <= 0.0 {
            return 0.0;
        }
        (self.travel_mm / self.max_travel_mm * 100.0).clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        if !self.condition_ok {
            return 0.0;
        }
        if self.bottomed_out() {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_travel_ok() {
        let s = SpringDamper::new();
        assert!(s.travel_ok());
    }

    #[test]
    fn test_not_bottomed() {
        let s = SpringDamper::new();
        assert!(!s.bottomed_out());
    }

    #[test]
    fn test_no_replace() {
        let s = SpringDamper::new();
        assert!(!s.needs_replacement());
    }

    #[test]
    fn test_travel_pct() {
        let s = SpringDamper::new();
        assert!(s.travel_pct() < 50.0);
    }

    #[test]
    fn test_worn() {
        let mut s = SpringDamper::new();
        s.condition_ok = false;
        assert!(s.needs_replacement());
    }

    #[test]
    fn test_health() {
        let s = SpringDamper::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
