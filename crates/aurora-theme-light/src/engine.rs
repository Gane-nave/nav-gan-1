/// aurora-theme-light: theme light
/// Phase 2469

#[derive(Debug, Clone)]
pub struct ThemeLight {
    pub apply_ok: bool,
    pub toggle_ok: bool,
    pub save_ok: bool,
    pub reset_ok: bool,
    pub preview_ok: bool,
}

impl Default for ThemeLight {
    fn default() -> Self {
        Self::new()
    }
}

impl ThemeLight {
    pub fn new() -> Self {
        Self {
            apply_ok: true,
            toggle_ok: true,
            save_ok: true,
            reset_ok: true,
            preview_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.apply_ok && self.toggle_ok && self.save_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reset_ok && self.preview_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.apply_ok || !self.toggle_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.apply_ok {
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
        let c = ThemeLight::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ThemeLight::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ThemeLight::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ThemeLight::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ThemeLight::new();
        c.apply_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ThemeLight::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = ThemeLight::default();
        assert!(c.all_ok());
    }
}
