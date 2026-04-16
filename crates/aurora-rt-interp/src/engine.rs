/// rt interp: parse, evaluate, execute, reset, log
/// Phase 2344

#[derive(Debug, Clone)]
pub struct RtInterp {
    pub parse_ok: bool,
    pub evaluate_ok: bool,
    pub execute_ok: bool,
    pub reset_ok: bool,
    pub log_ok: bool,
}

impl Default for RtInterp {
    fn default() -> Self {
        Self::new()
    }
}

impl RtInterp {
    pub fn new() -> Self {
        Self {
            parse_ok: true,
            evaluate_ok: true,
            execute_ok: true,
            reset_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.parse_ok && self.evaluate_ok && self.execute_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.reset_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.parse_ok || !self.evaluate_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.parse_ok {
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
        let c = RtInterp::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = RtInterp::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RtInterp::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = RtInterp::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = RtInterp::new();
        c.parse_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = RtInterp::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
