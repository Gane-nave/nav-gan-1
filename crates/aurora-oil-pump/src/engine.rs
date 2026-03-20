/// Oil pump: gears, pressure relief, pickup tube
/// Phase 619

#[derive(Debug, Clone)]
pub struct OilPump {
    pub gears_ok: bool,
    pub relief_ok: bool,
    pub pickup_ok: bool,
    pub pressure_ok: bool,
    pub seal_ok: bool,
}

impl Default for OilPump {
    fn default() -> Self {
        Self::new()
    }
}

impl OilPump {
    pub fn new() -> Self {
        Self {
            gears_ok: true,
            relief_ok: true,
            pickup_ok: true,
            pressure_ok: true,
            seal_ok: true,
        }
    }

    pub fn mechanical_ok(&self) -> bool {
        self.gears_ok && self.pickup_ok
    }

    pub fn pressure_good(&self) -> bool {
        self.pressure_ok && self.relief_ok
    }

    pub fn all_ok(&self) -> bool {
        self.mechanical_ok() && self.pressure_good() && self.seal_ok
    }

    pub fn needs_replacement(&self) -> bool {
        !self.gears_ok || !self.pickup_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.gears_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mechanical() {
        let c = OilPump::new();
        assert!(c.mechanical_ok());
    }

    #[test]
    fn test_pressure() {
        let c = OilPump::new();
        assert!(c.pressure_good());
    }

    #[test]
    fn test_all_ok() {
        let c = OilPump::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = OilPump::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_gears() {
        let mut c = OilPump::new();
        c.gears_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = OilPump::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
