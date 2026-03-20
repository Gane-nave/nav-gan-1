/// rt fiber: create, resume, yield_fiber, destroy, log
/// Phase 2331

#[derive(Debug, Clone)]
pub struct RtFiber {
    pub create_ok: bool,
    pub resume_ok: bool,
    pub yield_fiber_ok: bool,
    pub destroy_ok: bool,
    pub log_ok: bool,
}

impl Default for RtFiber {
    fn default() -> Self {
        Self::new()
    }
}

impl RtFiber {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            resume_ok: true,
            yield_fiber_ok: true,
            destroy_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.resume_ok && self.yield_fiber_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.destroy_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.resume_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok {
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
        let c = RtFiber::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RtFiber::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RtFiber::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RtFiber::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RtFiber::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RtFiber::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
