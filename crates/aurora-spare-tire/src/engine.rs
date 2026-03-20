/// Spare tire: carrier, winch, lock, jack
/// Phase 805

#[derive(Debug, Clone)]
pub struct SpareTire {
    pub carrier_ok: bool,
    pub winch_ok: bool,
    pub lock_ok: bool,
    pub jack_ok: bool,
    pub pressure_ok: bool,
}

impl Default for SpareTire {
    fn default() -> Self {
        Self::new()
    }
}

impl SpareTire {
    pub fn new() -> Self {
        Self {
            carrier_ok: true,
            winch_ok: true,
            lock_ok: true,
            jack_ok: true,
            pressure_ok: true,
        }
    }

    pub fn storage_ok(&self) -> bool {
        self.carrier_ok && self.winch_ok && self.lock_ok
    }

    pub fn readiness_ok(&self) -> bool {
        self.jack_ok && self.pressure_ok
    }

    pub fn all_ok(&self) -> bool {
        self.storage_ok() && self.readiness_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.pressure_ok || !self.winch_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.pressure_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage() {
        let c = SpareTire::new();
        assert!(c.storage_ok());
    }

    #[test]
    fn test_readiness() {
        let c = SpareTire::new();
        assert!(c.readiness_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SpareTire::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = SpareTire::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_pressure() {
        let mut c = SpareTire::new();
        c.pressure_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = SpareTire::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
