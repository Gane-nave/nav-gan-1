/// Rivet monitoring: self-piercing, blind rivet, pull strength, head integrity
/// Phase 397

#[derive(Debug, Clone)]
pub struct RivetMon {
    pub pull_kn: f64,
    pub min_pull_kn: f64,
    pub head_ok: bool,
    pub flush: bool,
    pub count: u32,
}

impl Default for RivetMon {
    fn default() -> Self {
        Self::new()
    }
}

impl RivetMon {
    pub fn new() -> Self {
        Self {
            pull_kn: 5.0,
            min_pull_kn: 3.0,
            head_ok: true,
            flush: true,
            count: 200,
        }
    }

    pub fn strength_ok(&self) -> bool {
        self.pull_kn >= self.min_pull_kn
    }

    pub fn all_ok(&self) -> bool {
        self.strength_ok() && self.head_ok && self.flush
    }

    pub fn needs_replacement(&self) -> bool {
        !self.head_ok || !self.strength_ok()
    }

    pub fn margin_pct(&self) -> f64 {
        if self.min_pull_kn <= 0.0 {
            return 0.0;
        }
        ((self.pull_kn / self.min_pull_kn - 1.0) * 100.0).max(0.0)
    }

    pub fn health_score(&self) -> f64 {
        if !self.head_ok {
            return 0.0;
        }
        if !self.strength_ok() {
            return 30.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strength() {
        let r = RivetMon::new();
        assert!(r.strength_ok());
    }

    #[test]
    fn test_all_ok() {
        let r = RivetMon::new();
        assert!(r.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let r = RivetMon::new();
        assert!(!r.needs_replacement());
    }

    #[test]
    fn test_margin() {
        let r = RivetMon::new();
        assert!(r.margin_pct() > 50.0);
    }

    #[test]
    fn test_bad_head() {
        let mut r = RivetMon::new();
        r.head_ok = false;
        assert!(r.needs_replacement());
    }

    #[test]
    fn test_health() {
        let r = RivetMon::new();
        assert!((r.health_score() - 100.0).abs() < 0.1);
    }
}
