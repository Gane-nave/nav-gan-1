/// ux feedback: haptic, audio, visual, confirm, log
/// Phase 1506

#[derive(Debug, Clone)]
pub struct UxFeedback {
    pub haptic_ok: bool,
    pub audio_ok: bool,
    pub visual_ok: bool,
    pub confirm_ok: bool,
    pub log_ok: bool,
}

impl Default for UxFeedback {
    fn default() -> Self {
        Self::new()
    }
}

impl UxFeedback {
    pub fn new() -> Self {
        Self {
            haptic_ok: true,
            audio_ok: true,
            visual_ok: true,
            confirm_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.haptic_ok && self.audio_ok && self.visual_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.confirm_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.haptic_ok || !self.audio_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.haptic_ok {
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
        let c = UxFeedback::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UxFeedback::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UxFeedback::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UxFeedback::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UxFeedback::new();
        c.haptic_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UxFeedback::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
