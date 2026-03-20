/// Glove box: latch, damper, light, lock
/// Phase 755

#[derive(Debug, Clone)]
pub struct GloveBox {
    pub latch_ok: bool,
    pub damper_ok: bool,
    pub light_ok: bool,
    pub lock_ok: bool,
    pub hinge_ok: bool,
}

impl Default for GloveBox {
    fn default() -> Self {
        Self::new()
    }
}

impl GloveBox {
    pub fn new() -> Self {
        Self {
            latch_ok: true,
            damper_ok: true,
            light_ok: true,
            lock_ok: true,
            hinge_ok: true,
        }
    }

    pub fn mechanism_ok(&self) -> bool {
        self.latch_ok && self.damper_ok && self.hinge_ok
    }

    pub fn features_ok(&self) -> bool {
        self.light_ok && self.lock_ok
    }

    pub fn all_ok(&self) -> bool {
        self.mechanism_ok() && self.features_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.latch_ok || !self.hinge_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.latch_ok { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mechanism() {
        let c = GloveBox::new();
        assert!(c.mechanism_ok());
    }

    #[test]
    fn test_features() {
        let c = GloveBox::new();
        assert!(c.features_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GloveBox::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = GloveBox::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_latch() {
        let mut c = GloveBox::new();
        c.latch_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = GloveBox::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
