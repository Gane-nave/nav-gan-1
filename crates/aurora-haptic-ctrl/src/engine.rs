/// haptic ctrl: vibrate, pattern, intensity, zone, sync
/// Phase 1313

#[derive(Debug, Clone)]
pub struct HapticCtrl {
    pub vibrate_ok: bool,
    pub pattern_ok: bool,
    pub intensity_ok: bool,
    pub zone_ok: bool,
    pub sync_ok: bool,
}

impl Default for HapticCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl HapticCtrl {
    pub fn new() -> Self {
        Self {
            vibrate_ok: true,
            pattern_ok: true,
            intensity_ok: true,
            zone_ok: true,
            sync_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.vibrate_ok && self.pattern_ok && self.intensity_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.zone_ok && self.sync_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.vibrate_ok || !self.pattern_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.vibrate_ok {
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
        let c = HapticCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = HapticCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = HapticCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = HapticCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = HapticCtrl::new();
        c.vibrate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = HapticCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
