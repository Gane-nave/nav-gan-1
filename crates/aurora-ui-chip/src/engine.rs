/// aurora-ui-chip: ui chip
/// Phase 2418

#[derive(Debug, Clone)]
pub struct UiChip {
    pub add_ok: bool,
    pub remove_ok: bool,
    pub select_ok: bool,
    pub focus_ok: bool,
    pub color_ok: bool,
}

impl Default for UiChip {
    fn default() -> Self {
        Self::new()
    }
}

impl UiChip {
    pub fn new() -> Self {
        Self {
            add_ok: true,
            remove_ok: true,
            select_ok: true,
            focus_ok: true,
            color_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.add_ok && self.remove_ok && self.select_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.focus_ok && self.color_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.add_ok || !self.remove_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.add_ok {
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
        let c = UiChip::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UiChip::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UiChip::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UiChip::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UiChip::new();
        c.add_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UiChip::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = UiChip::default();
        assert!(c.all_ok());
    }
}
