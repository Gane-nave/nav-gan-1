/// Mirror defogger: heated mirror, anti-fog element, auto activation
/// Phase 455

#[derive(Debug, Clone)]
pub struct MirrorDefog {
    pub left_ok: bool,
    pub right_ok: bool,
    pub auto_on: bool,
    pub element_ok: bool,
    pub temp_c: f64,
}

impl Default for MirrorDefog {
    fn default() -> Self {
        Self::new()
    }
}

impl MirrorDefog {
    pub fn new() -> Self {
        Self {
            left_ok: true,
            right_ok: true,
            auto_on: true,
            element_ok: true,
            temp_c: 30.0,
        }
    }

    pub fn all_ok(&self) -> bool {
        self.left_ok && self.right_ok && self.element_ok
    }

    pub fn functional(&self) -> bool {
        self.left_ok || self.right_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.left_ok && !self.right_ok
    }

    pub fn clear(&self) -> bool {
        self.temp_c > 25.0 && self.element_ok
    }

    pub fn health_score(&self) -> f64 {
        if self.needs_service() {
            return 0.0;
        }
        if !self.all_ok() {
            return 50.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ok() {
        let m = MirrorDefog::new();
        assert!(m.all_ok());
    }

    #[test]
    fn test_functional() {
        let m = MirrorDefog::new();
        assert!(m.functional());
    }

    #[test]
    fn test_no_service() {
        let m = MirrorDefog::new();
        assert!(!m.needs_service());
    }

    #[test]
    fn test_clear() {
        let m = MirrorDefog::new();
        assert!(m.clear());
    }

    #[test]
    fn test_both_out() {
        let mut m = MirrorDefog::new();
        m.left_ok = false;
        m.right_ok = false;
        assert!(m.needs_service());
    }

    #[test]
    fn test_health() {
        let m = MirrorDefog::new();
        assert!((m.health_score() - 100.0).abs() < 0.1);
    }
}
