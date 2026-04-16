/// fleet report: collect, aggregate, format, deliver, log
/// Phase 1419

#[derive(Debug, Clone)]
pub struct FleetReport {
    pub collect_ok: bool,
    pub aggregate_ok: bool,
    pub format_ok: bool,
    pub deliver_ok: bool,
    pub log_ok: bool,
}

impl Default for FleetReport {
    fn default() -> Self {
        Self::new()
    }
}

impl FleetReport {
    pub fn new() -> Self {
        Self {
            collect_ok: true,
            aggregate_ok: true,
            format_ok: true,
            deliver_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.collect_ok && self.aggregate_ok && self.format_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.deliver_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.collect_ok || !self.aggregate_ok
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
        let c = FleetReport::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FleetReport::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FleetReport::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FleetReport::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FleetReport::new();
        c.collect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FleetReport::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
