/// sched batch3: collect, execute, retry, report, log
/// Phase 2353

#[derive(Debug, Clone)]
pub struct SchedBatch3 {
    pub collect_ok: bool,
    pub execute_ok: bool,
    pub retry_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for SchedBatch3 {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedBatch3 {
    pub fn new() -> Self {
        Self {
            collect_ok: true,
            execute_ok: true,
            retry_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.collect_ok && self.execute_ok && self.retry_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.collect_ok || !self.execute_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.collect_ok {
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
        let c = SchedBatch3::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SchedBatch3::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SchedBatch3::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SchedBatch3::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SchedBatch3::new();
        c.collect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SchedBatch3::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
