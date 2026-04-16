/// Oil monitoring: level, quality, pressure, temperature, change intervals
/// Phase 160

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OilQuality {
    Fresh,
    Good,
    Fair,
    Degraded,
    Critical,
}

impl OilQuality {
    pub fn remaining_life_pct(&self) -> f64 {
        match self {
            OilQuality::Fresh => 100.0,
            OilQuality::Good => 75.0,
            OilQuality::Fair => 50.0,
            OilQuality::Degraded => 20.0,
            OilQuality::Critical => 5.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OilSystem {
    pub level_pct: f64,
    pub quality: OilQuality,
    pub pressure_psi: f64,
    pub temp_c: f64,
    pub km_since_change: f64,
    pub change_interval_km: f64,
}

impl Default for OilSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl OilSystem {
    pub fn new() -> Self {
        Self {
            level_pct: 100.0,
            quality: OilQuality::Good,
            pressure_psi: 40.0,
            temp_c: 95.0,
            km_since_change: 3000.0,
            change_interval_km: 10000.0,
        }
    }

    pub fn level_low(&self) -> bool {
        self.level_pct < 25.0
    }

    pub fn pressure_low(&self) -> bool {
        self.pressure_psi < 20.0
    }

    pub fn is_overheated(&self) -> bool {
        self.temp_c > 130.0
    }

    pub fn needs_change(&self) -> bool {
        self.km_since_change >= self.change_interval_km
            || matches!(self.quality, OilQuality::Critical)
    }

    pub fn km_until_change(&self) -> f64 {
        (self.change_interval_km - self.km_since_change).max(0.0)
    }

    pub fn health_score(&self) -> f64 {
        let level_s = (self.level_pct / 100.0).min(1.0) * 25.0;
        let quality_s = self.quality.remaining_life_pct() / 100.0 * 25.0;
        let pressure_s = if self.pressure_low() { 10.0 } else { 25.0 };
        let temp_s = if self.is_overheated() { 10.0 } else { 25.0 };
        level_s + quality_s + pressure_s + temp_s
    }

    pub fn any_warning(&self) -> bool {
        self.level_low() || self.pressure_low() || self.is_overheated() || self.needs_change()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_ok() {
        let s = OilSystem::new();
        assert!(!s.level_low());
    }

    #[test]
    fn test_level_low() {
        let mut s = OilSystem::new();
        s.level_pct = 15.0;
        assert!(s.level_low());
    }

    #[test]
    fn test_pressure_ok() {
        let s = OilSystem::new();
        assert!(!s.pressure_low());
    }

    #[test]
    fn test_pressure_low() {
        let mut s = OilSystem::new();
        s.pressure_psi = 10.0;
        assert!(s.pressure_low());
    }

    #[test]
    fn test_needs_change() {
        let mut s = OilSystem::new();
        s.km_since_change = 11000.0;
        assert!(s.needs_change());
    }

    #[test]
    fn test_no_change_needed() {
        let s = OilSystem::new();
        assert!(!s.needs_change());
    }

    #[test]
    fn test_km_until_change() {
        let s = OilSystem::new();
        assert!((s.km_until_change() - 7000.0).abs() < 1.0);
    }

    #[test]
    fn test_health_score() {
        let s = OilSystem::new();
        assert!(s.health_score() > 80.0);
    }

    #[test]
    fn test_no_warning() {
        let s = OilSystem::new();
        assert!(!s.any_warning());
    }
}
