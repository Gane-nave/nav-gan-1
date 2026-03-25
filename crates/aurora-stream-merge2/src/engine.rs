/// stream merge2: combine, interleave, priority, order, log
/// Phase 1928

#[derive(Debug, Clone)]
pub struct StreamMerge2 {
    pub combine_ok: bool,
    pub interleave_ok: bool,
    pub priority_ok: bool,
    pub order_ok: bool,
    pub log_ok: bool,
}

impl Default for StreamMerge2 {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamMerge2 {
    pub fn new() -> Self {
        Self {
            combine_ok: true,
            interleave_ok: true,
            priority_ok: true,
            order_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.combine_ok && self.interleave_ok && self.priority_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.order_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.combine_ok || !self.interleave_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.combine_ok {
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
        let c = StreamMerge2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = StreamMerge2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StreamMerge2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = StreamMerge2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = StreamMerge2::new();
        c.combine_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = StreamMerge2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
