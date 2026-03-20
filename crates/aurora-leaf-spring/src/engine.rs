/// Leaf spring: arch, clamp, shackle, bushing
/// Phase 644

#[derive(Debug, Clone)]
pub struct LeafSpring {
    pub arch_ok: bool,
    pub clamp_ok: bool,
    pub shackle_ok: bool,
    pub bushing_ok: bool,
    pub cracked: bool,
}

impl Default for LeafSpring {
    fn default() -> Self {
        Self::new()
    }
}

impl LeafSpring {
    pub fn new() -> Self {
        Self {
            arch_ok: true,
            clamp_ok: true,
            shackle_ok: true,
            bushing_ok: true,
            cracked: false,
        }
    }

    pub fn shape_ok(&self) -> bool {
        self.arch_ok && !self.cracked
    }

    pub fn hardware_ok(&self) -> bool {
        self.clamp_ok && self.shackle_ok && self.bushing_ok
    }

    pub fn all_ok(&self) -> bool {
        self.shape_ok() && self.hardware_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        self.cracked || !self.arch_ok
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
    fn test_shape() {
        let c = LeafSpring::new();
        assert!(c.shape_ok());
    }

    #[test]
    fn test_hardware() {
        let c = LeafSpring::new();
        assert!(c.hardware_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LeafSpring::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = LeafSpring::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_crack() {
        let mut c = LeafSpring::new();
        c.cracked = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = LeafSpring::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
