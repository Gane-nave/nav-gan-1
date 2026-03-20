/// Fender: panel, liner, bracket, clearance
/// Phase 786

#[derive(Debug, Clone)]
pub struct Fender {
    pub panel_ok: bool,
    pub liner_ok: bool,
    pub bracket_ok: bool,
    pub clearance_ok: bool,
    pub alignment_ok: bool,
}

impl Default for Fender {
    fn default() -> Self {
        Self::new()
    }
}

impl Fender {
    pub fn new() -> Self {
        Self {
            panel_ok: true,
            liner_ok: true,
            bracket_ok: true,
            clearance_ok: true,
            alignment_ok: true,
        }
    }

    pub fn body_ok(&self) -> bool {
        self.panel_ok && self.alignment_ok
    }

    pub fn protection_ok(&self) -> bool {
        self.liner_ok && self.bracket_ok && self.clearance_ok
    }

    pub fn all_ok(&self) -> bool {
        self.body_ok() && self.protection_ok()
    }

    pub fn needs_repair(&self) -> bool {
        !self.panel_ok || !self.bracket_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.panel_ok { return 10.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body() {
        let c = Fender::new();
        assert!(c.body_ok());
    }

    #[test]
    fn test_protection() {
        let c = Fender::new();
        assert!(c.protection_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = Fender::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_repair() {
        let c = Fender::new();
        assert!(!c.needs_repair());
    }

    #[test]
    fn test_panel() {
        let mut c = Fender::new();
        c.panel_ok = false;
        assert!(c.needs_repair());
    }

    #[test]
    fn test_health() {
        let c = Fender::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
