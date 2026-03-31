/// aurora-assert-gnss: assert gnss
/// Phase 2506

#[derive(Debug, Clone)]
pub struct AssertGnss {
    pub satellite_ok: bool,
    pub snr_ok: bool,
    pub pdop_ok: bool,
    pub fix_ok: bool,
    pub constellation_ok: bool,
}

impl Default for AssertGnss {
    fn default() -> Self {
        Self::new()
    }
}

impl AssertGnss {
    pub fn new() -> Self {
        Self {
            satellite_ok: true,
            snr_ok: true,
            pdop_ok: true,
            fix_ok: true,
            constellation_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.satellite_ok && self.snr_ok && self.pdop_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.fix_ok && self.constellation_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.satellite_ok || !self.snr_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.satellite_ok {
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
        let c = AssertGnss::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AssertGnss::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AssertGnss::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AssertGnss::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AssertGnss::new();
        c.satellite_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AssertGnss::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = AssertGnss::default();
        assert!(c.all_ok());
    }
}
