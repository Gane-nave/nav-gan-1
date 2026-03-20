/// wf signal: send, receive, timeout, cancel, log
/// Phase 2226

#[derive(Debug, Clone)]
pub struct WfSignal {
    pub send_ok: bool,
    pub receive_ok: bool,
    pub timeout_ok: bool,
    pub cancel_ok: bool,
    pub log_ok: bool,
}

impl Default for WfSignal {
    fn default() -> Self {
        Self::new()
    }
}

impl WfSignal {
    pub fn new() -> Self {
        Self {
            send_ok: true,
            receive_ok: true,
            timeout_ok: true,
            cancel_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.send_ok && self.receive_ok && self.timeout_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.cancel_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.send_ok || !self.receive_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.send_ok {
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
        let c = WfSignal::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = WfSignal::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = WfSignal::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = WfSignal::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = WfSignal::new();
        c.send_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = WfSignal::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
