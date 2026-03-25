/// Oil pan: drain plug, gasket, baffle, level sensor
/// Phase 578

#[derive(Debug, Clone)]
pub struct OilPan {
    pub drain_plug_ok: bool,
    pub gasket_ok: bool,
    pub baffle_ok: bool,
    pub sensor_ok: bool,
    pub leak_free: bool,
}

impl Default for OilPan {
    fn default() -> Self {
        Self::new()
    }
}

impl OilPan {
    pub fn new() -> Self {
        Self {
            drain_plug_ok: true,
            gasket_ok: true,
            baffle_ok: true,
            sensor_ok: true,
            leak_free: true,
        }
    }

    pub fn sealing_ok(&self) -> bool {
        self.drain_plug_ok && self.gasket_ok && self.leak_free
    }

    pub fn internals_ok(&self) -> bool {
        self.baffle_ok && self.sensor_ok
    }

    pub fn all_ok(&self) -> bool {
        self.sealing_ok() && self.internals_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.gasket_ok || !self.leak_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.leak_free {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sealing() {
        let c = OilPan::new();
        assert!(c.sealing_ok());
    }

    #[test]
    fn test_internals() {
        let c = OilPan::new();
        assert!(c.internals_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OilPan::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = OilPan::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_leak() {
        let mut c = OilPan::new();
        c.leak_free = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = OilPan::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
