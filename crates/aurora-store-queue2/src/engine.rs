/// store queue2: enqueue, dequeue, peek, size, log
/// Phase 1977

#[derive(Debug, Clone)]
pub struct StoreQueue2 {
    pub enqueue_ok: bool,
    pub dequeue_ok: bool,
    pub peek_ok: bool,
    pub size_ok: bool,
    pub log_ok: bool,
}

impl Default for StoreQueue2 {
    fn default() -> Self {
        Self::new()
    }
}

impl StoreQueue2 {
    pub fn new() -> Self {
        Self {
            enqueue_ok: true,
            dequeue_ok: true,
            peek_ok: true,
            size_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.enqueue_ok && self.dequeue_ok && self.peek_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.size_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.enqueue_ok || !self.dequeue_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.enqueue_ok {
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
        let c = StoreQueue2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = StoreQueue2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StoreQueue2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = StoreQueue2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = StoreQueue2::new();
        c.enqueue_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = StoreQueue2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
