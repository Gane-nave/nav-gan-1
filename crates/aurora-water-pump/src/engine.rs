/// Water pump monitoring: flow rate, bearing condition, impeller wear
/// Phase 219

#[derive(Debug, Clone)]
pub struct WaterPump {
    pub flow_rate_lpm: f64,
    pub bearing_noise_db: f64,
    pub impeller_wear_pct: f64,
    pub leak_detected: bool,
    pub electric: bool,
    pub rpm: f64,
}

impl Default for WaterPump {
    fn default() -> Self {
        Self::new()
    }
}

impl WaterPump {
    pub fn new() -> Self {
        Self {
            flow_rate_lpm: 60.0,
            bearing_noise_db: 15.0,
            impeller_wear_pct: 10.0,
            leak_detected: false,
            electric: false,
            rpm: 3000.0,
        }
    }

    pub fn flow_ok(&self) -> bool {
        self.flow_rate_lpm > 20.0
    }

    pub fn bearing_ok(&self) -> bool {
        self.bearing_noise_db < 40.0
    }

    pub fn impeller_ok(&self) -> bool {
        self.impeller_wear_pct < 50.0
    }

    pub fn needs_replacement(&self) -> bool {
        self.leak_detected || !self.bearing_ok() || self.impeller_wear_pct > 70.0
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if self.leak_detected {
            score -= 40.0;
        }
        if !self.bearing_ok() {
            score -= 25.0;
        }
        if !self.impeller_ok() {
            score -= 20.0;
        }
        if !self.flow_ok() {
            score -= 15.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow_ok() {
        let w = WaterPump::new();
        assert!(w.flow_ok());
    }

    #[test]
    fn test_bearing_ok() {
        let w = WaterPump::new();
        assert!(w.bearing_ok());
    }

    #[test]
    fn test_impeller_ok() {
        let w = WaterPump::new();
        assert!(w.impeller_ok());
    }

    #[test]
    fn test_no_replacement() {
        let w = WaterPump::new();
        assert!(!w.needs_replacement());
    }

    #[test]
    fn test_leak() {
        let mut w = WaterPump::new();
        w.leak_detected = true;
        assert!(w.needs_replacement());
    }

    #[test]
    fn test_health() {
        let w = WaterPump::new();
        assert!((w.health_score() - 100.0).abs() < 0.1);
    }
}
