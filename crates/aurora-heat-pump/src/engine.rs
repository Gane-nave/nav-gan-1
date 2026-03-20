/// Heat pump: compressor, valve, refrigerant, COP
/// Phase 875

#[derive(Debug, Clone)]
pub struct HeatPump {
    pub compressor_ok: bool,
    pub valve_ok: bool,
    pub refrigerant_ok: bool,
    pub cop_ok: bool,
    pub defrost_ok: bool,
}

impl Default for HeatPump {
    fn default() -> Self {
        Self::new()
    }
}

impl HeatPump {
    pub fn new() -> Self {
        Self {
            compressor_ok: true,
            valve_ok: true,
            refrigerant_ok: true,
            cop_ok: true,
            defrost_ok: true,
        }
    }

    pub fn heating_ok(&self) -> bool {
        self.compressor_ok && self.valve_ok && self.refrigerant_ok
    }

    pub fn efficiency_ok(&self) -> bool {
        self.cop_ok && self.defrost_ok
    }

    pub fn all_ok(&self) -> bool {
        self.heating_ok() && self.efficiency_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.compressor_ok || !self.refrigerant_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.compressor_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heating() {
        let c = HeatPump::new();
        assert!(c.heating_ok());
    }

    #[test]
    fn test_efficiency() {
        let c = HeatPump::new();
        assert!(c.efficiency_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HeatPump::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = HeatPump::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_compressor() {
        let mut c = HeatPump::new();
        c.compressor_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = HeatPump::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
