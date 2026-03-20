/// otel context: attach, detach, current, extract, log
/// Phase 1756

#[derive(Debug, Clone)]
pub struct OtelContext {
    pub attach_ok: bool,
    pub detach_ok: bool,
    pub current_ok: bool,
    pub extract_ok: bool,
    pub log_ok: bool,
}

impl Default for OtelContext {
    fn default() -> Self {
        Self::new()
    }
}

impl OtelContext {
    pub fn new() -> Self {
        Self {
            attach_ok: true,
            detach_ok: true,
            current_ok: true,
            extract_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.attach_ok && self.detach_ok && self.current_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.extract_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.attach_ok || !self.detach_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.attach_ok {
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
        let c = OtelContext::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = OtelContext::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OtelContext::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = OtelContext::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = OtelContext::new();
        c.attach_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = OtelContext::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
