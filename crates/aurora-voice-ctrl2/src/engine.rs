/// voice ctrl: listen, parse, execute, confirm, learn
/// Phase 1312

#[derive(Debug, Clone)]
pub struct VoiceCtrl2 {
    pub listen_ok: bool,
    pub parse_ok: bool,
    pub execute_ok: bool,
    pub confirm_ok: bool,
    pub learn_ok: bool,
}

impl Default for VoiceCtrl2 {
    fn default() -> Self {
        Self::new()
    }
}

impl VoiceCtrl2 {
    pub fn new() -> Self {
        Self {
            listen_ok: true,
            parse_ok: true,
            execute_ok: true,
            confirm_ok: true,
            learn_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.listen_ok && self.parse_ok && self.execute_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.confirm_ok && self.learn_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.listen_ok || !self.parse_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.listen_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = VoiceCtrl2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = VoiceCtrl2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = VoiceCtrl2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = VoiceCtrl2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = VoiceCtrl2::new();
        c.listen_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = VoiceCtrl2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
