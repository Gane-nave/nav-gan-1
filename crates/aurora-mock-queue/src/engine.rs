/// aurora-mock-queue: mock queue
/// Phase 2485

#[derive(Debug, Clone)]
pub struct MockQueue {
    pub enqueue_ok: bool,
    pub dequeue_ok: bool,
    pub peek_ok: bool,
    pub clear_ok: bool,
    pub verify_ok: bool,
}

impl Default for MockQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl MockQueue {
    pub fn new() -> Self {
        Self {
            enqueue_ok: true,
            dequeue_ok: true,
            peek_ok: true,
            clear_ok: true,
            verify_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.enqueue_ok && self.dequeue_ok && self.peek_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.clear_ok && self.verify_ok
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
        let c = MockQueue::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MockQueue::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MockQueue::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MockQueue::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MockQueue::new();
        c.enqueue_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MockQueue::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = MockQueue::default();
        assert!(c.all_ok());
    }
}
