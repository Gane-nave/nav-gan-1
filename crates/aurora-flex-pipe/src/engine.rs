/// Flex pipe: bellows, mesh, inner liner, clamps
/// Phase 616

#[derive(Debug, Clone)]
pub struct FlexPipe {
    pub bellows_ok: bool,
    pub mesh_ok: bool,
    pub liner_ok: bool,
    pub clamps_ok: bool,
    pub leak_free: bool,
}

impl Default for FlexPipe {
    fn default() -> Self {
        Self::new()
    }
}

impl FlexPipe {
    pub fn new() -> Self {
        Self {
            bellows_ok: true,
            mesh_ok: true,
            liner_ok: true,
            clamps_ok: true,
            leak_free: true,
        }
    }

    pub fn flexibility_ok(&self) -> bool {
        self.bellows_ok && self.mesh_ok
    }

    pub fn sealing_ok(&self) -> bool {
        self.liner_ok && self.leak_free && self.clamps_ok
    }

    pub fn all_ok(&self) -> bool {
        self.flexibility_ok() && self.sealing_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.bellows_ok || !self.leak_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.bellows_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flexibility() {
        let c = FlexPipe::new();
        assert!(c.flexibility_ok());
    }

    #[test]
    fn test_sealing() {
        let c = FlexPipe::new();
        assert!(c.sealing_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FlexPipe::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = FlexPipe::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_bellows() {
        let mut c = FlexPipe::new();
        c.bellows_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = FlexPipe::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
