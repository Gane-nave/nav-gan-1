/// stream join: inner, left, outer, temporal, log
/// Phase 1926

#[derive(Debug, Clone)]
pub struct StreamJoin {
    pub inner_ok: bool,
    pub left_ok: bool,
    pub outer_ok: bool,
    pub temporal_ok: bool,
    pub log_ok: bool,
}

impl Default for StreamJoin {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamJoin {
    pub fn new() -> Self {
        Self {
            inner_ok: true,
            left_ok: true,
            outer_ok: true,
            temporal_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.inner_ok && self.left_ok && self.outer_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.temporal_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.inner_ok || !self.left_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.inner_ok {
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
        let c = StreamJoin::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = StreamJoin::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StreamJoin::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = StreamJoin::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = StreamJoin::new();
        c.inner_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = StreamJoin::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
