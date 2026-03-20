/// stream buffer: collect, flush, timeout, overflow, log
/// Phase 1929

#[derive(Debug, Clone)]
pub struct StreamBuffer {
    pub collect_ok: bool,
    pub flush_ok: bool,
    pub timeout_ok: bool,
    pub overflow_ok: bool,
    pub log_ok: bool,
}

impl Default for StreamBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamBuffer {
    pub fn new() -> Self {
        Self {
            collect_ok: true,
            flush_ok: true,
            timeout_ok: true,
            overflow_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.collect_ok && self.flush_ok && self.timeout_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.overflow_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.collect_ok || !self.flush_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.collect_ok {
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
        let c = StreamBuffer::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = StreamBuffer::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StreamBuffer::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = StreamBuffer::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = StreamBuffer::new();
        c.collect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = StreamBuffer::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
