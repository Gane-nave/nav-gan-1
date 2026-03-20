/// ux tooltip: detect, position, show, dismiss, log
/// Phase 1515

#[derive(Debug, Clone)]
pub struct UxTooltip {
    pub detect_ok: bool,
    pub position_ok: bool,
    pub show_ok: bool,
    pub dismiss_ok: bool,
    pub log_ok: bool,
}

impl Default for UxTooltip {
    fn default() -> Self {
        Self::new()
    }
}

impl UxTooltip {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            position_ok: true,
            show_ok: true,
            dismiss_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.position_ok && self.show_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.dismiss_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.position_ok
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
        let c = UxTooltip::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UxTooltip::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UxTooltip::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UxTooltip::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UxTooltip::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UxTooltip::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
