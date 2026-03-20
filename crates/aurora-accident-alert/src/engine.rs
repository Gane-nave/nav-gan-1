/// Accident alert: detection, notification, reroute, ETA
/// Phase 921

#[derive(Debug, Clone)]
pub struct AccidentAlert {
    pub detect_ok: bool,
    pub notify_ok: bool,
    pub reroute_ok: bool,
    pub eta_ok: bool,
    pub feed_ok: bool,
}

impl Default for AccidentAlert {
    fn default() -> Self {
        Self::new()
    }
}

impl AccidentAlert {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            notify_ok: true,
            reroute_ok: true,
            eta_ok: true,
            feed_ok: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.detect_ok && self.feed_ok
    }

    pub fn response_ok(&self) -> bool {
        self.notify_ok && self.reroute_ok && self.eta_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.response_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.feed_ok || !self.detect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = AccidentAlert::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_response() {
        let c = AccidentAlert::new();
        assert!(c.response_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AccidentAlert::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = AccidentAlert::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_feed() {
        let mut c = AccidentAlert::new();
        c.feed_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = AccidentAlert::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
