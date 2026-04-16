/// aurora-dash-report: dash report
/// Phase 2453

#[derive(Debug, Clone)]
pub struct DashReport {
    pub generate_ok: bool,
    pub schedule_ok: bool,
    pub export_ok: bool,
    pub email_ok: bool,
    pub archive_ok: bool,
}

impl Default for DashReport {
    fn default() -> Self {
        Self::new()
    }
}

impl DashReport {
    pub fn new() -> Self {
        Self {
            generate_ok: true,
            schedule_ok: true,
            export_ok: true,
            email_ok: true,
            archive_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.generate_ok && self.schedule_ok && self.export_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.email_ok && self.archive_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.generate_ok || !self.schedule_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.generate_ok {
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
        let c = DashReport::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DashReport::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DashReport::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DashReport::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DashReport::new();
        c.generate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DashReport::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_default() {
        let c = DashReport::default();
        assert!(c.all_ok());
    }
}
