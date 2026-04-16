/// Stream processing: window, aggregate, join, filter, emit
/// Phase 1034

#[derive(Debug, Clone)]
pub struct StreamProc {
    pub window_ok: bool,
    pub aggregate_ok: bool,
    pub join_ok: bool,
    pub filter_ok: bool,
    pub emit_ok: bool,
}

impl Default for StreamProc {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamProc {
    pub fn new() -> Self {
        Self {
            window_ok: true,
            aggregate_ok: true,
            join_ok: true,
            filter_ok: true,
            emit_ok: true,
        }
    }

    pub fn processing_ok(&self) -> bool {
        self.window_ok && self.aggregate_ok && self.join_ok
    }

    pub fn output_ok(&self) -> bool {
        self.filter_ok && self.emit_ok
    }

    pub fn all_ok(&self) -> bool {
        self.processing_ok() && self.output_ok()
    }

    pub fn needs_reset(&self) -> bool {
        !self.window_ok || !self.aggregate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.window_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processing() {
        let c = StreamProc::new();
        assert!(c.processing_ok());
    }

    #[test]
    fn test_output() {
        let c = StreamProc::new();
        assert!(c.output_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StreamProc::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_reset() {
        let c = StreamProc::new();
        assert!(!c.needs_reset());
    }

    #[test]
    fn test_window() {
        let mut c = StreamProc::new();
        c.window_ok = false;
        assert!(c.needs_reset());
    }

    #[test]
    fn test_health() {
        let c = StreamProc::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
