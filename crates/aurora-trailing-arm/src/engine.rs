/// Trailing arm: bushing, pivot, mount, alignment
/// Phase 652

#[derive(Debug, Clone)]
pub struct TrailingArm {
    pub bushing_ok: bool,
    pub pivot_ok: bool,
    pub mount_ok: bool,
    pub aligned: bool,
    pub cracked: bool,
}

impl Default for TrailingArm {
    fn default() -> Self {
        Self::new()
    }
}

impl TrailingArm {
    pub fn new() -> Self {
        Self {
            bushing_ok: true,
            pivot_ok: true,
            mount_ok: true,
            aligned: true,
            cracked: false,
        }
    }

    pub fn joints_ok(&self) -> bool {
        self.bushing_ok && self.pivot_ok
    }

    pub fn structure_ok(&self) -> bool {
        self.mount_ok && !self.cracked
    }

    pub fn all_ok(&self) -> bool {
        self.joints_ok() && self.structure_ok() && self.aligned
    }

    pub fn needs_replacement(&self) -> bool {
        self.cracked || !self.bushing_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.cracked {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joints() {
        let c = TrailingArm::new();
        assert!(c.joints_ok());
    }

    #[test]
    fn test_structure() {
        let c = TrailingArm::new();
        assert!(c.structure_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TrailingArm::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = TrailingArm::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_crack() {
        let mut c = TrailingArm::new();
        c.cracked = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = TrailingArm::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
