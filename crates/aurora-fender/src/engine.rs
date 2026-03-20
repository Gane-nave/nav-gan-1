/// Fender: panel alignment, gap, clearance, liner
/// Phase 552

#[derive(Debug, Clone)]
pub struct Fender {
    pub gap_mm: f64,
    pub target_gap_mm: f64,
    pub liner_ok: bool,
    pub aligned: bool,
    pub rust_free: bool,
}

impl Default for Fender {
    fn default() -> Self {
        Self::new()
    }
}

impl Fender {
    pub fn new() -> Self {
        Self {
            gap_mm: 4.0,
            target_gap_mm: 4.0,
            liner_ok: true,
            aligned: true,
            rust_free: true,
        }
    }

    pub fn gap_ok(&self) -> bool {
        (self.gap_mm - self.target_gap_mm).abs() < 2.0
    }

    pub fn panel_ok(&self) -> bool {
        self.aligned && self.rust_free
    }

    pub fn all_ok(&self) -> bool {
        self.gap_ok() && self.panel_ok() && self.liner_ok
    }

    pub fn needs_repair(&self) -> bool {
        !self.aligned || !self.rust_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.rust_free { return 20.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gap() {
        let c = Fender::new();
        assert!(c.gap_ok());
    }

    #[test]
    fn test_panel() {
        let c = Fender::new();
        assert!(c.panel_ok());
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
    fn test_rust() {
        let mut c = Fender::new();
        c.rust_free = false;
        assert!(c.needs_repair());
    }

    #[test]
    fn test_health() {
        let c = Fender::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
