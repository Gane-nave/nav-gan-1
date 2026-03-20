/// nvh sim: excite, transfer, damp, optimize, log
/// Phase 1406

#[derive(Debug, Clone)]
pub struct NvhSim {
    pub excite_ok: bool,
    pub transfer_ok: bool,
    pub damp_ok: bool,
    pub optimize_ok: bool,
    pub log_ok: bool,
}

impl Default for NvhSim {
    fn default() -> Self {
        Self::new()
    }
}

impl NvhSim {
    pub fn new() -> Self {
        Self {
            excite_ok: true,
            transfer_ok: true,
            damp_ok: true,
            optimize_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.excite_ok && self.transfer_ok && self.damp_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.optimize_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.excite_ok || !self.transfer_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.excite_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = NvhSim::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NvhSim::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NvhSim::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NvhSim::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NvhSim::new();
        c.excite_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NvhSim::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
