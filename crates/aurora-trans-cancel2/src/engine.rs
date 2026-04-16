/// trans cancel2: request, propagate, handle, cleanup, log
/// Phase 2291

#[derive(Debug, Clone)]
pub struct TransCancel2 {
    pub request_ok: bool,
    pub propagate_ok: bool,
    pub handle_ok: bool,
    pub cleanup_ok: bool,
    pub log_ok: bool,
}

impl Default for TransCancel2 {
    fn default() -> Self {
        Self::new()
    }
}

impl TransCancel2 {
    pub fn new() -> Self {
        Self {
            request_ok: true,
            propagate_ok: true,
            handle_ok: true,
            cleanup_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.request_ok && self.propagate_ok && self.handle_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cleanup_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.request_ok || !self.propagate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.request_ok {
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
        let c = TransCancel2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TransCancel2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TransCancel2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TransCancel2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TransCancel2::new();
        c.request_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TransCancel2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
