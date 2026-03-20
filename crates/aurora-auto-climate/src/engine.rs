/// Auto climate: automatic climate control, multi-zone, air quality
/// Phase 451

#[derive(Debug, Clone)]
pub struct AutoClimate {
    pub target_c: f64,
    pub actual_c: f64,
    pub zone_count: u8,
    pub auto_mode: bool,
    pub air_quality_ok: bool,
}

impl Default for AutoClimate {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoClimate {
    pub fn new() -> Self {
        Self {
            target_c: 22.0,
            actual_c: 22.5,
            zone_count: 2,
            auto_mode: true,
            air_quality_ok: true,
        }
    }

    pub fn at_target(&self) -> bool {
        (self.actual_c - self.target_c).abs() < 2.0
    }

    pub fn all_ok(&self) -> bool {
        self.at_target() && self.air_quality_ok
    }

    pub fn multi_zone(&self) -> bool {
        self.zone_count > 1
    }

    pub fn needs_attention(&self) -> bool {
        !self.at_target() && self.auto_mode
    }

    pub fn health_score(&self) -> f64 {
        if !self.air_quality_ok {
            return 40.0;
        }
        if !self.at_target() {
            return 60.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target() {
        let a = AutoClimate::new();
        assert!(a.at_target());
    }

    #[test]
    fn test_all_ok() {
        let a = AutoClimate::new();
        assert!(a.all_ok());
    }

    #[test]
    fn test_multi_zone() {
        let a = AutoClimate::new();
        assert!(a.multi_zone());
    }

    #[test]
    fn test_no_attention() {
        let a = AutoClimate::new();
        assert!(!a.needs_attention());
    }

    #[test]
    fn test_off_target() {
        let mut a = AutoClimate::new();
        a.actual_c = 30.0;
        assert!(a.needs_attention());
    }

    #[test]
    fn test_health() {
        let a = AutoClimate::new();
        assert!((a.health_score() - 100.0).abs() < 0.1);
    }
}
