/// Threat intel: IOC, feed, score, block, report
/// Phase 1006

#[derive(Debug, Clone)]
pub struct ThreatIntel {
    pub ioc_ok: bool,
    pub feed_ok: bool,
    pub score_ok: bool,
    pub block_ok: bool,
    pub report_ok: bool,
}

impl Default for ThreatIntel {
    fn default() -> Self {
        Self::new()
    }
}

impl ThreatIntel {
    pub fn new() -> Self {
        Self {
            ioc_ok: true,
            feed_ok: true,
            score_ok: true,
            block_ok: true,
            report_ok: true,
        }
    }

    pub fn intelligence_ok(&self) -> bool {
        self.ioc_ok && self.feed_ok && self.score_ok
    }

    pub fn response_ok(&self) -> bool {
        self.block_ok && self.report_ok
    }

    pub fn all_ok(&self) -> bool {
        self.intelligence_ok() && self.response_ok()
    }

    pub fn needs_update(&self) -> bool {
        !self.feed_ok || !self.ioc_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.feed_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intelligence() {
        let c = ThreatIntel::new();
        assert!(c.intelligence_ok());
    }

    #[test]
    fn test_response() {
        let c = ThreatIntel::new();
        assert!(c.response_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ThreatIntel::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_update() {
        let c = ThreatIntel::new();
        assert!(!c.needs_update());
    }

    #[test]
    fn test_feed() {
        let mut c = ThreatIntel::new();
        c.feed_ok = false;
        assert!(c.needs_update());
    }

    #[test]
    fn test_health() {
        let c = ThreatIntel::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
