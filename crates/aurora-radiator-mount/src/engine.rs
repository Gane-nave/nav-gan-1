/// Radiator mount: upper bracket, lower cushion, isolator
/// Phase 803

#[derive(Debug, Clone)]
pub struct RadiatorMount {
    pub upper_ok: bool,
    pub lower_ok: bool,
    pub isolator_ok: bool,
    pub pin_ok: bool,
    pub clearance_ok: bool,
}

impl Default for RadiatorMount {
    fn default() -> Self {
        Self::new()
    }
}

impl RadiatorMount {
    pub fn new() -> Self {
        Self {
            upper_ok: true,
            lower_ok: true,
            isolator_ok: true,
            pin_ok: true,
            clearance_ok: true,
        }
    }

    pub fn support_ok(&self) -> bool {
        self.upper_ok && self.lower_ok && self.pin_ok
    }

    pub fn isolation_ok(&self) -> bool {
        self.isolator_ok && self.clearance_ok
    }

    pub fn all_ok(&self) -> bool {
        self.support_ok() && self.isolation_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.upper_ok || !self.lower_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.upper_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_support() {
        let c = RadiatorMount::new();
        assert!(c.support_ok());
    }

    #[test]
    fn test_isolation() {
        let c = RadiatorMount::new();
        assert!(c.isolation_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RadiatorMount::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = RadiatorMount::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_upper() {
        let mut c = RadiatorMount::new();
        c.upper_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = RadiatorMount::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
