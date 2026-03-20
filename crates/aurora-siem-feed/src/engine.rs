/// SIEM feed: event, correlate, alert, enrich, forward
/// Phase 1005

#[derive(Debug, Clone)]
pub struct SiemFeed {
    pub event_ok: bool,
    pub correlate_ok: bool,
    pub alert_ok: bool,
    pub enrich_ok: bool,
    pub forward_ok: bool,
}

impl Default for SiemFeed {
    fn default() -> Self {
        Self::new()
    }
}

impl SiemFeed {
    pub fn new() -> Self {
        Self {
            event_ok: true,
            correlate_ok: true,
            alert_ok: true,
            enrich_ok: true,
            forward_ok: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.event_ok && self.correlate_ok && self.alert_ok
    }

    pub fn processing_ok(&self) -> bool {
        self.enrich_ok && self.forward_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.processing_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.event_ok || !self.forward_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.event_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = SiemFeed::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_processing() {
        let c = SiemFeed::new();
        assert!(c.processing_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SiemFeed::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = SiemFeed::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_event() {
        let mut c = SiemFeed::new();
        c.event_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = SiemFeed::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
