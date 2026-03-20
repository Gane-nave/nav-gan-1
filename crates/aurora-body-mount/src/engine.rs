/// Body mount: bushing, bolt, shim, isolator
/// Phase 802

#[derive(Debug, Clone)]
pub struct BodyMount {
    pub bushing_ok: bool,
    pub bolt_ok: bool,
    pub shim_ok: bool,
    pub isolator_ok: bool,
    pub torque_ok: bool,
}

impl Default for BodyMount {
    fn default() -> Self {
        Self::new()
    }
}

impl BodyMount {
    pub fn new() -> Self {
        Self {
            bushing_ok: true,
            bolt_ok: true,
            shim_ok: true,
            isolator_ok: true,
            torque_ok: true,
        }
    }

    pub fn cushioning_ok(&self) -> bool {
        self.bushing_ok && self.isolator_ok
    }

    pub fn fastening_ok(&self) -> bool {
        self.bolt_ok && self.shim_ok && self.torque_ok
    }

    pub fn all_ok(&self) -> bool {
        self.cushioning_ok() && self.fastening_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        !self.bushing_ok || !self.bolt_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bushing_ok {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cushioning() {
        let c = BodyMount::new();
        assert!(c.cushioning_ok());
    }

    #[test]
    fn test_fastening() {
        let c = BodyMount::new();
        assert!(c.fastening_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BodyMount::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = BodyMount::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_bushing() {
        let mut c = BodyMount::new();
        c.bushing_ok = false;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = BodyMount::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
