/// ml experiment: create, track, compare, archive, log
/// Phase 1955

#[derive(Debug, Clone)]
pub struct MlExperiment {
    pub create_ok: bool,
    pub track_ok: bool,
    pub compare_ok: bool,
    pub archive_ok: bool,
    pub log_ok: bool,
}

impl Default for MlExperiment {
    fn default() -> Self {
        Self::new()
    }
}

impl MlExperiment {
    pub fn new() -> Self {
        Self {
            create_ok: true,
            track_ok: true,
            compare_ok: true,
            archive_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.create_ok && self.track_ok && self.compare_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.archive_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.create_ok || !self.track_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.create_ok {
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
        let c = MlExperiment::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MlExperiment::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MlExperiment::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MlExperiment::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MlExperiment::new();
        c.create_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MlExperiment::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
