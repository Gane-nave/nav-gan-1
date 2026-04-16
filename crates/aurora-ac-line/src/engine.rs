/// AC refrigerant line: high side, low side, fitting
/// Phase 630

#[derive(Debug, Clone)]
pub struct AcLine {
    pub high_side_ok: bool,
    pub low_side_ok: bool,
    pub fitting_ok: bool,
    pub insulation_ok: bool,
    pub leak_free: bool,
}

impl Default for AcLine {
    fn default() -> Self {
        Self::new()
    }
}

impl AcLine {
    pub fn new() -> Self {
        Self {
            high_side_ok: true,
            low_side_ok: true,
            fitting_ok: true,
            insulation_ok: true,
            leak_free: true,
        }
    }

    pub fn lines_ok(&self) -> bool {
        self.high_side_ok && self.low_side_ok
    }

    pub fn connections_ok(&self) -> bool {
        self.fitting_ok && self.leak_free
    }

    pub fn all_ok(&self) -> bool {
        self.lines_ok() && self.connections_ok() && self.insulation_ok
    }

    pub fn needs_service(&self) -> bool {
        !self.leak_free || !self.fitting_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.leak_free {
            return 10.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lines() {
        let c = AcLine::new();
        assert!(c.lines_ok());
    }

    #[test]
    fn test_connections() {
        let c = AcLine::new();
        assert!(c.connections_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AcLine::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_service() {
        let c = AcLine::new();
        assert!(!c.needs_service());
    }

    #[test]
    fn test_leak() {
        let mut c = AcLine::new();
        c.leak_free = false;
        assert!(c.needs_service());
    }

    #[test]
    fn test_health() {
        let c = AcLine::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
