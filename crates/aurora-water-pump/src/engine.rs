/// Water pump: impeller, bearing, seal, flow rate
/// Phase 511

#[derive(Debug, Clone)]
pub struct WaterPump {
    pub flow_lpm: f64,
    pub min_flow_lpm: f64,
    pub bearing_ok: bool,
    pub seal_ok: bool,
    pub impeller_ok: bool,
}

impl Default for WaterPump {
    fn default() -> Self {
        Self::new()
    }
}

impl WaterPump {
    pub fn new() -> Self {
        Self {
            flow_lpm: 80.0,
            min_flow_lpm: 40.0,
            bearing_ok: true,
            seal_ok: true,
            impeller_ok: true,
        }
    }

    pub fn flow_ok(&self) -> bool {
        self.flow_lpm > self.min_flow_lpm
    }

    pub fn mechanical_ok(&self) -> bool {
        self.bearing_ok && self.seal_ok && self.impeller_ok
    }

    pub fn all_ok(&self) -> bool {
        self.flow_ok() && self.mechanical_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.bearing_ok || !self.seal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bearing_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow() {
        let c = WaterPump::new();
        assert!(c.flow_ok());
    }

    #[test]
    fn test_mechanical() {
        let c = WaterPump::new();
        assert!(c.mechanical_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WaterPump::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = WaterPump::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_bearing() {
        let mut c = WaterPump::new();
        c.bearing_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = WaterPump::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
