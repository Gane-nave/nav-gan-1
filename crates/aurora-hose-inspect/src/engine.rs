/// Hose inspection: bulge, crack, clamp, routing
/// Phase 830

#[derive(Debug, Clone)]
pub struct HoseInspect {
    pub bulge_free: bool,
    pub crack_free: bool,
    pub clamp_ok: bool,
    pub routing_ok: bool,
    pub flexibility_ok: bool,
}

impl Default for HoseInspect {
    fn default() -> Self {
        Self::new()
    }
}

impl HoseInspect {
    pub fn new() -> Self {
        Self {
            bulge_free: true,
            crack_free: true,
            clamp_ok: true,
            routing_ok: true,
            flexibility_ok: true,
        }
    }

    pub fn condition_ok(&self) -> bool {
        self.bulge_free && self.crack_free && self.flexibility_ok
    }

    pub fn installation_ok(&self) -> bool {
        self.clamp_ok && self.routing_ok
    }

    pub fn all_ok(&self) -> bool {
        self.condition_ok() && self.installation_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.bulge_free || !self.crack_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.bulge_free { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition() {
        let c = HoseInspect::new();
        assert!(c.condition_ok());
    }

    #[test]
    fn test_installation() {
        let c = HoseInspect::new();
        assert!(c.installation_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HoseInspect::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = HoseInspect::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_bulge() {
        let mut c = HoseInspect::new();
        c.bulge_free = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = HoseInspect::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
