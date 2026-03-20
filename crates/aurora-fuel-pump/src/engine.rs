/// Fuel pump: pressure, flow rate, relay
/// Phase 499

#[derive(Debug, Clone)]
pub struct FuelPump {
    pub pressure_bar: f64,
    pub target_pressure_bar: f64,
    pub flow_lph: f64,
    pub relay_ok: bool,
    pub filter_ok: bool,
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
            flow_lph: 120.0,
            relay_ok: true,
            filter_ok: true,
        }
    }

    pub fn pressure_ok(&self) -> bool {
        (self.pressure_bar - self.target_pressure_bar).abs() < 0.5
    }

    pub fn flow_ok(&self) -> bool {
        self.flow_lph > 80.0
    }

    pub fn all_ok(&self) -> bool {
        self.pressure_ok() && self.flow_ok() && self.relay_ok && self.filter_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.filter_ok || !self.relay_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.relay_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure() {
        let c = FuelPump::new();
        assert!(c.pressure_ok());
    }

    #[test]
    fn test_flow() {
        let c = FuelPump::new();
        assert!(c.flow_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FuelPump::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = FuelPump::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_relay_fail() {
        let mut c = FuelPump::new();
        c.relay_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = FuelPump::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
