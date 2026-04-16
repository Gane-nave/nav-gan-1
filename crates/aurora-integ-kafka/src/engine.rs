/// integ kafka: produce, consume, commit, seek, log
/// Phase 1650

#[derive(Debug, Clone)]
pub struct IntegKafka {
    pub produce_ok: bool,
    pub consume_ok: bool,
    pub commit_ok: bool,
    pub seek_ok: bool,
    pub log_ok: bool,
}

impl Default for IntegKafka {
    fn default() -> Self {
        Self::new()
    }
}

impl IntegKafka {
    pub fn new() -> Self {
        Self {
            produce_ok: true,
            consume_ok: true,
            commit_ok: true,
            seek_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.produce_ok && self.consume_ok && self.commit_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.seek_ok && self.log_ok
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
        let c = IntegKafka::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IntegKafka::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IntegKafka::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IntegKafka::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IntegKafka::new();
        c.produce_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IntegKafka::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
