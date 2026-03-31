/// aurora-sim-gnss: sim gnss
/// Phase 2522

#[derive(Debug, Clone)]
pub struct SimGnss {
    pub generate_ok: bool,
    pub fix_ok: bool,
    pub noise_ok: bool,
    pub drift_ok: bool,
    pub multipath_ok: bool,
}

impl Default for SimGnss {
    fn default() -> Self {
        Self::new()
    }
}

impl SimGnss {
    pub fn new() -> Self {
        Self {
            generate_ok: true,
            fix_ok: true,
            noise_ok: true,
            drift_ok: true,
            multipath_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.generate_ok && self.fix_ok && self.noise_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.drift_ok && self.multipath_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.generate_ok || !self.fix_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.generate_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = SimGnss::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SimGnss::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SimGnss::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SimGnss::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SimGnss::new();
        c.generate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SimGnss::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = SimGnss::default();
        assert!(c.all_ok());
    }
}
