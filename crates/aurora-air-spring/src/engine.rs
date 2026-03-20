/// air spring: inflate, deflate, level, adapt, report
/// Phase 1194

#[derive(Debug, Clone)]
pub struct AirSpring {
    pub inflate_ok: bool,
    pub deflate_ok: bool,
    pub level_ok: bool,
    pub adapt_ok: bool,
    pub report_ok: bool,
}

impl Default for AirSpring {
    fn default() -> Self {
        Self::new()
    }
}

impl AirSpring {
    pub fn new() -> Self {
        Self {
            inflate_ok: true,
            deflate_ok: true,
            level_ok: true,
            adapt_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.inflate_ok && self.deflate_ok && self.level_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.adapt_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.inflate_ok || !self.deflate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.inflate_ok {
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
        let c = AirSpring::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AirSpring::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AirSpring::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AirSpring::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AirSpring::new();
        c.inflate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AirSpring::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
