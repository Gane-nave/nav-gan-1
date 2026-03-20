/// Hood latch: primary latch, safety catch, cable release
/// Phase 414

#[derive(Debug, Clone)]
pub struct HoodLatch {
    pub primary_engaged: bool,
    pub safety_catch: bool,
    pub cable_ok: bool,
    pub spring_ok: bool,
    pub lubricated: bool,
}

impl Default for HoodLatch {
    fn default() -> Self {
        Self::new()
    }
}

impl HoodLatch {
    pub fn new() -> Self {
        Self {
            primary_engaged: true,
            safety_catch: true,
            cable_ok: true,
            spring_ok: true,
            lubricated: true,
        }
    }

    pub fn secure(&self) -> bool {
        self.primary_engaged && self.safety_catch
    }

    pub fn all_ok(&self) -> bool {
        self.secure() && self.cable_ok && self.spring_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.cable_ok || !self.spring_ok || !self.lubricated
    }

    pub fn safety_ok(&self) -> bool {
        self.safety_catch
    }

    pub fn health_score(&self) -> f64 {
        if !self.safety_catch {
            return 0.0;
        }
        if !self.cable_ok {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure() {
        let h = HoodLatch::new();
        assert!(h.secure());
    }

    #[test]
    fn test_all_ok() {
        let h = HoodLatch::new();
        assert!(h.all_ok());
    }

    #[test]
    fn test_no_service() {
        let h = HoodLatch::new();
        assert!(!h.needs_service());
    }

    #[test]
    fn test_safety() {
        let h = HoodLatch::new();
        assert!(h.safety_ok());
    }

    #[test]
    fn test_bad_cable() {
        let mut h = HoodLatch::new();
        h.cable_ok = false;
        assert!(h.needs_service());
    }

    #[test]
    fn test_health() {
        let h = HoodLatch::new();
        assert!((h.health_score() - 100.0).abs() < 0.1);
    }
}
