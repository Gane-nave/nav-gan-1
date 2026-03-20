/// Recent destinations: history, frequency, prediction, suggest
/// Phase 909

#[derive(Debug, Clone)]
pub struct RecentDest {
    pub history_ok: bool,
    pub frequency_ok: bool,
    pub predict_ok: bool,
    pub suggest_ok: bool,
    pub privacy_ok: bool,
}

impl Default for RecentDest {
    fn default() -> Self {
        Self::new()
    }
}

impl RecentDest {
    pub fn new() -> Self {
        Self {
            history_ok: true,
            frequency_ok: true,
            predict_ok: true,
            suggest_ok: true,
            privacy_ok: true,
        }
    }

    pub fn tracking_ok(&self) -> bool {
        self.history_ok && self.frequency_ok
    }

    pub fn intelligence_ok(&self) -> bool {
        self.predict_ok && self.suggest_ok
    }

    pub fn all_ok(&self) -> bool {
        self.tracking_ok() && self.intelligence_ok() && self.privacy_ok
    }

    pub fn needs_cleanup(&self) -> bool {
        !self.privacy_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.history_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracking() {
        let c = RecentDest::new();
        assert!(c.tracking_ok());
    }

    #[test]
    fn test_intelligence() {
        let c = RecentDest::new();
        assert!(c.intelligence_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RecentDest::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_cleanup() {
        let c = RecentDest::new();
        assert!(!c.needs_cleanup());
    }

    #[test]
    fn test_privacy() {
        let mut c = RecentDest::new();
        c.privacy_ok = false;
        assert!(c.needs_cleanup());
    }

    #[test]
    fn test_health() {
        let c = RecentDest::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
