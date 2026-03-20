/// Tie rod monitoring: end play, boot condition, steering response
/// Phase 202

#[derive(Debug, Clone)]
pub struct TieRod {
    pub side: String,
    pub end_play_mm: f64,
    pub boot_intact: bool,
    pub steering_play_deg: f64,
    pub mileage_km: f64,
}

impl Default for TieRod {
    fn default() -> Self {
        Self::new()
    }
}

impl TieRod {
    pub fn new() -> Self {
        Self {
            side: "left".into(),
            end_play_mm: 0.2,
            boot_intact: true,
            steering_play_deg: 1.0,
            mileage_km: 65000.0,
        }
    }

    pub fn end_play_ok(&self) -> bool {
        self.end_play_mm < 1.0
    }

    pub fn steering_tight(&self) -> bool {
        self.steering_play_deg < 3.0
    }

    pub fn needs_replacement(&self) -> bool {
        self.end_play_mm > 2.0 || !self.boot_intact || self.steering_play_deg > 5.0
    }

    pub fn affects_alignment(&self) -> bool {
        self.end_play_mm > 0.5
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if !self.end_play_ok() {
            score -= 30.0;
        }
        if !self.boot_intact {
            score -= 25.0;
        }
        if !self.steering_tight() {
            score -= 25.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_healthy() {
        let t = TieRod::new();
        assert!(!t.needs_replacement());
    }

    #[test]
    fn test_end_play_ok() {
        let t = TieRod::new();
        assert!(t.end_play_ok());
    }

    #[test]
    fn test_steering_tight() {
        let t = TieRod::new();
        assert!(t.steering_tight());
    }

    #[test]
    fn test_no_alignment_effect() {
        let t = TieRod::new();
        assert!(!t.affects_alignment());
    }

    #[test]
    fn test_torn_boot() {
        let mut t = TieRod::new();
        t.boot_intact = false;
        assert!(t.needs_replacement());
    }

    #[test]
    fn test_health() {
        let t = TieRod::new();
        assert!((t.health_score() - 100.0).abs() < 0.1);
    }
}
