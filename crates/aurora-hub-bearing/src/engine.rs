/// Hub bearing monitoring: noise, play, temperature, ABS tone ring
/// Phase 198

#[derive(Debug, Clone)]
pub struct HubBearing {
    pub position: String,
    pub noise_level_db: f64,
    pub play_mm: f64,
    pub temp_c: f64,
    pub abs_ring_ok: bool,
    pub mileage_km: f64,
}

impl Default for HubBearing {
    fn default() -> Self {
        Self::new()
    }
}

impl HubBearing {
    pub fn new() -> Self {
        Self {
            position: "front_left".into(),
            noise_level_db: 20.0,
            play_mm: 0.02,
            temp_c: 40.0,
            abs_ring_ok: true,
            mileage_km: 60000.0,
        }
    }

    pub fn noise_ok(&self) -> bool {
        self.noise_level_db < 50.0
    }

    pub fn play_ok(&self) -> bool {
        self.play_mm < 0.1
    }

    pub fn overheating(&self) -> bool {
        self.temp_c > 100.0
    }

    pub fn needs_replacement(&self) -> bool {
        !self.noise_ok() || !self.play_ok() || !self.abs_ring_ok
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if !self.noise_ok() {
            score -= 30.0;
        }
        if !self.play_ok() {
            score -= 30.0;
        }
        if self.overheating() {
            score -= 20.0;
        }
        if !self.abs_ring_ok {
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
        let h = HubBearing::new();
        assert!(!h.needs_replacement());
    }

    #[test]
    fn test_noise_ok() {
        let h = HubBearing::new();
        assert!(h.noise_ok());
    }

    #[test]
    fn test_play_ok() {
        let h = HubBearing::new();
        assert!(h.play_ok());
    }

    #[test]
    fn test_not_overheating() {
        let h = HubBearing::new();
        assert!(!h.overheating());
    }

    #[test]
    fn test_noisy() {
        let mut h = HubBearing::new();
        h.noise_level_db = 60.0;
        assert!(h.needs_replacement());
    }

    #[test]
    fn test_health() {
        let h = HubBearing::new();
        assert!((h.health_score() - 100.0).abs() < 0.1);
    }
}
