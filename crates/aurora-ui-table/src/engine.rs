/// aurora-ui-table: ui table
/// Phase 2402

#[derive(Debug, Clone)]
pub struct UiTable {
    pub sort_ok: bool,
    pub filter_ok: bool,
    pub page_ok: bool,
    pub select_ok: bool,
    pub export_ok: bool,
}

impl Default for UiTable {
    fn default() -> Self {
        Self::new()
    }
}

impl UiTable {
    pub fn new() -> Self {
        Self {
            sort_ok: true,
            filter_ok: true,
            page_ok: true,
            select_ok: true,
            export_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.sort_ok && self.filter_ok && self.page_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.select_ok && self.export_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.sort_ok || !self.filter_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sort_ok {
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
        let c = UiTable::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UiTable::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UiTable::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UiTable::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UiTable::new();
        c.sort_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UiTable::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = UiTable::default();
        assert!(c.all_ok());
    }
}
