/// Brake line: hydraulic pressure, flex hose, corrosion
/// Phase 487

#[derive(Debug, Clone)]
pub struct BrakeLine {
    pub pressure_bar: f64,
    pub max_pressure_bar: f64,
    pub flex_ok: bool,
    pub corroded: bool,
    pub leak_free: bool,
}

impl Default for BrakeLine {
    fn default() -> Self {
        Self::new()
    }
}

impl BrakeLine {
    pub fn new() -> Self {
        Self {
            pressure_bar: 120.0,
            max_pressure_bar: 180.0,
            flex_ok: true,
            corroded: false,
            leak_free: true,
        }
    }

    pub fn pressure_ok(&self) -> bool {
        self.pressure_bar < self.max_pressure_bar
    }

    pub fn line_ok(&self) -> bool {
        self.flex_ok && !self.corroded && self.leak_free
    }

    pub fn all_ok(&self) -> bool {
        self.pressure_ok() && self.line_ok()
    }

    pub fn needs_replacement(&self) -> bool {
        self.corroded || !self.leak_free
    }

    pub fn health_score(&self) -> f64 {
        if !self.leak_free { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure() {
        let c = BrakeLine::new();
        assert!(c.pressure_ok());
    }

    #[test]
    fn test_line() {
        let c = BrakeLine::new();
        assert!(c.line_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = BrakeLine::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = BrakeLine::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_corroded() {
        let mut c = BrakeLine::new();
        c.corroded = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = BrakeLine::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
