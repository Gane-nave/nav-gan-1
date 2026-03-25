/// misfire mon: detect, count, identify, report, log
/// Phase 1383

#[derive(Debug, Clone)]
pub struct MisfireMon {
    pub detect_ok: bool,
    pub count_ok: bool,
    pub identify_ok: bool,
    pub report_ok: bool,
    pub log_ok: bool,
}

impl Default for MisfireMon {
    fn default() -> Self {
        Self::new()
    }
}

impl MisfireMon {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            count_ok: true,
            identify_ok: true,
            report_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.count_ok && self.identify_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.report_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.count_ok
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
        let c = MisfireMon::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MisfireMon::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MisfireMon::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MisfireMon::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MisfireMon::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MisfireMon::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
