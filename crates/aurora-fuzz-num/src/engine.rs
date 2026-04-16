/// aurora-fuzz-num: fuzz num
/// Phase 2500

#[derive(Debug, Clone)]
pub struct FuzzNum {
    pub generate_ok: bool,
    pub boundary_ok: bool,
    pub overflow_ok: bool,
    pub nan_ok: bool,
    pub report_ok: bool,
}

impl Default for FuzzNum {
    fn default() -> Self {
        Self::new()
    }
}

impl FuzzNum {
    pub fn new() -> Self {
        Self {
            generate_ok: true,
            boundary_ok: true,
            overflow_ok: true,
            nan_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.generate_ok && self.boundary_ok && self.overflow_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.nan_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.generate_ok || !self.boundary_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.generate_ok {
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
        let c = FuzzNum::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FuzzNum::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FuzzNum::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FuzzNum::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FuzzNum::new();
        c.generate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FuzzNum::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = FuzzNum::default();
        assert!(c.all_ok());
    }
}
