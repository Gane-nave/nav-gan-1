/// radio tuner: scan, tune, seek, store, recall
/// Phase 1177

#[derive(Debug, Clone)]
pub struct RadioTuner {
    pub scan_ok: bool,
    pub tune_ok: bool,
    pub seek_ok: bool,
    pub store_ok: bool,
    pub recall_ok: bool,
}

impl Default for RadioTuner {
    fn default() -> Self {
        Self::new()
    }
}

impl RadioTuner {
    pub fn new() -> Self {
        Self {
            scan_ok: true,
            tune_ok: true,
            seek_ok: true,
            store_ok: true,
            recall_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.scan_ok && self.tune_ok && self.seek_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.store_ok && self.recall_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.scan_ok || !self.tune_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.scan_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = RadioTuner::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RadioTuner::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RadioTuner::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RadioTuner::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RadioTuner::new();
        c.scan_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RadioTuner::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
