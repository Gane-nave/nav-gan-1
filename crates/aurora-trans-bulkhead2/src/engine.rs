/// trans bulkhead2: partition, limit, queue, reject, log
/// Phase 2288

#[derive(Debug, Clone)]
pub struct TransBulkhead2 {
    pub partition_ok: bool,
    pub limit_ok: bool,
    pub queue_ok: bool,
    pub reject_ok: bool,
    pub log_ok: bool,
}

impl Default for TransBulkhead2 {
    fn default() -> Self {
        Self::new()
    }
}

impl TransBulkhead2 {
    pub fn new() -> Self {
        Self {
            partition_ok: true,
            limit_ok: true,
            queue_ok: true,
            reject_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.partition_ok && self.limit_ok && self.queue_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reject_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.partition_ok || !self.limit_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.partition_ok {
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
        let c = TransBulkhead2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TransBulkhead2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TransBulkhead2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TransBulkhead2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TransBulkhead2::new();
        c.partition_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TransBulkhead2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
