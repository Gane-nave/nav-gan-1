/// aurora-ui-form: ui form
/// Phase 2401

#[derive(Debug, Clone)]
pub struct UiForm {
    pub validate_ok: bool,
    pub submit_ok: bool,
    pub reset_ok: bool,
    pub bind_ok: bool,
    pub track_ok: bool,
}

impl Default for UiForm {
    fn default() -> Self {
        Self::new()
    }
}

impl UiForm {
    pub fn new() -> Self {
        Self {
            validate_ok: true,
            submit_ok: true,
            reset_ok: true,
            bind_ok: true,
            track_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.validate_ok && self.submit_ok && self.reset_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.bind_ok && self.track_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.validate_ok || !self.submit_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.validate_ok {
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
        let c = UiForm::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UiForm::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UiForm::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UiForm::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UiForm::new();
        c.validate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UiForm::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = UiForm::default();
        assert!(c.all_ok());
    }
}
