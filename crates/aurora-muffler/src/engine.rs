/// Muffler: sound attenuation, backpressure, corrosion
/// Phase 493

#[derive(Debug, Clone)]
pub struct Muffler {
    pub attenuation_db: f64,
    pub backpressure_kpa: f64,
    pub max_backpressure_kpa: f64,
    pub corroded: bool,
    pub leak_free: bool,
}

impl Default for Muffler {
    fn default() -> Self {
        Self::new()
    }
}

impl Muffler {
    pub fn new() -> Self {
        Self {
            attenuation_db: 25.0,
            backpressure_kpa: 5.0,
            max_backpressure_kpa: 15.0,
            corroded: false,
            leak_free: true,
        }
    }

    pub fn attenuation_ok(&self) -> bool {
        self.attenuation_db > 15.0
    }

    pub fn backpressure_ok(&self) -> bool {
        self.backpressure_kpa < self.max_backpressure_kpa
    }

    pub fn all_ok(&self) -> bool {
        self.attenuation_ok() && self.backpressure_ok() && !self.corroded && self.leak_free
    }

    pub fn needs_replacement(&self) -> bool {
        self.corroded || !self.leak_free
    }

    pub fn health_score(&self) -> f64 {
        if self.corroded { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attenuation() {
        let c = Muffler::new();
        assert!(c.attenuation_ok());
    }

    #[test]
    fn test_backpressure() {
        let c = Muffler::new();
        assert!(c.backpressure_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Muffler::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = Muffler::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_corroded() {
        let mut c = Muffler::new();
        c.corroded = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = Muffler::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
