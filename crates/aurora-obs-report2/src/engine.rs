/// obs report2: generate, schedule, deliver, archive, log
/// Phase 2163

#[derive(Debug, Clone)]
pub struct ObsReport2 {
    pub generate_ok: bool,
    pub schedule_ok: bool,
    pub deliver_ok: bool,
    pub archive_ok: bool,
    pub log_ok: bool,
}

impl Default for ObsReport2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ObsReport2 {
    pub fn new() -> Self {
        Self {
            generate_ok: true,
            schedule_ok: true,
            deliver_ok: true,
            archive_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.generate_ok && self.schedule_ok && self.deliver_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.archive_ok && self.log_ok
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
        let c = ObsReport2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ObsReport2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ObsReport2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ObsReport2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ObsReport2::new();
        c.generate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ObsReport2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
