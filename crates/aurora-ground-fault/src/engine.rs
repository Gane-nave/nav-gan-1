/// ground fault: detect, isolate, measure, clear, report
/// Phase 1157

#[derive(Debug, Clone)]
pub struct GroundFault {
    pub detect_ok: bool,
    pub isolate_ok: bool,
    pub measure_ok: bool,
    pub clear_ok: bool,
    pub report_ok: bool,
}

impl Default for GroundFault {
    fn default() -> Self {
        Self::new()
    }
}

impl GroundFault {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            isolate_ok: true,
            measure_ok: true,
            clear_ok: true,
            report_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.isolate_ok && self.measure_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.clear_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.isolate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok {
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
        let c = GroundFault::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = GroundFault::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = GroundFault::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = GroundFault::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = GroundFault::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = GroundFault::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
