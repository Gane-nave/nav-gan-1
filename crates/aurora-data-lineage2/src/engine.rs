/// data lineage2: track, visualize, impact, audit, log
/// Phase 2209

#[derive(Debug, Clone)]
pub struct DataLineage2 {
    pub track_ok: bool,
    pub visualize_ok: bool,
    pub impact_ok: bool,
    pub audit_ok: bool,
    pub log_ok: bool,
}

impl Default for DataLineage2 {
    fn default() -> Self {
        Self::new()
    }
}

impl DataLineage2 {
    pub fn new() -> Self {
        Self {
            track_ok: true,
            visualize_ok: true,
            impact_ok: true,
            audit_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.track_ok && self.visualize_ok && self.impact_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.audit_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.track_ok || !self.visualize_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.track_ok {
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
        let c = DataLineage2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = DataLineage2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = DataLineage2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = DataLineage2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = DataLineage2::new();
        c.track_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = DataLineage2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
