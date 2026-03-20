/// Shock absorber: damping, oil, seal, mount
/// Phase 646

#[derive(Debug, Clone)]
pub struct ShockAbsorber {
    pub damping_ok: bool,
    pub oil_ok: bool,
    pub seal_ok: bool,
    pub mount_ok: bool,
    pub leak_free: bool,
}

impl Default for ShockAbsorber {
    fn default() -> Self {
        Self::new()
    }
}

impl ShockAbsorber {
    pub fn new() -> Self {
        Self {
            damping_ok: true,
            oil_ok: true,
            seal_ok: true,
            mount_ok: true,
            leak_free: true,
        }
    }

    pub fn damper_ok(&self) -> bool {
        self.damping_ok && self.oil_ok
    }

    pub fn physical_ok(&self) -> bool {
        self.seal_ok && self.mount_ok && self.leak_free
    }

    pub fn all_ok(&self) -> bool {
        self.damper_ok() && self.physical_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.damping_ok || !self.leak_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.damping_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damper() {
        let c = ShockAbsorber::new();
        assert!(c.damper_ok());
    }

    #[test]
    fn test_physical() {
        let c = ShockAbsorber::new();
        assert!(c.physical_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ShockAbsorber::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = ShockAbsorber::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_damping() {
        let mut c = ShockAbsorber::new();
        c.damping_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = ShockAbsorber::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
