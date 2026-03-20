/// Proportioning valve: bias, front-rear balance, adjustment
/// Phase 663

#[derive(Debug, Clone)]
pub struct PropValve {
    pub bias_ok: bool,
    pub balance_ok: bool,
    pub adjusted: bool,
    pub seal_ok: bool,
    pub flow_ok: bool,
}

impl Default for PropValve {
    fn default() -> Self {
        Self::new()
    }
}

impl PropValve {
    pub fn new() -> Self {
        Self {
            bias_ok: true,
            balance_ok: true,
            adjusted: true,
            seal_ok: true,
            flow_ok: true,
        }
    }

    pub fn regulation_ok(&self) -> bool {
        self.bias_ok && self.balance_ok
    }

    pub fn condition_ok(&self) -> bool {
        self.adjusted && self.seal_ok && self.flow_ok
    }

    pub fn all_ok(&self) -> bool {
        self.regulation_ok() && self.condition_ok()
    }

    pub fn needs_adjustment(&self) -> bool {
        !self.adjusted || !self.bias_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.bias_ok { return 15.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regulation() {
        let c = PropValve::new();
        assert!(c.regulation_ok());
    }

    #[test]
    fn test_condition() {
        let c = PropValve::new();
        assert!(c.condition_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PropValve::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_adjust() {
        let c = PropValve::new();
        assert!(!c.needs_adjustment());
    }

    #[test]
    fn test_bias() {
        let mut c = PropValve::new();
        c.bias_ok = false;
        assert!(c.needs_adjustment());
    }

    #[test]
    fn test_health() {
        let c = PropValve::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
