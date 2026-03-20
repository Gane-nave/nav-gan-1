/// Roll cage: tube, joint, padding, harness, mount
/// Phase 956

#[derive(Debug, Clone)]
pub struct RollCage {
    pub tube_ok: bool,
    pub joint_ok: bool,
    pub padding_ok: bool,
    pub harness_ok: bool,
    pub mount_ok: bool,
}

impl Default for RollCage {
    fn default() -> Self {
        Self::new()
    }
}

impl RollCage {
    pub fn new() -> Self {
        Self {
            tube_ok: true,
            joint_ok: true,
            padding_ok: true,
            harness_ok: true,
            mount_ok: true,
        }
    }

    pub fn structure_ok(&self) -> bool {
        self.tube_ok && self.joint_ok && self.mount_ok
    }

    pub fn safety_ok(&self) -> bool {
        self.padding_ok && self.harness_ok
    }

    pub fn all_ok(&self) -> bool {
        self.structure_ok() && self.safety_ok()
    }

    pub fn needs_inspect(&self) -> bool {
        !self.tube_ok || !self.joint_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.tube_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure() {
        let c = RollCage::new();
        assert!(c.structure_ok());
    }

    #[test]
    fn test_safety() {
        let c = RollCage::new();
        assert!(c.safety_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RollCage::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_inspect() {
        let c = RollCage::new();
        assert!(!c.needs_inspect());
    }

    #[test]
    fn test_tube() {
        let mut c = RollCage::new();
        c.tube_ok = false;
        assert!(c.needs_inspect());
    }

    #[test]
    fn test_health() {
        let c = RollCage::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
