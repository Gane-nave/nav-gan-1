/// ux carousel: load, slide, loop, indicator, log
/// Phase 1517

#[derive(Debug, Clone)]
pub struct UxCarousel {
    pub load_ok: bool,
    pub slide_ok: bool,
    pub loop_ok: bool,
    pub indicator_ok: bool,
    pub log_ok: bool,
}

impl Default for UxCarousel {
    fn default() -> Self {
        Self::new()
    }
}

impl UxCarousel {
    pub fn new() -> Self {
        Self {
            load_ok: true,
            slide_ok: true,
            loop_ok: true,
            indicator_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.load_ok && self.slide_ok && self.loop_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.indicator_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.load_ok || !self.slide_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.load_ok {
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
        let c = UxCarousel::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = UxCarousel::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = UxCarousel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = UxCarousel::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = UxCarousel::new();
        c.load_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = UxCarousel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
