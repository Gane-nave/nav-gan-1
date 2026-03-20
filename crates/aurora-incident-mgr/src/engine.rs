/// Incident manager: detect, triage, assign, resolve, postmortem
/// Phase 1082

#[derive(Debug, Clone)]
pub struct IncidentMgr {
    pub detect_ok: bool,
    pub triage_ok: bool,
    pub assign_ok: bool,
    pub resolve_ok: bool,
    pub postmortem_ok: bool,
}

impl Default for IncidentMgr {
    fn default() -> Self {
        Self::new()
    }
}

impl IncidentMgr {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            triage_ok: true,
            assign_ok: true,
            resolve_ok: true,
            postmortem_ok: true,
        }
    }

    pub fn response_ok(&self) -> bool {
        self.detect_ok && self.triage_ok && self.assign_ok
    }

    pub fn resolution_ok(&self) -> bool {
        self.resolve_ok && self.postmortem_ok
    }

    pub fn all_ok(&self) -> bool {
        self.response_ok() && self.resolution_ok()
    }

    pub fn needs_escalate(&self) -> bool {
        !self.detect_ok || !self.triage_ok
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
    fn test_response() {
        let c = IncidentMgr::new();
        assert!(c.response_ok());
    }

    #[test]
    fn test_resolution() {
        let c = IncidentMgr::new();
        assert!(c.resolution_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IncidentMgr::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_escalate() {
        let c = IncidentMgr::new();
        assert!(!c.needs_escalate());
    }

    #[test]
    fn test_detect() {
        let mut c = IncidentMgr::new();
        c.detect_ok = false;
        assert!(c.needs_escalate());
    }

    #[test]
    fn test_health() {
        let c = IncidentMgr::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
