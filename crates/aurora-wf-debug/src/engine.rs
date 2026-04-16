/// wf debug: trace, inspect, replay, breakpoint, log
/// Phase 2237

#[derive(Debug, Clone)]
pub struct WfDebug {
    pub trace_ok: bool,
    pub inspect_ok: bool,
    pub replay_ok: bool,
    pub breakpoint_ok: bool,
    pub log_ok: bool,
}

impl Default for WfDebug {
    fn default() -> Self {
        Self::new()
    }
}

impl WfDebug {
    pub fn new() -> Self {
        Self {
            trace_ok: true,
            inspect_ok: true,
            replay_ok: true,
            breakpoint_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.trace_ok && self.inspect_ok && self.replay_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.breakpoint_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.trace_ok || !self.inspect_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.trace_ok {
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
        let c = WfDebug::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WfDebug::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WfDebug::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WfDebug::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WfDebug::new();
        c.trace_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WfDebug::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
