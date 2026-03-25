/// ux theme: load, apply, customize, save, log
/// Phase 1500

#[derive(Debug, Clone)]
pub struct UxTheme {
    pub load_ok: bool,
    pub apply_ok: bool,
    pub customize_ok: bool,
    pub save_ok: bool,
    pub log_ok: bool,
}

impl Default for UxTheme {
    fn default() -> Self {
        Self::new()
    }
}

impl UxTheme {
    pub fn new() -> Self {
        Self {
            load_ok: true,
            apply_ok: true,
            customize_ok: true,
            save_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.load_ok && self.apply_ok && self.customize_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.save_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.load_ok || !self.apply_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.load_ok {
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
        let c = UxTheme::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UxTheme::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UxTheme::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UxTheme::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UxTheme::new();
        c.load_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UxTheme::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
