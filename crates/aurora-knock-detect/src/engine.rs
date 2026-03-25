/// Knock detection: detonation sensing, timing retard, octane adaptation
/// Phase 212

#[derive(Debug, Clone)]
pub struct KnockDetector {
    pub knock_intensity: f64,
    pub knock_count: u32,
    pub timing_retard_deg: f64,
    pub octane_adaptation: f64,
    pub threshold: f64,
    pub sensor_ok: bool,
}

impl Default for KnockDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl KnockDetector {
    pub fn new() -> Self {
        Self {
            knock_intensity: 0.1,
            knock_count: 0,
            timing_retard_deg: 0.0,
            octane_adaptation: 0.0,
            threshold: 1.0,
            sensor_ok: true,
        }
    }

    pub fn knock_detected(&self) -> bool {
        self.knock_intensity > self.threshold
    }

    pub fn severe_knock(&self) -> bool {
        self.knock_intensity > self.threshold * 2.0
    }

    pub fn timing_adjusted(&self) -> bool {
        self.timing_retard_deg > 0.5
    }

    pub fn needs_fuel_upgrade(&self) -> bool {
        self.octane_adaptation < -5.0
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if !self.sensor_ok {
            score -= 40.0;
        }
        if self.knock_detected() {
            score -= 20.0;
        }
        if self.severe_knock() {
            score -= 30.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_knock() {
        let k = KnockDetector::new();
        assert!(!k.knock_detected());
    }

    #[test]
    fn test_no_severe() {
        let k = KnockDetector::new();
        assert!(!k.severe_knock());
    }

    #[test]
    fn test_no_timing_adjust() {
        let k = KnockDetector::new();
        assert!(!k.timing_adjusted());
    }

    #[test]
    fn test_no_fuel_upgrade() {
        let k = KnockDetector::new();
        assert!(!k.needs_fuel_upgrade());
    }

    #[test]
    fn test_knock() {
        let mut k = KnockDetector::new();
        k.knock_intensity = 2.0;
        assert!(k.knock_detected());
    }

    #[test]
    fn test_health() {
        let k = KnockDetector::new();
        assert!((k.health_score() - 100.0).abs() < 0.1);
    }
}
