/// HVAC noise: blower motor, duct resonance, register whistle
/// Phase 375

#[derive(Debug, Clone)]
pub struct HvacNoise {
    pub blower_db: f64,
    pub duct_resonance: bool,
    pub register_whistle: bool,
    pub fan_speed: u8,
    pub max_fan: u8,
}

impl Default for HvacNoise {
    fn default() -> Self {
        Self::new()
    }
}

impl HvacNoise {
    pub fn new() -> Self {
        Self {
            blower_db: 30.0,
            duct_resonance: false,
            register_whistle: false,
            fan_speed: 3,
            max_fan: 7,
        }
    }

    pub fn acceptable(&self) -> bool {
        self.blower_db < 40.0 && !self.register_whistle
    }

    pub fn quiet(&self) -> bool {
        self.blower_db < 25.0
    }

    pub fn issues(&self) -> bool {
        self.duct_resonance || self.register_whistle
    }

    pub fn fan_pct(&self) -> f64 {
        if self.max_fan == 0 {
            return 0.0;
        }
        f64::from(self.fan_speed) / f64::from(self.max_fan) * 100.0
    }

    pub fn health_score(&self) -> f64 {
        if self.register_whistle {
            return 30.0;
        }
        if self.duct_resonance {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acceptable() {
        let h = HvacNoise::new();
        assert!(h.acceptable());
    }

    #[test]
    fn test_not_quiet() {
        let h = HvacNoise::new();
        assert!(!h.quiet());
    }

    #[test]
    fn test_no_issues() {
        let h = HvacNoise::new();
        assert!(!h.issues());
    }

    #[test]
    fn test_fan_pct() {
        let h = HvacNoise::new();
        assert!(h.fan_pct() < 50.0);
    }

    #[test]
    fn test_whistle() {
        let mut h = HvacNoise::new();
        h.register_whistle = true;
        assert!(h.issues());
    }

    #[test]
    fn test_health() {
        let h = HvacNoise::new();
        assert!((h.health_score() - 100.0).abs() < 0.1);
    }
}
