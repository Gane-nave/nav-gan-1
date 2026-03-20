/// Pitch rate sensor: nose dive/squat measurement, brake dive control
/// Phase 343

#[derive(Debug, Clone)]
pub struct PitchRate {
    pub rate_deg_s: f64,
    pub max_rate_deg_s: f64,
    pub sensor_ok: bool,
    pub calibrated: bool,
}

impl Default for PitchRate {
    fn default() -> Self {
        Self::new()
    }
}

impl PitchRate {
    pub fn new() -> Self {
        Self {
            rate_deg_s: 0.0,
            max_rate_deg_s: 20.0,
            sensor_ok: true,
            calibrated: true,
        }
    }

    pub fn stable(&self) -> bool {
        self.rate_deg_s.abs() < 2.0
    }

    pub fn diving(&self) -> bool {
        self.rate_deg_s < -5.0
    }

    pub fn squatting(&self) -> bool {
        self.rate_deg_s > 5.0
    }

    pub fn ready(&self) -> bool {
        self.sensor_ok && self.calibrated
    }

    pub fn health_score(&self) -> f64 {
        if !self.sensor_ok {
            return 0.0;
        }
        if !self.calibrated {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stable() {
        let p = PitchRate::new();
        assert!(p.stable());
    }

    #[test]
    fn test_not_diving() {
        let p = PitchRate::new();
        assert!(!p.diving());
    }

    #[test]
    fn test_not_squatting() {
        let p = PitchRate::new();
        assert!(!p.squatting());
    }

    #[test]
    fn test_ready() {
        let p = PitchRate::new();
        assert!(p.ready());
    }

    #[test]
    fn test_dive() {
        let mut p = PitchRate::new();
        p.rate_deg_s = -8.0;
        assert!(p.diving());
    }

    #[test]
    fn test_health() {
        let p = PitchRate::new();
        assert!((p.health_score() - 100.0).abs() < 0.1);
    }
}
