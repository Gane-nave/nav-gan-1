/// glove box: lock, light, cool, damper, check
/// Phase 1280

#[derive(Debug, Clone)]
pub struct GloveBox {
    pub lock_ok: bool,
    pub light_ok: bool,
    pub cool_ok: bool,
    pub damper_ok: bool,
    pub check_ok: bool,
}

impl Default for GloveBox {
    fn default() -> Self {
        Self::new()
    }
}

impl GloveBox {
    pub fn new() -> Self {
        Self {
            lock_ok: true,
            light_ok: true,
            cool_ok: true,
            damper_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.lock_ok && self.light_ok && self.cool_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.damper_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.lock_ok || !self.light_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.lock_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = GloveBox::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GloveBox::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GloveBox::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GloveBox::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GloveBox::new();
        c.lock_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GloveBox::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
