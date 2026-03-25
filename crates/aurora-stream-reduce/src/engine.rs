/// stream reduce: fold, scan, aggregate, emit, log
/// Phase 1924

#[derive(Debug, Clone)]
pub struct StreamReduce {
    pub fold_ok: bool,
    pub scan_ok: bool,
    pub aggregate_ok: bool,
    pub emit_ok: bool,
    pub log_ok: bool,
}

impl Default for StreamReduce {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamReduce {
    pub fn new() -> Self {
        Self {
            fold_ok: true,
            scan_ok: true,
            aggregate_ok: true,
            emit_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.fold_ok && self.scan_ok && self.aggregate_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.emit_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.fold_ok || !self.scan_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.fold_ok {
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
        let c = StreamReduce::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = StreamReduce::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StreamReduce::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = StreamReduce::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = StreamReduce::new();
        c.fold_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = StreamReduce::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
