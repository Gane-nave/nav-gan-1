/// Regenerative braking: energy recovery, brake blending, one-pedal driving
/// Phase 164

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegenLevel {
    Off,
    Low,
    Medium,
    High,
    Max,
}

impl RegenLevel {
    pub fn decel_g(&self) -> f64 {
        match self {
            RegenLevel::Off => 0.0,
            RegenLevel::Low => 0.05,
            RegenLevel::Medium => 0.1,
            RegenLevel::High => 0.2,
            RegenLevel::Max => 0.3,
        }
    }

    pub fn recovery_efficiency_pct(&self) -> f64 {
        match self {
            RegenLevel::Off => 0.0,
            RegenLevel::Low => 60.0,
            RegenLevel::Medium => 70.0,
            RegenLevel::High => 75.0,
            RegenLevel::Max => 80.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegenBrakeSystem {
    pub level: RegenLevel,
    pub one_pedal_mode: bool,
    pub battery_soc_pct: f64,
    pub energy_recovered_kwh: f64,
    pub speed_kmh: f64,
}

impl Default for RegenBrakeSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl RegenBrakeSystem {
    pub fn new() -> Self {
        Self {
            level: RegenLevel::Medium,
            one_pedal_mode: false,
            battery_soc_pct: 60.0,
            energy_recovered_kwh: 0.0,
            speed_kmh: 0.0,
        }
    }

    pub fn can_regen(&self) -> bool {
        self.battery_soc_pct < 95.0 && self.speed_kmh > 5.0
    }

    pub fn current_recovery_kw(&self) -> f64 {
        if !self.can_regen() {
            return 0.0;
        }
        let speed_factor = (self.speed_kmh / 100.0).min(1.0);
        let base_kw = self.level.decel_g() * 100.0;
        base_kw * speed_factor * self.level.recovery_efficiency_pct() / 100.0
    }

    pub fn battery_nearly_full(&self) -> bool {
        self.battery_soc_pct > 90.0
    }

    pub fn range_extension_km(&self) -> f64 {
        self.energy_recovered_kwh * 5.0
    }

    pub fn effective_one_pedal(&self) -> bool {
        self.one_pedal_mode && matches!(self.level, RegenLevel::High | RegenLevel::Max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decel_levels() {
        assert!(RegenLevel::Max.decel_g() > RegenLevel::Low.decel_g());
    }

    #[test]
    fn test_efficiency() {
        assert!(RegenLevel::Max.recovery_efficiency_pct() > 70.0);
    }

    #[test]
    fn test_can_regen() {
        let mut s = RegenBrakeSystem::new();
        s.speed_kmh = 50.0;
        assert!(s.can_regen());
    }

    #[test]
    fn test_no_regen_full_battery() {
        let mut s = RegenBrakeSystem::new();
        s.battery_soc_pct = 98.0;
        s.speed_kmh = 50.0;
        assert!(!s.can_regen());
    }

    #[test]
    fn test_recovery_kw() {
        let mut s = RegenBrakeSystem::new();
        s.speed_kmh = 80.0;
        assert!(s.current_recovery_kw() > 0.0);
    }

    #[test]
    fn test_range_extension() {
        let mut s = RegenBrakeSystem::new();
        s.energy_recovered_kwh = 2.0;
        assert!((s.range_extension_km() - 10.0).abs() < 0.1);
    }

    #[test]
    fn test_one_pedal() {
        let mut s = RegenBrakeSystem::new();
        s.one_pedal_mode = true;
        s.level = RegenLevel::High;
        assert!(s.effective_one_pedal());
    }

    #[test]
    fn test_no_one_pedal_low() {
        let mut s = RegenBrakeSystem::new();
        s.one_pedal_mode = true;
        s.level = RegenLevel::Low;
        assert!(!s.effective_one_pedal());
    }
}
