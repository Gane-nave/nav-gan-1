/// egr valve: open, close, recirculate, cool, check
/// Phase 1238

#[derive(Debug, Clone)]
pub struct EgrValve {
    pub open_ok: bool,
    pub close_ok: bool,
    pub recirculate_ok: bool,
    pub cool_ok: bool,
    pub check_ok: bool,
}

impl Default for EgrValve {
    fn default() -> Self {
        Self::new()
    }
}

impl EgrValve {
    pub fn new() -> Self {
        Self {
            open_ok: true,
            close_ok: true,
            recirculate_ok: true,
            cool_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.open_ok && self.close_ok && self.recirculate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cool_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.open_ok || !self.close_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.open_ok {
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
        let c = EgrValve::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EgrValve::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EgrValve::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EgrValve::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EgrValve::new();
        c.open_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EgrValve::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
