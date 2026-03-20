/// feature rollout: start, progress, pause, complete, log
/// Phase 1781

#[derive(Debug, Clone)]
pub struct FeatureRollout {
    pub start_ok: bool,
    pub progress_ok: bool,
    pub pause_ok: bool,
    pub complete_ok: bool,
    pub log_ok: bool,
}

impl Default for FeatureRollout {
    fn default() -> Self {
        Self::new()
    }
}

impl FeatureRollout {
    pub fn new() -> Self {
        Self {
            start_ok: true,
            progress_ok: true,
            pause_ok: true,
            complete_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.start_ok && self.progress_ok && self.pause_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.complete_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.start_ok || !self.progress_ok
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
        let c = FeatureRollout::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = FeatureRollout::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FeatureRollout::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = FeatureRollout::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = FeatureRollout::new();
        c.start_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = FeatureRollout::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
