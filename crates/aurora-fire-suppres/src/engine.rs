/// Fire suppression: detect, agent, nozzle, manual, auto
/// Phase 957

#[derive(Debug, Clone)]
pub struct FireSuppres {
    pub detect_ok: bool,
    pub agent_ok: bool,
    pub nozzle_ok: bool,
    pub manual_ok: bool,
    pub auto_ok: bool,
}

impl Default for FireSuppres {
    fn default() -> Self {
        Self::new()
    }
}

impl FireSuppres {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            agent_ok: true,
            nozzle_ok: true,
            manual_ok: true,
            auto_ok: true,
        }
    }

    pub fn detection_ok(&self) -> bool {
        self.detect_ok && self.auto_ok
    }

    pub fn suppression_ok(&self) -> bool {
        self.agent_ok && self.nozzle_ok && self.manual_ok
    }

    pub fn all_ok(&self) -> bool {
        self.detection_ok() && self.suppression_ok()
    }

    pub fn needs_service(&self) -> bool {
        !self.agent_ok || !self.nozzle_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.agent_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detection() {
        let c = FireSuppres::new();
        assert!(c.detection_ok());
    }

    #[test]
    fn test_suppression() {
        let c = FireSuppres::new();
        assert!(c.suppression_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = FireSuppres::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = FireSuppres::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_agent() {
        let mut c = FireSuppres::new();
        c.agent_ok = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = FireSuppres::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
