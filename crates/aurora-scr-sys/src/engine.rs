/// scr sys: inject, reduce, dose, heat, check
/// Phase 1241

#[derive(Debug, Clone)]
pub struct ScrSys {
    pub inject_ok: bool,
    pub reduce_ok: bool,
    pub dose_ok: bool,
    pub heat_ok: bool,
    pub check_ok: bool,
}

impl Default for ScrSys {
    fn default() -> Self {
        Self::new()
    }
}

impl ScrSys {
    pub fn new() -> Self {
        Self {
            inject_ok: true,
            reduce_ok: true,
            dose_ok: true,
            heat_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.inject_ok && self.reduce_ok && self.dose_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.heat_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.inject_ok || !self.reduce_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.inject_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = ScrSys::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ScrSys::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ScrSys::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ScrSys::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ScrSys::new();
        c.inject_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ScrSys::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
