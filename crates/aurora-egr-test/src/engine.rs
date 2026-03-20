/// egr test: command, measure, flow, evaluate, log
/// Phase 1386

#[derive(Debug, Clone)]
pub struct EgrTest {
    pub command_ok: bool,
    pub measure_ok: bool,
    pub flow_ok: bool,
    pub evaluate_ok: bool,
    pub log_ok: bool,
}

impl Default for EgrTest {
    fn default() -> Self {
        Self::new()
    }
}

impl EgrTest {
    pub fn new() -> Self {
        Self {
            command_ok: true,
            measure_ok: true,
            flow_ok: true,
            evaluate_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.command_ok && self.measure_ok && self.flow_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.evaluate_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.command_ok || !self.measure_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.command_ok {
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
        let c = EgrTest::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EgrTest::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EgrTest::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EgrTest::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EgrTest::new();
        c.command_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EgrTest::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
