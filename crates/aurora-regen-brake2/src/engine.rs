/// regen brake: capture, convert, store, blend, log
/// Phase 1365

#[derive(Debug, Clone)]
pub struct RegenBrake2 {
    pub capture_ok: bool,
    pub convert_ok: bool,
    pub store_ok: bool,
    pub blend_ok: bool,
    pub log_ok: bool,
}

impl Default for RegenBrake2 {
    fn default() -> Self {
        Self::new()
    }
}

impl RegenBrake2 {
    pub fn new() -> Self {
        Self {
            capture_ok: true,
            convert_ok: true,
            store_ok: true,
            blend_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.capture_ok && self.convert_ok && self.store_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.blend_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.capture_ok || !self.convert_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.capture_ok {
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
        let c = RegenBrake2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RegenBrake2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RegenBrake2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RegenBrake2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RegenBrake2::new();
        c.capture_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RegenBrake2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
