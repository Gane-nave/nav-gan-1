/// fmt arrow: build, read, verify, transform, log
/// Phase 1675

#[derive(Debug, Clone)]
pub struct FmtArrow {
    pub build_ok: bool,
    pub read_ok: bool,
    pub verify_ok: bool,
    pub transform_ok: bool,
    pub log_ok: bool,
}

impl Default for FmtArrow {
    fn default() -> Self {
        Self::new()
    }
}

impl FmtArrow {
    pub fn new() -> Self {
        Self {
            build_ok: true,
            read_ok: true,
            verify_ok: true,
            transform_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.build_ok && self.read_ok && self.verify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.transform_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.build_ok || !self.read_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.build_ok {
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
        let c = FmtArrow::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FmtArrow::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FmtArrow::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FmtArrow::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FmtArrow::new();
        c.build_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FmtArrow::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
