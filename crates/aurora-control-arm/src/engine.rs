/// control arm: pivot, guide, absorb, align, inspect
/// Phase 1199

#[derive(Debug, Clone)]
pub struct ControlArm {
    pub pivot_ok: bool,
    pub guide_ok: bool,
    pub absorb_ok: bool,
    pub align_ok: bool,
    pub inspect_ok: bool,
}

impl Default for ControlArm {
    fn default() -> Self {
        Self::new()
    }
}

impl ControlArm {
    pub fn new() -> Self {
        Self {
            pivot_ok: true,
            guide_ok: true,
            absorb_ok: true,
            align_ok: true,
            inspect_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.pivot_ok && self.guide_ok && self.absorb_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.align_ok && self.inspect_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.pivot_ok || !self.guide_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.pivot_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = ControlArm::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ControlArm::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ControlArm::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ControlArm::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ControlArm::new();
        c.pivot_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ControlArm::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
