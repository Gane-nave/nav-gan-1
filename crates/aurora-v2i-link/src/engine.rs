/// v2i link: infra, signal, priority, preempt, feedback
/// Phase 1123

#[derive(Debug, Clone)]
pub struct V2iLink {
    pub infra_ok: bool,
    pub signal_ok: bool,
    pub priority_ok: bool,
    pub preempt_ok: bool,
    pub feedback_ok: bool,
}

impl Default for V2iLink {
    fn default() -> Self {
        Self::new()
    }
}

impl V2iLink {
    pub fn new() -> Self {
        Self {
            infra_ok: true,
            signal_ok: true,
            priority_ok: true,
            preempt_ok: true,
            feedback_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.infra_ok && self.signal_ok && self.priority_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.preempt_ok && self.feedback_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.infra_ok || !self.signal_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.infra_ok {
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
        let c = V2iLink::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = V2iLink::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = V2iLink::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = V2iLink::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = V2iLink::new();
        c.infra_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = V2iLink::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
