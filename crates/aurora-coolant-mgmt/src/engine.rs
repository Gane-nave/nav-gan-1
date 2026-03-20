/// Coolant management: temperature regulation, thermostat control, flow monitoring
/// Phase 159

#[derive(Debug, Clone)]
pub struct CoolantSystem {
    pub temp_c: f64,
    pub target_temp_c: f64,
    pub flow_rate_lpm: f64,
    pub level_pct: f64,
    pub thermostat_open: bool,
    pub fan_speed_pct: f64,
}

impl Default for CoolantSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl CoolantSystem {
    pub fn new() -> Self {
        Self {
            temp_c: 85.0,
            target_temp_c: 90.0,
            flow_rate_lpm: 10.0,
            level_pct: 100.0,
            thermostat_open: true,
            fan_speed_pct: 30.0,
        }
    }

    pub fn is_overheating(&self) -> bool {
        self.temp_c > 110.0
    }

    pub fn is_cold(&self) -> bool {
        self.temp_c < 60.0
    }

    pub fn at_operating_temp(&self) -> bool {
        (80.0..=100.0).contains(&self.temp_c)
    }

    pub fn level_low(&self) -> bool {
        self.level_pct < 20.0
    }

    pub fn needs_more_cooling(&self) -> bool {
        self.temp_c > self.target_temp_c + 5.0
    }

    pub fn recommended_fan_pct(&self) -> f64 {
        if self.temp_c > 105.0 {
            100.0
        } else if self.temp_c > 95.0 {
            70.0
        } else if self.temp_c > 85.0 {
            40.0
        } else {
            0.0
        }
    }

    pub fn health_score(&self) -> f64 {
        let temp_score = if self.at_operating_temp() { 40.0 } else { 20.0 };
        let level_score = (self.level_pct / 100.0) * 30.0;
        let flow_score = if self.flow_rate_lpm > 5.0 { 30.0 } else { 15.0 };
        temp_score + level_score + flow_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operating_temp() {
        let s = CoolantSystem::new();
        assert!(s.at_operating_temp());
    }

    #[test]
    fn test_overheating() {
        let mut s = CoolantSystem::new();
        s.temp_c = 115.0;
        assert!(s.is_overheating());
    }

    #[test]
    fn test_cold() {
        let mut s = CoolantSystem::new();
        s.temp_c = 40.0;
        assert!(s.is_cold());
    }

    #[test]
    fn test_level_ok() {
        let s = CoolantSystem::new();
        assert!(!s.level_low());
    }

    #[test]
    fn test_level_low() {
        let mut s = CoolantSystem::new();
        s.level_pct = 10.0;
        assert!(s.level_low());
    }

    #[test]
    fn test_fan_recommendation() {
        let mut s = CoolantSystem::new();
        s.temp_c = 108.0;
        assert!((s.recommended_fan_pct() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_health_score() {
        let s = CoolantSystem::new();
        assert!(s.health_score() > 80.0);
    }

    #[test]
    fn test_needs_cooling() {
        let mut s = CoolantSystem::new();
        s.temp_c = 100.0;
        assert!(s.needs_more_cooling());
    }
}
