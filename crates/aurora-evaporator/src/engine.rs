/// Evaporator: cooling coil temperature, frost detection, drain
/// Phase 236

#[derive(Debug, Clone)]
pub struct Evaporator {
    pub coil_temp_c: f64,
    pub air_outlet_temp_c: f64,
    pub frost_detected: bool,
    pub drain_clear: bool,
    pub efficiency_pct: f64,
}

impl Default for Evaporator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaporator {
    pub fn new() -> Self {
        Self {
            coil_temp_c: 3.0,
            air_outlet_temp_c: 8.0,
            frost_detected: false,
            drain_clear: true,
            efficiency_pct: 90.0,
        }
    }

    pub fn cooling_ok(&self) -> bool {
        self.air_outlet_temp_c < 12.0
    }

    pub fn frost_risk(&self) -> bool {
        self.coil_temp_c < 0.0
    }

    pub fn drain_blocked(&self) -> bool {
        !self.drain_clear
    }

    pub fn needs_service(&self) -> bool {
        self.frost_detected || self.drain_blocked() || self.efficiency_pct < 60.0
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if self.frost_detected {
            score -= 25.0;
        }
        if !self.drain_clear {
            score -= 25.0;
        }
        if self.efficiency_pct < 70.0 {
            score -= 25.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cooling_ok() {
        let e = Evaporator::new();
        assert!(e.cooling_ok());
    }

    #[test]
    fn test_no_frost_risk() {
        let e = Evaporator::new();
        assert!(!e.frost_risk());
    }

    #[test]
    fn test_drain_clear() {
        let e = Evaporator::new();
        assert!(!e.drain_blocked());
    }

    #[test]
    fn test_no_service() {
        let e = Evaporator::new();
        assert!(!e.needs_service());
    }

    #[test]
    fn test_frost() {
        let mut e = Evaporator::new();
        e.frost_detected = true;
        assert!(e.needs_service());
    }

    #[test]
    fn test_health() {
        let e = Evaporator::new();
        assert!((e.health_score() - 100.0).abs() < 0.1);
    }
}
