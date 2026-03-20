/// sound ctrl: volume, balance, fade, equalizer, source
/// Phase 1314

#[derive(Debug, Clone)]
pub struct SoundCtrl {
    pub volume_ok: bool,
    pub balance_ok: bool,
    pub fade_ok: bool,
    pub equalizer_ok: bool,
    pub source_ok: bool,
}

impl Default for SoundCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl SoundCtrl {
    pub fn new() -> Self {
        Self {
            volume_ok: true,
            balance_ok: true,
            fade_ok: true,
            equalizer_ok: true,
            source_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.volume_ok && self.balance_ok && self.fade_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.equalizer_ok && self.source_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.volume_ok || !self.balance_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.volume_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = SoundCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SoundCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SoundCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SoundCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SoundCtrl::new();
        c.volume_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SoundCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
