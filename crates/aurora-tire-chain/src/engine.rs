/// Tire chain: auto-tensioner, link, cross-chain, cam
/// Phase 845

#[derive(Debug, Clone)]
pub struct TireChain {
    pub tensioner_ok: bool,
    pub link_ok: bool,
    pub cross_ok: bool,
    pub cam_ok: bool,
    pub fit_ok: bool,
}

impl Default for TireChain {
    fn default() -> Self {
        Self::new()
    }
}

impl TireChain {
    pub fn new() -> Self {
        Self {
            tensioner_ok: true,
            link_ok: true,
            cross_ok: true,
            cam_ok: true,
            fit_ok: true,
        }
    }

    pub fn traction_ok(&self) -> bool {
        self.link_ok && self.cross_ok && self.fit_ok
    }

    pub fn mechanism_ok(&self) -> bool {
        self.tensioner_ok && self.cam_ok
    }

    pub fn all_ok(&self) -> bool {
        self.traction_ok() && self.mechanism_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.link_ok || !self.tensioner_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.link_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traction() {
        let c = TireChain::new();
        assert!(c.traction_ok());
    }

    #[test]
    fn test_mechanism() {
        let c = TireChain::new();
        assert!(c.mechanism_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TireChain::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = TireChain::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_link() {
        let mut c = TireChain::new();
        c.link_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = TireChain::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
