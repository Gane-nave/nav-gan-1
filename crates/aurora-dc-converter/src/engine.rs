/// DC-DC converter: buck, boost, isolation, regulation
/// Phase 718

#[derive(Debug, Clone)]
pub struct DcConverter {
    pub buck_ok: bool,
    pub boost_ok: bool,
    pub isolation_ok: bool,
    pub regulation_ok: bool,
    pub efficiency_ok: bool,
}

impl Default for DcConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl DcConverter {
    pub fn new() -> Self {
        Self {
            buck_ok: true,
            boost_ok: true,
            isolation_ok: true,
            regulation_ok: true,
            efficiency_ok: true,
        }
    }

    pub fn conversion_ok(&self) -> bool {
        self.buck_ok && self.boost_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.isolation_ok && self.regulation_ok && self.efficiency_ok
    }

    pub fn all_ok(&self) -> bool {
        self.conversion_ok() && self.safety_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.buck_ok || !self.isolation_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.isolation_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        let c = DcConverter::new();
        assert!(c.conversion_ok());
    }

    #[test]
    fn test_safety() {
        let c = DcConverter::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DcConverter::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = DcConverter::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_buck() {
        let mut c = DcConverter::new();
        c.buck_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = DcConverter::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
