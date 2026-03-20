/// Nitrous system: bottle, solenoid, jet, safety, purge
/// Phase 955

#[derive(Debug, Clone)]
pub struct NitrousSys {
    pub bottle_ok: bool,
    pub solenoid_ok: bool,
    pub jet_ok: bool,
    pub safety_ok: bool,
    pub purge_ok: bool,
}

impl Default for NitrousSys {
    fn default() -> Self {
        Self::new()
    }
}

impl NitrousSys {
    pub fn new() -> Self {
        Self {
            bottle_ok: true,
            solenoid_ok: true,
            jet_ok: true,
            safety_ok: true,
            purge_ok: true,
        }
    }

    pub fn delivery_ok(&self) -> bool {
        self.bottle_ok && self.solenoid_ok && self.jet_ok
    }

    pub fn protection_ok(&self) -> bool {
        self.safety_ok && self.purge_ok
    }

    pub fn all_ok(&self) -> bool {
        self.delivery_ok() && self.protection_ok()
    }

    pub fn needs_refill(&self) -> bool {
        !self.bottle_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bottle_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery() {
        let c = NitrousSys::new();
        assert!(c.delivery_ok());
    }

    #[test]
    fn test_protection() {
        let c = NitrousSys::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NitrousSys::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_refill() {
        let c = NitrousSys::new();
        assert!(!c.needs_refill());
    }

    #[test]
    fn test_bottle() {
        let mut c = NitrousSys::new();
        c.bottle_ok = false;
        assert!(c.needs_refill());
    }

    #[test]
    fn test_health() {
        let c = NitrousSys::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
