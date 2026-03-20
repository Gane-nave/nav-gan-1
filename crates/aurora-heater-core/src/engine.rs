/// Heater core: cabin heat, coolant flow, blend door
/// Phase 513

#[derive(Debug, Clone)]
pub struct HeaterCore {
    pub inlet_temp_c: f64,
    pub outlet_temp_c: f64,
    pub flow_ok: bool,
    pub leak_free: bool,
    pub blend_door_ok: bool,
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
            flow_ok: true,
            leak_free: true,
            blend_door_ok: true,
        }
    }

    pub fn heat_transfer(&self) -> f64 {
        self.inlet_temp_c - self.outlet_temp_c
    }

    pub fn heating_ok(&self) -> bool {
        self.heat_transfer() > 5.0 && self.flow_ok
    }

    pub fn all_ok(&self) -> bool {
        self.heating_ok() && self.leak_free && self.blend_door_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.leak_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.leak_free { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer() {
        let c = HeaterCore::new();
        assert!(c.heat_transfer() > 10.0);
    }

    #[test]
    fn test_heating() {
        let c = HeaterCore::new();
        assert!(c.heating_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HeaterCore::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = HeaterCore::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_leak() {
        let mut c = HeaterCore::new();
        c.leak_free = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = HeaterCore::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
