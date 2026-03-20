/// dct ctrl: preselect, engage, swap, cool, report
/// Phase 1215

#[derive(Debug, Clone)]
pub struct DctCtrl {
    pub preselect_ok: bool,
    pub engage_ok: bool,
    pub swap_ok: bool,
    pub cool_ok: bool,
    pub report_ok: bool,
}

impl Default for DctCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl DctCtrl {
    pub fn new() -> Self {
        Self {
            preselect_ok: true,
            engage_ok: true,
            swap_ok: true,
            cool_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.preselect_ok && self.engage_ok && self.swap_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cool_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.preselect_ok || !self.engage_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.preselect_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = DctCtrl::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DctCtrl::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DctCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DctCtrl::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DctCtrl::new();
        c.preselect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DctCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
