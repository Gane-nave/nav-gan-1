/// aurora-log-loki: log loki
/// Phase 2613

#[derive(Debug, Clone)]
pub struct LogLoki {
    pub push_ok: bool,
    pub query_ok: bool,
    pub label_ok: bool,
    pub stream_ok: bool,
    pub tail_ok: bool,
}

impl Default for LogLoki {
    fn default() -> Self {
        Self::new()
    }
}

impl LogLoki {
    pub fn new() -> Self {
        Self {
            push_ok: true,
            query_ok: true,
            label_ok: true,
            stream_ok: true,
            tail_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.push_ok && self.query_ok && self.label_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.stream_ok && self.tail_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.push_ok || !self.query_ok
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
        let c = LogLoki::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = LogLoki::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = LogLoki::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = LogLoki::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = LogLoki::new();
        c.push_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = LogLoki::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = LogLoki::default();
        assert!(c.all_ok());
    }
}
