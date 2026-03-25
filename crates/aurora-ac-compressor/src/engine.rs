/// ac compressor: engage, compress, cycle, oil, check
/// Phase 1265

#[derive(Debug, Clone)]
pub struct AcCompressor {
    pub engage_ok: bool,
    pub compress_ok: bool,
    pub cycle_ok: bool,
    pub oil_ok: bool,
    pub check_ok: bool,
}

impl Default for AcCompressor {
    fn default() -> Self {
        Self::new()
    }
}

impl AcCompressor {
    pub fn new() -> Self {
        Self {
            engage_ok: true,
            compress_ok: true,
            cycle_ok: true,
            oil_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.engage_ok && self.compress_ok && self.cycle_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.oil_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.engage_ok || !self.compress_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.engage_ok {
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
        let c = AcCompressor::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = AcCompressor::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AcCompressor::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = AcCompressor::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = AcCompressor::new();
        c.engage_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = AcCompressor::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
