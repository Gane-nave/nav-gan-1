/// Muffler monitoring: backpressure, leak detection, noise levels
/// Phase 195

#[derive(Debug, Clone)]
pub struct MufflerSystem {
    pub backpressure_kpa: f64,
    pub noise_db: f64,
    pub exhaust_temp_c: f64,
    pub leak_detected: bool,
}

impl Default for MufflerSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl MufflerSystem {
    pub fn new() -> Self {
        Self {
            backpressure_kpa: 5.0,
            noise_db: 70.0,
            exhaust_temp_c: 300.0,
            leak_detected: false,
        }
    }

    pub fn backpressure_ok(&self) -> bool {
        self.backpressure_kpa < 10.0
    }

    pub fn noise_ok(&self) -> bool {
        self.noise_db < 85.0
    }

    pub fn overheating(&self) -> bool {
        self.exhaust_temp_c > 800.0
    }

    pub fn needs_repair(&self) -> bool {
        self.leak_detected || !self.backpressure_ok() || !self.noise_ok()
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if self.leak_detected {
            score -= 40.0;
        }
        if !self.backpressure_ok() {
            score -= 20.0;
        }
        if !self.noise_ok() {
            score -= 20.0;
        }
        if self.overheating() {
            score -= 20.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_healthy() {
        let m = MufflerSystem::new();
        assert!(!m.needs_repair());
    }

    #[test]
    fn test_backpressure_ok() {
        let m = MufflerSystem::new();
        assert!(m.backpressure_ok());
    }

    #[test]
    fn test_noise_ok() {
        let m = MufflerSystem::new();
        assert!(m.noise_ok());
    }

    #[test]
    fn test_not_overheating() {
        let m = MufflerSystem::new();
        assert!(!m.overheating());
    }

    #[test]
    fn test_leak() {
        let mut m = MufflerSystem::new();
        m.leak_detected = true;
        assert!(m.needs_repair());
    }

    #[test]
    fn test_health() {
        let m = MufflerSystem::new();
        assert!((m.health_score() - 100.0).abs() < 0.1);
    }
}
