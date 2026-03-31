/// aurora-ui-grid: ui grid
/// Phase 2410

#[derive(Debug, Clone)]
pub struct UiGrid {
    pub layout_ok: bool,
    pub cols_ok: bool,
    pub rows_ok: bool,
    pub gap_ok: bool,
    pub align_ok: bool,
}

impl Default for UiGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl UiGrid {
    pub fn new() -> Self {
        Self {
            layout_ok: true,
            cols_ok: true,
            rows_ok: true,
            gap_ok: true,
            align_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.layout_ok && self.cols_ok && self.rows_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.gap_ok && self.align_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.layout_ok || !self.cols_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.layout_ok {
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
        let c = UiGrid::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UiGrid::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UiGrid::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UiGrid::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UiGrid::new();
        c.layout_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UiGrid::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = UiGrid::default();
        assert!(c.all_ok());
    }
}
