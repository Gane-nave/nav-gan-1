/// Spark plug: gap, electrode wear, fouling
/// Phase 519

#[derive(Debug, Clone)]
pub struct SparkPlug {
    pub gap_mm: f64,
    pub target_gap_mm: f64,
    pub electrode_ok: bool,
    pub fouled: bool,
    pub cracked: bool,
}

impl Default for SparkPlug {
    fn default() -> Self {
        Self::new()
    }
}

impl SparkPlug {
    pub fn new() -> Self {
        Self {
            gap_mm: 0.9,
            target_gap_mm: 0.9,
            electrode_ok: true,
            fouled: false,
            cracked: false,
        }
    }

    pub fn gap_ok(&self) -> bool {
        (self.gap_mm - self.target_gap_mm).abs() < 0.2
    }

    pub fn is_clean(&self) -> bool {
        !self.fouled
    }

    pub fn all_ok(&self) -> bool {
        self.gap_ok() && self.is_clean() && self.electrode_ok && !self.cracked
    }

    pub fn needs_replacement(&self) -> bool {
        self.cracked || self.fouled
    }

    pub fn health_score(&self) -> f64 {
        if self.cracked { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gap() {
        let c = SparkPlug::new();
        assert!(c.gap_ok());
    }

    #[test]
    fn test_clean() {
        let c = SparkPlug::new();
        assert!(c.is_clean());
    }

    #[test]
    fn test_all_ok() {
        let c = SparkPlug::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_replace() {
        let c = SparkPlug::new();
        assert!(!c.needs_replacement());
    }

    #[test]
    fn test_cracked() {
        let mut c = SparkPlug::new();
        c.cracked = true;
        assert!(c.needs_replacement());
    }

    #[test]
    fn test_health() {
        let c = SparkPlug::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
