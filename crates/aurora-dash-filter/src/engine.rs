/// aurora-dash-filter: dash filter
/// Phase 2448

#[derive(Debug, Clone)]
pub struct DashFilter {
    pub apply_ok: bool,
    pub clear_ok: bool,
    pub save_ok: bool,
    pub load_ok: bool,
    pub validate_ok: bool,
}

impl Default for DashFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl DashFilter {
    pub fn new() -> Self {
        Self {
            apply_ok: true,
            clear_ok: true,
            save_ok: true,
            load_ok: true,
            validate_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.apply_ok && self.clear_ok && self.save_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.load_ok && self.validate_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.apply_ok || !self.clear_ok
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
        let c = DashFilter::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DashFilter::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DashFilter::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DashFilter::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DashFilter::new();
        c.apply_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DashFilter::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = DashFilter::default();
        assert!(c.all_ok());
    }
}
