/// Bearing noise: rolling element, race defect, envelope analysis
/// Phase 380

#[derive(Debug, Clone)]
pub struct BearingNoise {
    pub vibration_mm_s: f64,
    pub max_vibration_mm_s: f64,
    pub defect_detected: bool,
    pub race_ok: bool,
    pub lubrication_ok: bool,
}

impl Default for BearingNoise {
    fn default() -> Self {
        Self::new()
    }
}

impl BearingNoise {
    pub fn new() -> Self {
        Self {
            vibration_mm_s: 0.5,
            max_vibration_mm_s: 4.0,
            defect_detected: false,
            race_ok: true,
            lubrication_ok: true,
        }
    }

    pub fn vibration_ok(&self) -> bool {
        self.vibration_mm_s < self.max_vibration_mm_s
    }

    pub fn all_ok(&self) -> bool {
        !self.defect_detected && self.race_ok && self.lubrication_ok
    }

    pub fn needs_replacement(&self) -> bool {
        self.defect_detected || !self.race_ok
    }

    pub fn severity_pct(&self) -> f64 {
        if self.max_vibration_mm_s <= 0.0 {
            return 0.0;
        }
        (self.vibration_mm_s / self.max_vibration_mm_s * 100.0).clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        if self.defect_detected {
            return 0.0;
        }
        if !self.race_ok {
            return 20.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vibration() {
        let b = BearingNoise::new();
        assert!(b.vibration_ok());
    }

    #[test]
    fn test_all_ok() {
        let b = BearingNoise::new();
        assert!(b.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let b = BearingNoise::new();
        assert!(!b.needs_replacement());
    }

    #[test]
    fn test_severity() {
        let b = BearingNoise::new();
        assert!(b.severity_pct() < 20.0);
    }

    #[test]
    fn test_defect() {
        let mut b = BearingNoise::new();
        b.defect_detected = true;
        assert!(b.needs_replacement());
    }

    #[test]
    fn test_health() {
        let b = BearingNoise::new();
        assert!((b.health_score() - 100.0).abs() < 0.1);
    }
}
