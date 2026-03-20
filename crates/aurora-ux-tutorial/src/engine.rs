/// ux tutorial: detect, guide, explain, dismiss, log
/// Phase 1507

#[derive(Debug, Clone)]
pub struct UxTutorial {
    pub detect_ok: bool,
    pub guide_ok: bool,
    pub explain_ok: bool,
    pub dismiss_ok: bool,
    pub log_ok: bool,
}

impl Default for UxTutorial {
    fn default() -> Self {
        Self::new()
    }
}

impl UxTutorial {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            guide_ok: true,
            explain_ok: true,
            dismiss_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.guide_ok && self.explain_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.dismiss_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.guide_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok {
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
        let c = UxTutorial::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UxTutorial::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UxTutorial::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UxTutorial::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UxTutorial::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UxTutorial::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
