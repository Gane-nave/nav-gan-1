/// AC evaporator: coil, expansion valve, drain
/// Phase 624

#[derive(Debug, Clone)]
pub struct Evaporator {
    pub coil_ok: bool,
    pub expansion_ok: bool,
    pub drain_ok: bool,
    pub temp_ok: bool,
    pub odor_free: bool,
}

impl Default for Evaporator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaporator {
    pub fn new() -> Self {
        Self {
            coil_ok: true,
            expansion_ok: true,
            drain_ok: true,
            temp_ok: true,
            odor_free: true,
        }
    }

    pub fn cooling_ok(&self) -> bool {
        self.coil_ok && self.expansion_ok && self.temp_ok
    }

    pub fn drainage_ok(&self) -> bool {
        self.drain_ok && self.odor_free
    }

    pub fn all_ok(&self) -> bool {
        self.cooling_ok() && self.drainage_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.coil_ok || !self.drain_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.coil_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cooling() {
        let c = Evaporator::new();
        assert!(c.cooling_ok());
    }

    #[test]
    fn test_drainage() {
        let c = Evaporator::new();
        assert!(c.drainage_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Evaporator::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = Evaporator::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_coil() {
        let mut c = Evaporator::new();
        c.coil_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = Evaporator::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
