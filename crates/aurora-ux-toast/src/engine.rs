/// ux toast: create, show, queue, dismiss, log
/// Phase 1514

#[derive(Debug, Clone)]
pub struct UxToast {
    pub create_ok: bool,
    pub show_ok: bool,
    pub queue_ok: bool,
    pub dismiss_ok: bool,
    pub log_ok: bool,
}

impl Default for UxToast {
    fn default() -> Self {
        Self::new()
    }
}

impl UxToast {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            show_ok: true,
            queue_ok: true,
            dismiss_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.show_ok && self.queue_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.dismiss_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.show_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = UxToast::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UxToast::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UxToast::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UxToast::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UxToast::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UxToast::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
