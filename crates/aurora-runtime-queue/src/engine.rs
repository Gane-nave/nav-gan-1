/// runtime queue: push, pop, peek, drain, log
/// Phase 1798

#[derive(Debug, Clone)]
pub struct RuntimeQueue {
    pub push_ok: bool,
    pub pop_ok: bool,
    pub peek_ok: bool,
    pub drain_ok: bool,
    pub log_ok: bool,
}

impl Default for RuntimeQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeQueue {
    pub fn new() -> Self {
        Self {
            push_ok: true,
            pop_ok: true,
            peek_ok: true,
            drain_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.push_ok && self.pop_ok && self.peek_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.drain_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.push_ok || !self.pop_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.push_ok {
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
        let c = RuntimeQueue::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RuntimeQueue::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RuntimeQueue::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RuntimeQueue::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RuntimeQueue::new();
        c.push_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RuntimeQueue::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
