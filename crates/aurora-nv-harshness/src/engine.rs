/// NVH (Noise, Vibration, Harshness): cabin comfort, frequency analysis
/// Phase 365

#[derive(Debug, Clone)]
pub struct NvHarshness {
    pub noise_db: f64,
    pub vibration_score: f64,
    pub harshness_score: f64,
    pub target_db: f64,
    pub compliant: bool,
}

impl Default for NvHarshness {
    fn default() -> Self {
        Self::new()
    }
}

impl NvHarshness {
    pub fn new() -> Self {
        Self {
            noise_db: 38.0,
            vibration_score: 90.0,
            harshness_score: 85.0,
            target_db: 42.0,
            compliant: true,
        }
    }

    pub fn noise_ok(&self) -> bool {
        self.noise_db <= self.target_db
    }

    pub fn comfort_ok(&self) -> bool {
        self.vibration_score > 80.0 && self.harshness_score > 80.0
    }

    pub fn overall_score(&self) -> f64 {
        (self.vibration_score + self.harshness_score) / 2.0
    }

    pub fn premium_quality(&self) -> bool {
        self.noise_db < 35.0 && self.overall_score() > 90.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.noise_ok() {
            return 30.0;
        }
        if !self.comfort_ok() {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noise() {
        let n = NvHarshness::new();
        assert!(n.noise_ok());
    }

    #[test]
    fn test_comfort() {
        let n = NvHarshness::new();
        assert!(n.comfort_ok());
    }

    #[test]
    fn test_score() {
        let n = NvHarshness::new();
        assert!(n.overall_score() > 85.0);
    }

    #[test]
    fn test_not_premium() {
        let n = NvHarshness::new();
        assert!(!n.premium_quality());
    }

    #[test]
    fn test_loud() {
        let mut n = NvHarshness::new();
        n.noise_db = 50.0;
        assert!(!n.noise_ok());
    }

    #[test]
    fn test_health() {
        let n = NvHarshness::new();
        assert!((n.health_score() - 100.0).abs() < 0.1);
    }
}
