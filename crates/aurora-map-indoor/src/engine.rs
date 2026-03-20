/// map indoor: scan, model, navigate, locate, log
/// Phase 1443

#[derive(Debug, Clone)]
pub struct MapIndoor {
    pub scan_ok: bool,
    pub model_ok: bool,
    pub navigate_ok: bool,
    pub locate_ok: bool,
    pub log_ok: bool,
}

impl Default for MapIndoor {
    fn default() -> Self {
        Self::new()
    }
}

impl MapIndoor {
    pub fn new() -> Self {
        Self {
            scan_ok: true,
            model_ok: true,
            navigate_ok: true,
            locate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.scan_ok && self.model_ok && self.navigate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.locate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.scan_ok || !self.model_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.scan_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = MapIndoor::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MapIndoor::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MapIndoor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MapIndoor::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MapIndoor::new();
        c.scan_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MapIndoor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
