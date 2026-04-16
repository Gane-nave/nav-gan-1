/// aurora-dash-compare: dash compare
/// Phase 2455

#[derive(Debug, Clone)]
pub struct DashCompare {
    pub select_ok: bool,
    pub diff_ok: bool,
    pub overlay_ok: bool,
    pub export_ok: bool,
    pub reset_ok: bool,
}

impl Default for DashCompare {
    fn default() -> Self {
        Self::new()
    }
}

impl DashCompare {
    pub fn new() -> Self {
        Self {
            select_ok: true,
            diff_ok: true,
            overlay_ok: true,
            export_ok: true,
            reset_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.select_ok && self.diff_ok && self.overlay_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.export_ok && self.reset_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.select_ok || !self.diff_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.select_ok {
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
        let c = DashCompare::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DashCompare::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DashCompare::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DashCompare::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DashCompare::new();
        c.select_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DashCompare::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = DashCompare::default();
        assert!(c.all_ok());
    }
}
