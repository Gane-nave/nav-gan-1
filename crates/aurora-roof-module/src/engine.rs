/// roof module: open, close, tilt, shade, check
/// Phase 1276

#[derive(Debug, Clone)]
pub struct RoofModule {
    pub open_ok: bool,
    pub close_ok: bool,
    pub tilt_ok: bool,
    pub shade_ok: bool,
    pub check_ok: bool,
}

impl Default for RoofModule {
    fn default() -> Self {
        Self::new()
    }
}

impl RoofModule {
    pub fn new() -> Self {
        Self {
            open_ok: true,
            close_ok: true,
            tilt_ok: true,
            shade_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.open_ok && self.close_ok && self.tilt_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.shade_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.open_ok || !self.close_ok
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
        let c = RoofModule::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RoofModule::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RoofModule::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RoofModule::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RoofModule::new();
        c.open_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RoofModule::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
