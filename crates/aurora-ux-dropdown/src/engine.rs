/// ux dropdown: open, filter, select, close, log
/// Phase 1516

#[derive(Debug, Clone)]
pub struct UxDropdown {
    pub open_ok: bool,
    pub filter_ok: bool,
    pub select_ok: bool,
    pub close_ok: bool,
    pub log_ok: bool,
}

impl Default for UxDropdown {
    fn default() -> Self {
        Self::new()
    }
}

impl UxDropdown {
    pub fn new() -> Self {
        Self {
            open_ok: true,
            filter_ok: true,
            select_ok: true,
            close_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.open_ok && self.filter_ok && self.select_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.close_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.open_ok || !self.filter_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.open_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = UxDropdown::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UxDropdown::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UxDropdown::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UxDropdown::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UxDropdown::new();
        c.open_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UxDropdown::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
