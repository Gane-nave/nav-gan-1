/// Heater core: coolant flow, cabin heating, blend door position
/// Phase 237

#[derive(Debug, Clone)]
pub struct HeaterCore {
    pub inlet_temp_c: f64,
    pub outlet_temp_c: f64,
    pub flow_rate_lpm: f64,
    pub blend_door_pct: f64,
    pub leak_detected: bool,
}

impl Default for HeaterCore {
    fn default() -> Self {
        Self::new()
    }
}

impl HeaterCore {
    pub fn new() -> Self {
        Self {
            inlet_temp_c: 85.0,
            outlet_temp_c: 70.0,
            flow_rate_lpm: 10.0,
            blend_door_pct: 50.0,
            leak_detected: false,
        }
    }

    pub fn heat_output_c(&self) -> f64 {
        self.inlet_temp_c - self.outlet_temp_c
    }

    pub fn flow_ok(&self) -> bool {
        self.flow_rate_lpm > 3.0
    }

    pub fn heating_ok(&self) -> bool {
        self.heat_output_c() > 10.0 && self.flow_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        self.leak_detected || !self.flow_ok()
    }

    pub fn health_score(&self) -> f64 {
        if self.leak_detected {
            return 10.0;
        }
        let mut score: f64 = 100.0;
        if !self.flow_ok() {
            score -= 40.0;
        }
        if !self.heating_ok() {
            score -= 20.0;
        }
        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heat_output() {
        let h = HeaterCore::new();
        assert!((h.heat_output_c() - 15.0).abs() < 0.1);
    }

    #[test]
    fn test_flow_ok() {
        let h = HeaterCore::new();
        assert!(h.flow_ok());
    }

    #[test]
    fn test_heating_ok() {
        let h = HeaterCore::new();
        assert!(h.heating_ok());
    }

    #[test]
    fn test_no_replacement() {
        let h = HeaterCore::new();
        assert!(!h.needs_replacement());
    }

    #[test]
    fn test_leak() {
        let mut h = HeaterCore::new();
        h.leak_detected = true;
        assert!(h.needs_replacement());
    }

    #[test]
    fn test_health() {
        let h = HeaterCore::new();
        assert!((h.health_score() - 100.0).abs() < 0.1);
    }
}
