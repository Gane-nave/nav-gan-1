/// Fan balance: dynamic balancing, blade condition, vibration measurement
/// Phase 376

#[derive(Debug, Clone)]
pub struct FanBalance {
    pub vibration_mm_s: f64,
    pub max_vibration_mm_s: f64,
    pub blades_ok: bool,
    pub balanced: bool,
    pub rpm: f64,
}

impl Default for FanBalance {
    fn default() -> Self {
        Self::new()
    }
}

impl FanBalance {
    pub fn new() -> Self {
        Self {
            vibration_mm_s: 1.0,
            max_vibration_mm_s: 5.0,
            blades_ok: true,
            balanced: true,
            rpm: 2000.0,
        }
    }

    pub fn vibration_ok(&self) -> bool {
        self.vibration_mm_s < self.max_vibration_mm_s
    }

    pub fn all_ok(&self) -> bool {
        self.blades_ok && self.balanced && self.vibration_ok()
    }

    pub fn needs_balancing(&self) -> bool {
        !self.balanced || self.vibration_mm_s > self.max_vibration_mm_s * 0.7
    }

    pub fn vibration_pct(&self) -> f64 {
        if self.max_vibration_mm_s <= 0.0 {
            return 0.0;
        }
        (self.vibration_mm_s / self.max_vibration_mm_s * 100.0).clamp(0.0, 100.0)
    }

    pub fn health_score(&self) -> f64 {
        if !self.blades_ok {
            return 0.0;
        }
        if !self.balanced {
            return 40.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vibration() {
        let f = FanBalance::new();
        assert!(f.vibration_ok());
    }

    #[test]
    fn test_all_ok() {
        let f = FanBalance::new();
        assert!(f.all_ok());
    }

    #[test]
    fn test_no_balance() {
        let f = FanBalance::new();
        assert!(!f.needs_balancing());
    }

    #[test]
    fn test_vib_pct() {
        let f = FanBalance::new();
        assert!(f.vibration_pct() < 30.0);
    }

    #[test]
    fn test_unbalanced() {
        let mut f = FanBalance::new();
        f.balanced = false;
        assert!(f.needs_balancing());
    }

    #[test]
    fn test_health() {
        let f = FanBalance::new();
        assert!((f.health_score() - 100.0).abs() < 0.1);
    }
}
