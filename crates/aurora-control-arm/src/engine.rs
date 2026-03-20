/// Control arm: bushing, pivot, ball joint mount
/// Phase 640

#[derive(Debug, Clone)]
pub struct ControlArm {
    pub bushing_ok: bool,
    pub pivot_ok: bool,
    pub mount_ok: bool,
    pub bent: bool,
    pub corrosion_free: bool,
}

impl Default for ControlArm {
    fn default() -> Self {
        Self::new()
    }
}

impl ControlArm {
    pub fn new() -> Self {
        Self {
            bushing_ok: true,
            pivot_ok: true,
            mount_ok: true,
            bent: false,
            corrosion_free: true,
        }
    }

    pub fn structural_ok(&self) -> bool {
        !self.bent && self.corrosion_free
    }

    pub fn joints_ok(&self) -> bool {
        self.bushing_ok && self.pivot_ok && self.mount_ok
    }

    pub fn all_ok(&self) -> bool {
        self.structural_ok() && self.joints_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        self.bent || !self.bushing_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.bent { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structural() {
        let c = ControlArm::new();
        assert!(c.structural_ok());
    }

    #[test]
    fn test_joints() {
        let c = ControlArm::new();
        assert!(c.joints_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ControlArm::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = ControlArm::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_bent() {
        let mut c = ControlArm::new();
        c.bent = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = ControlArm::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
