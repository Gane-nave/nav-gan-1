/// Brake shoe: lining, spring, adjuster, anchor
/// Phase 657

#[derive(Debug, Clone)]
pub struct BrakeShoe {
    pub lining_ok: bool,
    pub spring_ok: bool,
    pub adjuster_ok: bool,
    pub anchor_ok: bool,
    pub thickness_ok: bool,
}

impl Default for BrakeShoe {
    fn default() -> Self {
        Self::new()
    }
}

impl BrakeShoe {
    pub fn new() -> Self {
        Self {
            lining_ok: true,
            spring_ok: true,
            adjuster_ok: true,
            anchor_ok: true,
            thickness_ok: true,
        }
    }

    pub fn friction_ok(&self) -> bool {
        self.lining_ok && self.thickness_ok
    }

    pub fn hardware_ok(&self) -> bool {
        self.spring_ok && self.adjuster_ok && self.anchor_ok
    }

    pub fn all_ok(&self) -> bool {
        self.friction_ok() && self.hardware_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.lining_ok || !self.thickness_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.lining_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_friction() {
        let c = BrakeShoe::new();
        assert!(c.friction_ok());
    }

    #[test]
    fn test_hardware() {
        let c = BrakeShoe::new();
        assert!(c.hardware_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BrakeShoe::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = BrakeShoe::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_lining() {
        let mut c = BrakeShoe::new();
        c.lining_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = BrakeShoe::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
