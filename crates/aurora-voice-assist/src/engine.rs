/// voice assist: wake, listen, parse, respond, learn
/// Phase 1180

#[derive(Debug, Clone)]
pub struct VoiceAssist {
    pub wake_ok: bool,
    pub listen_ok: bool,
    pub parse_ok: bool,
    pub respond_ok: bool,
    pub learn_ok: bool,
}

impl Default for VoiceAssist {
    fn default() -> Self {
        Self::new()
    }
}

impl VoiceAssist {
    pub fn new() -> Self {
        Self {
            wake_ok: true,
            listen_ok: true,
            parse_ok: true,
            respond_ok: true,
            learn_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.wake_ok && self.listen_ok && self.parse_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.respond_ok && self.learn_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.wake_ok || !self.listen_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.wake_ok {
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
        let c = VoiceAssist::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = VoiceAssist::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VoiceAssist::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = VoiceAssist::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = VoiceAssist::new();
        c.wake_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = VoiceAssist::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
