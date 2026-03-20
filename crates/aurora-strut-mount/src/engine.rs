/// Strut mount monitoring: bearing wear, noise, alignment effect
/// Phase 199

#[derive(Debug, Clone)]
pub struct StrutMount {
    pub position: String,
    pub bearing_wear_pct: f64,
    pub clunking_detected: bool,
    pub alignment_offset_deg: f64,
    pub mileage_km: f64,
}

impl Default for StrutMount {
    fn default() -> Self {
        Self::new()
    }
}

impl StrutMount {
    pub fn new() -> Self {
        Self {
            position: "front_left".into(),
            bearing_wear_pct: 15.0,
            clunking_detected: false,
            alignment_offset_deg: 0.1,
            mileage_km: 80000.0,
        }
    }

    pub fn bearing_ok(&self) -> bool {
        self.bearing_wear_pct < 60.0
    }

    pub fn affects_alignment(&self) -> bool {
        self.alignment_offset_deg.abs() > 0.5
    }

    pub fn needs_replacement(&self) -> bool {
        self.bearing_wear_pct > 75.0 || self.clunking_detected
    }

    pub fn remaining_life_pct(&self) -> f64 {
        (100.0 - self.bearing_wear_pct).max(0.0)
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if !self.bearing_ok() {
            score -= 30.0;
        }
        if self.clunking_detected {
            score -= 30.0;
        }
        if self.affects_alignment() {
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
        let s = StrutMount::new();
        assert!(!s.needs_replacement());
    }

    #[test]
    fn test_bearing_ok() {
        let s = StrutMount::new();
        assert!(s.bearing_ok());
    }

    #[test]
    fn test_no_alignment_effect() {
        let s = StrutMount::new();
        assert!(!s.affects_alignment());
    }

    #[test]
    fn test_remaining_life() {
        let s = StrutMount::new();
        assert!((s.remaining_life_pct() - 85.0).abs() < 0.1);
    }

    #[test]
    fn test_clunking() {
        let mut s = StrutMount::new();
        s.clunking_detected = true;
        assert!(s.needs_replacement());
    }

    #[test]
    fn test_health() {
        let s = StrutMount::new();
        assert!((s.health_score() - 100.0).abs() < 0.1);
    }
}
