/// trans priority2: classify, queue, schedule, send, log
/// Phase 2276

#[derive(Debug, Clone)]
pub struct TransPriority2 {
    pub classify_ok: bool,
    pub queue_ok: bool,
    pub schedule_ok: bool,
    pub send_ok: bool,
    pub log_ok: bool,
}

impl Default for TransPriority2 {
    fn default() -> Self {
        Self::new()
    }
}

impl TransPriority2 {
    pub fn new() -> Self {
        Self {
            classify_ok: true,
            queue_ok: true,
            schedule_ok: true,
            send_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.classify_ok && self.queue_ok && self.schedule_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.send_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.classify_ok || !self.queue_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.classify_ok {
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
        let c = TransPriority2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = TransPriority2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = TransPriority2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = TransPriority2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = TransPriority2::new();
        c.classify_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = TransPriority2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
