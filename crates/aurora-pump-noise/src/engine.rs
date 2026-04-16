/// Pump noise: hydraulic pump, fuel pump, water pump noise isolation
/// Phase 378

#[derive(Debug, Clone)]
pub struct PumpNoise {
    pub noise_db: f64,
    pub max_db: f64,
    pub pulsation_ok: bool,
    pub mount_ok: bool,
    pub cavitation: bool,
}

impl Default for PumpNoise {
    fn default() -> Self {
        Self::new()
    }
}

impl PumpNoise {
    pub fn new() -> Self {
        Self {
            noise_db: 25.0,
            max_db: 40.0,
            pulsation_ok: true,
            mount_ok: true,
            cavitation: false,
        }
    }

    pub fn noise_ok(&self) -> bool {
        self.noise_db < self.max_db
    }

    pub fn all_ok(&self) -> bool {
        self.noise_ok() && self.pulsation_ok && self.mount_ok && !self.cavitation
    }

    pub fn needs_service(&self) -> bool {
        self.cavitation || !self.mount_ok
    }

    pub fn noise_pct(&self) -> f64 {
        if self.max_db <= 0.0 {
            return 0.0;
        }
        (self.noise_db / self.max_db * 100.0).clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        if self.cavitation {
            return 0.0;
        }
        if !self.mount_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noise() {
        let p = PumpNoise::new();
        assert!(p.noise_ok());
    }

    #[test]
    fn test_all_ok() {
        let p = PumpNoise::new();
        assert!(p.all_ok());
    }

    #[test]
    fn test_no_service() {
        let p = PumpNoise::new();
        assert!(!p.needs_service());
    }

    #[test]
    fn test_noise_pct() {
        let p = PumpNoise::new();
        assert!(p.noise_pct() < 70.0);
    }

    #[test]
    fn test_cavitation() {
        let mut p = PumpNoise::new();
        p.cavitation = true;
        assert!(p.needs_service());
    }

    #[test]
    fn test_health() {
        let p = PumpNoise::new();
        assert!((p.health_score() - 100.0).abs() < 0.1);
    }
}
