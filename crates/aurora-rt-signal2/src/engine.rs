/// rt signal2: register, handle, mask, restore, log
/// Phase 2337

#[derive(Debug, Clone)]
pub struct RtSignal2 {
    pub register_ok: bool,
    pub handle_ok: bool,
    pub mask_ok: bool,
    pub restore_ok: bool,
    pub log_ok: bool,
}

impl Default for RtSignal2 {
    fn default() -> Self {
        Self::new()
    }
}

impl RtSignal2 {
    pub fn new() -> Self {
        Self {
            register_ok: true,
            handle_ok: true,
            mask_ok: true,
            restore_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.register_ok && self.handle_ok && self.mask_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.restore_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.register_ok || !self.handle_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.register_ok {
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
        let c = RtSignal2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RtSignal2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RtSignal2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RtSignal2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RtSignal2::new();
        c.register_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RtSignal2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
