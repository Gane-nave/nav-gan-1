/// ux sort: define, apply, direction, stable, log
/// Phase 1511

#[derive(Debug, Clone)]
pub struct UxSort {
    pub define_ok: bool,
    pub apply_ok: bool,
    pub direction_ok: bool,
    pub stable_ok: bool,
    pub log_ok: bool,
}

impl Default for UxSort {
    fn default() -> Self {
        Self::new()
    }
}

impl UxSort {
    pub fn new() -> Self {
        Self {
            define_ok: true,
            apply_ok: true,
            direction_ok: true,
            stable_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.define_ok && self.apply_ok && self.direction_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.stable_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.define_ok || !self.apply_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.define_ok {
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
        let c = UxSort::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UxSort::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UxSort::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UxSort::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UxSort::new();
        c.define_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UxSort::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
