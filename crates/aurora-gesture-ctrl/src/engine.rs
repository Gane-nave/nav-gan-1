/// gesture ctrl: detect, classify, interpret, execute, feedback
/// Phase 1182

#[derive(Debug, Clone)]
pub struct GestureCtrl {
    pub detect_ok: bool,
    pub classify_ok: bool,
    pub interpret_ok: bool,
    pub execute_ok: bool,
    pub feedback_ok: bool,
}

impl Default for GestureCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl GestureCtrl {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            classify_ok: true,
            interpret_ok: true,
            execute_ok: true,
            feedback_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.classify_ok && self.interpret_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.execute_ok && self.feedback_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.classify_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = GestureCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GestureCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GestureCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GestureCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GestureCtrl::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GestureCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
