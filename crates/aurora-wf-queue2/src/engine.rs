/// wf queue2: enqueue, dequeue, priority, retry, log
/// Phase 2231

#[derive(Debug, Clone)]
pub struct WfQueue2 {
    pub enqueue_ok: bool,
    pub dequeue_ok: bool,
    pub priority_ok: bool,
    pub retry_ok: bool,
    pub log_ok: bool,
}

impl Default for WfQueue2 {
    fn default() -> Self {
        Self::new()
    }
}

impl WfQueue2 {
    pub fn new() -> Self {
        Self {
            enqueue_ok: true,
            dequeue_ok: true,
            priority_ok: true,
            retry_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.enqueue_ok && self.dequeue_ok && self.priority_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.retry_ok && self.log_ok
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
        let c = WfQueue2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WfQueue2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WfQueue2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WfQueue2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WfQueue2::new();
        c.enqueue_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WfQueue2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
