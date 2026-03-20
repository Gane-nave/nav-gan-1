/// ux modal: create, show, interact, dismiss, log
/// Phase 1513

#[derive(Debug, Clone)]
pub struct UxModal {
    pub create_ok: bool,
    pub show_ok: bool,
    pub interact_ok: bool,
    pub dismiss_ok: bool,
    pub log_ok: bool,
}

impl Default for UxModal {
    fn default() -> Self {
        Self::new()
    }
}

impl UxModal {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            show_ok: true,
            interact_ok: true,
            dismiss_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.show_ok && self.interact_ok
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
        let c = UxModal::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UxModal::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UxModal::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UxModal::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UxModal::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UxModal::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
