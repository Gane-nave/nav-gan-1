/// Air spring: bellows, valve, height sensor, compressor
/// Phase 645

#[derive(Debug, Clone)]
pub struct AirSpring {
    pub bellows_ok: bool,
    pub valve_ok: bool,
    pub height_ok: bool,
    pub compressor_ok: bool,
    pub leak_free: bool,
}

impl Default for AirSpring {
    fn default() -> Self {
        Self::new()
    }
}

impl AirSpring {
    pub fn new() -> Self {
        Self {
            bellows_ok: true,
            valve_ok: true,
            height_ok: true,
            compressor_ok: true,
            leak_free: true,
        }
    }

    pub fn spring_ok(&self) -> bool {
        self.bellows_ok && self.leak_free
    }

    pub fn system_ok(&self) -> bool {
        self.valve_ok && self.height_ok && self.compressor_ok
    }

    pub fn all_ok(&self) -> bool {
        self.spring_ok() && self.system_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.bellows_ok || !self.leak_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.bellows_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spring() {
        let c = AirSpring::new();
        assert!(c.spring_ok());
    }

    #[test]
    fn test_system() {
        let c = AirSpring::new();
        assert!(c.system_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AirSpring::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = AirSpring::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_bellows() {
        let mut c = AirSpring::new();
        c.bellows_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = AirSpring::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
