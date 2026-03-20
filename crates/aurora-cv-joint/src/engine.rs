/// CV joint: grease, boot, cage, balls
/// Phase 650

#[derive(Debug, Clone)]
pub struct CvJoint {
    pub grease_ok: bool,
    pub boot_ok: bool,
    pub cage_ok: bool,
    pub balls_ok: bool,
    pub noise_free: bool,
}

impl Default for CvJoint {
    fn default() -> Self {
        Self::new()
    }
}

impl CvJoint {
    pub fn new() -> Self {
        Self {
            grease_ok: true,
            boot_ok: true,
            cage_ok: true,
            balls_ok: true,
            noise_free: true,
        }
    }

    pub fn lubrication_ok(&self) -> bool {
        self.grease_ok && self.boot_ok
    }

    pub fn internals_ok(&self) -> bool {
        self.cage_ok && self.balls_ok
    }

    pub fn all_ok(&self) -> bool {
        self.lubrication_ok() && self.internals_ok() && self.noise_free
    }

    pub fn needs_replacement(&self) -> bool {
        !self.boot_ok || !self.cage_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.cage_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lubrication() {
        let c = CvJoint::new();
        assert!(c.lubrication_ok());
    }

    #[test]
    fn test_internals() {
        let c = CvJoint::new();
        assert!(c.internals_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CvJoint::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = CvJoint::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_boot() {
        let mut c = CvJoint::new();
        c.boot_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = CvJoint::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
