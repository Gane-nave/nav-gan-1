/// aurora-dash-theme: dash theme
/// Phase 2444

#[derive(Debug, Clone)]
pub struct DashTheme {
    pub apply_ok: bool,
    pub switch_ok: bool,
    pub dark_ok: bool,
    pub light_ok: bool,
    pub custom_ok: bool,
}

impl Default for DashTheme {
    fn default() -> Self {
        Self::new()
    }
}

impl DashTheme {
    pub fn new() -> Self {
        Self {
            apply_ok: true,
            switch_ok: true,
            dark_ok: true,
            light_ok: true,
            custom_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.apply_ok && self.switch_ok && self.dark_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.light_ok && self.custom_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.apply_ok || !self.switch_ok
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
        let c = DashTheme::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DashTheme::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DashTheme::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DashTheme::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DashTheme::new();
        c.apply_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DashTheme::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = DashTheme::default();
        assert!(c.all_ok());
    }
}
