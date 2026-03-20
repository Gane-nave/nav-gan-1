/// Fuel pump monitoring: pressure, flow rate, priming, relay status
/// Phase 218

#[derive(Debug, Clone)]
pub struct FuelPump {
    pub pressure_bar: f64,
    pub target_pressure_bar: f64,
    pub flow_rate_lph: f64,
    pub current_draw_a: f64,
    pub relay_ok: bool,
    pub running: bool,
}

impl Default for FuelPump {
    fn default() -> Self {
        Self::new()
    }
}

impl FuelPump {
    pub fn new() -> Self {
        Self {
            pressure_bar: 3.5,
            target_pressure_bar: 3.5,
            flow_rate_lph: 80.0,
            current_draw_a: 5.0,
            relay_ok: true,
            running: true,
        }
    }

    pub fn pressure_ok(&self) -> bool {
        (self.pressure_bar - self.target_pressure_bar).abs() < 0.5
    }

    pub fn flow_ok(&self) -> bool {
        self.flow_rate_lph > 20.0
    }

    pub fn current_ok(&self) -> bool {
        self.current_draw_a > 2.0 && self.current_draw_a < 12.0
    }

    pub fn needs_attention(&self) -> bool {
        !self.pressure_ok() || !self.flow_ok() || !self.relay_ok
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;
        if !self.relay_ok {
            score -= 40.0;
        }
        if !self.pressure_ok() {
            score -= 25.0;
        }
        if !self.flow_ok() {
            score -= 25.0;
        }
        if !self.current_ok() {
            score -= 15.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure_ok() {
        let f = FuelPump::new();
        assert!(f.pressure_ok());
    }

    #[test]
    fn test_flow_ok() {
        let f = FuelPump::new();
        assert!(f.flow_ok());
    }

    #[test]
    fn test_current_ok() {
        let f = FuelPump::new();
        assert!(f.current_ok());
    }

    #[test]
    fn test_no_attention() {
        let f = FuelPump::new();
        assert!(!f.needs_attention());
    }

    #[test]
    fn test_relay_fail() {
        let mut f = FuelPump::new();
        f.relay_ok = false;
        assert!(f.needs_attention());
    }

    #[test]
    fn test_health() {
        let f = FuelPump::new();
        assert!((f.health_score() - 100.0).abs() < 0.1);
    }
}
