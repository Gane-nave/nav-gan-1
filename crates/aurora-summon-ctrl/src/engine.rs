/// Summon control: locate, navigate, arrive, notify, cancel
/// Phase 1120

#[derive(Debug, Clone)]
pub struct SummonCtrl {
    pub locate_ok: bool,
    pub navigate_ok: bool,
    pub arrive_ok: bool,
    pub notify_ok: bool,
    pub cancel_ok: bool,
}

impl Default for SummonCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl SummonCtrl {
    pub fn new() -> Self {
        Self {
            locate_ok: true,
            navigate_ok: true,
            arrive_ok: true,
            notify_ok: true,
            cancel_ok: true,
        }
    }

    pub fn operation_ok(&self) -> bool {
        self.locate_ok && self.navigate_ok && self.arrive_ok
    }

    pub fn communication_ok(&self) -> bool {
        self.notify_ok && self.cancel_ok
    }

    pub fn all_ok(&self) -> bool {
        self.operation_ok() && self.communication_ok()
    }

    pub fn needs_retry(&self) -> bool {
        !self.locate_ok || !self.navigate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.locate_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation() {
        let c = SummonCtrl::new();
        assert!(c.operation_ok());
    }

    #[test]
    fn test_communication() {
        let c = SummonCtrl::new();
        assert!(c.communication_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SummonCtrl::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_retry() {
        let c = SummonCtrl::new();
        assert!(!c.needs_retry());
    }

    #[test]
    fn test_locate() {
        let mut c = SummonCtrl::new();
        c.locate_ok = false;
        assert!(c.needs_retry());
    }

    #[test]
    fn test_health() {
        let c = SummonCtrl::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
