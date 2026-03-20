/// Seat belt pretensioner: pyro, retractor, load limiter
/// Phase 836

#[derive(Debug, Clone)]
pub struct SeatBeltPre {
    pub pyro_ok: bool,
    pub retractor_ok: bool,
    pub limiter_ok: bool,
    pub buckle_ok: bool,
    pub webbing_ok: bool,
}

impl Default for SeatBeltPre {
    fn default() -> Self {
        Self::new()
    }
}

impl SeatBeltPre {
    pub fn new() -> Self {
        Self {
            pyro_ok: true,
            retractor_ok: true,
            limiter_ok: true,
            buckle_ok: true,
            webbing_ok: true,
        }
    }

    pub fn restraint_ok(&self) -> bool {
        self.pyro_ok && self.retractor_ok && self.limiter_ok
    }

    pub fn hardware_ok(&self) -> bool {
        self.buckle_ok && self.webbing_ok
    }

    pub fn all_ok(&self) -> bool {
        self.restraint_ok() && self.hardware_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.pyro_ok || !self.webbing_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.pyro_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restraint() {
        let c = SeatBeltPre::new();
        assert!(c.restraint_ok());
    }

    #[test]
    fn test_hardware() {
        let c = SeatBeltPre::new();
        assert!(c.hardware_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SeatBeltPre::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = SeatBeltPre::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_pyro() {
        let mut c = SeatBeltPre::new();
        c.pyro_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = SeatBeltPre::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
