/// Amplifier monitoring: power output, thermal protection, distortion
/// Phase 432

#[derive(Debug, Clone)]
pub struct AmplifierMon {
    pub power_w: f64,
    pub max_power_w: f64,
    pub temp_c: f64,
    pub max_temp_c: f64,
    pub distortion_pct: f64,
}

impl Default for AmplifierMon {
    fn default() -> Self {
        Self::new()
    }
}

impl AmplifierMon {
    pub fn new() -> Self {
        Self {
            power_w: 150.0,
            max_power_w: 300.0,
            temp_c: 45.0,
            max_temp_c: 80.0,
            distortion_pct: 0.5,
        }
    }

    pub fn power_ok(&self) -> bool {
        self.power_w <= self.max_power_w
    }

    pub fn temp_ok(&self) -> bool {
        self.temp_c < self.max_temp_c
    }

    pub fn clean_output(&self) -> bool {
        self.distortion_pct < 1.0
    }

    pub fn needs_service(&self) -> bool {
        self.temp_c > self.max_temp_c * 0.9 || self.distortion_pct > 5.0
    }

    pub fn health_score(&self) -> f64 {
        if !self.temp_ok() {
            return 10.0;
        }
        if !self.clean_output() {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power() {
        let a = AmplifierMon::new();
        assert!(a.power_ok());
    }

    #[test]
    fn test_temp() {
        let a = AmplifierMon::new();
        assert!(a.temp_ok());
    }

    #[test]
    fn test_clean() {
        let a = AmplifierMon::new();
        assert!(a.clean_output());
    }

    #[test]
    fn test_no_service() {
        let a = AmplifierMon::new();
        assert!(!a.needs_service());
    }

    #[test]
    fn test_overheating() {
        let mut a = AmplifierMon::new();
        a.temp_c = 85.0;
        assert!(!a.temp_ok());
    }

    #[test]
    fn test_health() {
        let a = AmplifierMon::new();
        assert!((a.health_score() - 100.0).abs() < 0.1);
    }
}
