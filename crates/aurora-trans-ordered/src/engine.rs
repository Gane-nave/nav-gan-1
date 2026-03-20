/// trans ordered: sequence, buffer, deliver, flush, log
/// Phase 2275

#[derive(Debug, Clone)]
pub struct TransOrdered {
    pub sequence_ok: bool,
    pub buffer_ok: bool,
    pub deliver_ok: bool,
    pub flush_ok: bool,
    pub log_ok: bool,
}

impl Default for TransOrdered {
    fn default() -> Self {
        Self::new()
    }
}

impl TransOrdered {
    pub fn new() -> Self {
        Self {
            sequence_ok: true,
            buffer_ok: true,
            deliver_ok: true,
            flush_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.sequence_ok && self.buffer_ok && self.deliver_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.flush_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.sequence_ok || !self.buffer_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.sequence_ok {
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
        let c = TransOrdered::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TransOrdered::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TransOrdered::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TransOrdered::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TransOrdered::new();
        c.sequence_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TransOrdered::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
