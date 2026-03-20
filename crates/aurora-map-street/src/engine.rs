/// map street: capture, stitch, navigate, annotate, log
/// Phase 1442

#[derive(Debug, Clone)]
pub struct MapStreet {
    pub capture_ok: bool,
    pub stitch_ok: bool,
    pub navigate_ok: bool,
    pub annotate_ok: bool,
    pub log_ok: bool,
}

impl Default for MapStreet {
    fn default() -> Self {
        Self::new()
    }
}

impl MapStreet {
    pub fn new() -> Self {
        Self {
            capture_ok: true,
            stitch_ok: true,
            navigate_ok: true,
            annotate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.capture_ok && self.stitch_ok && self.navigate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.annotate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.capture_ok || !self.stitch_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.capture_ok {
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
        let c = MapStreet::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MapStreet::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MapStreet::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MapStreet::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MapStreet::new();
        c.capture_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MapStreet::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
