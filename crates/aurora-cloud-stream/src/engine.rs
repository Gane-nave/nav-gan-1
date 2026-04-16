/// cloud stream: produce, consume, partition, replay, log
/// Phase 1462

#[derive(Debug, Clone)]
pub struct CloudStream {
    pub produce_ok: bool,
    pub consume_ok: bool,
    pub partition_ok: bool,
    pub replay_ok: bool,
    pub log_ok: bool,
}

impl Default for CloudStream {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudStream {
    pub fn new() -> Self {
        Self {
            produce_ok: true,
            consume_ok: true,
            partition_ok: true,
            replay_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.produce_ok && self.consume_ok && self.partition_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.replay_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.produce_ok || !self.consume_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.produce_ok {
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
        let c = CloudStream::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CloudStream::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CloudStream::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CloudStream::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CloudStream::new();
        c.produce_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CloudStream::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
