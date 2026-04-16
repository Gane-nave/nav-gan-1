/// obs profile2: start, collect, analyze, report, log
/// Phase 2152

#[derive(Debug, Clone)]
pub struct ObsProfile2 {
    pub start_ok: bool,
    pub collect_ok: bool,
    pub analyze_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for ObsProfile2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ObsProfile2 {
    pub fn new() -> Self {
        Self {
            start_ok: true,
            collect_ok: true,
            analyze_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.start_ok && self.collect_ok && self.analyze_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.start_ok || !self.collect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.start_ok {
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
        let c = ObsProfile2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ObsProfile2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ObsProfile2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ObsProfile2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ObsProfile2::new();
        c.start_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ObsProfile2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
