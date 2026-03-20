/// Resonator: Helmholtz, quarter-wave, acoustic tuning
/// Phase 381

#[derive(Debug, Clone)]
pub struct Resonator {
    pub target_freq_hz: f64,
    pub bandwidth_hz: f64,
    pub attenuation_db: f64,
    pub intact: bool,
    pub tuned: bool,
}

impl Default for Resonator {
    fn default() -> Self {
        Self::new()
    }
}

impl Resonator {
    pub fn new() -> Self {
        Self {
            target_freq_hz: 200.0,
            bandwidth_hz: 50.0,
            attenuation_db: 15.0,
            intact: true,
            tuned: true,
        }
    }

    pub fn effective(&self) -> bool {
        self.attenuation_db > 10.0 && self.intact && self.tuned
    }

    pub fn freq_range(&self) -> (f64, f64) {
        (
            self.target_freq_hz - self.bandwidth_hz / 2.0,
            self.target_freq_hz + self.bandwidth_hz / 2.0,
        )
    }

    pub fn needs_service(&self) -> bool {
        !self.intact || !self.tuned
    }

    pub fn broadband(&self) -> bool {
        self.bandwidth_hz > 100.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.intact {
            return 0.0;
        }
        if !self.tuned {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effective() {
        let r = Resonator::new();
        assert!(r.effective());
    }

    #[test]
    fn test_range() {
        let r = Resonator::new();
        let (lo, hi) = r.freq_range();
        assert!(lo < 200.0 && hi > 200.0);
    }

    #[test]
    fn test_no_service() {
        let r = Resonator::new();
        assert!(!r.needs_service());
    }

    #[test]
    fn test_not_broadband() {
        let r = Resonator::new();
        assert!(!r.broadband());
    }

    #[test]
    fn test_damaged() {
        let mut r = Resonator::new();
        r.intact = false;
        assert!(r.needs_service());
    }

    #[test]
    fn test_health() {
        let r = Resonator::new();
        assert!((r.health_score() - 100.0).abs() < 0.1);
    }
}
