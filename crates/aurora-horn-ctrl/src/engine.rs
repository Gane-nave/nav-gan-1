/// horn ctrl: short, long, pattern, volume, mute
/// Phase 1191

#[derive(Debug, Clone)]
pub struct HornCtrl {
    pub short_ok: bool,
    pub long_ok: bool,
    pub pattern_ok: bool,
    pub volume_ok: bool,
    pub mute_ok: bool,
}

impl Default for HornCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl HornCtrl {
    pub fn new() -> Self {
        Self {
            short_ok: true,
            long_ok: true,
            pattern_ok: true,
            volume_ok: true,
            mute_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.short_ok && self.long_ok && self.pattern_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.volume_ok && self.mute_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.short_ok || !self.long_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.short_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = HornCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = HornCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HornCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = HornCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = HornCtrl::new();
        c.short_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = HornCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
