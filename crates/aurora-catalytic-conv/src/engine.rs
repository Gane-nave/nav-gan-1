/// catalytic conv: oxidize, reduce, heat, monitor, check
/// Phase 1239

#[derive(Debug, Clone)]
pub struct CatalyticConv {
    pub oxidize_ok: bool,
    pub reduce_ok: bool,
    pub heat_ok: bool,
    pub monitor_ok: bool,
    pub check_ok: bool,
}

impl Default for CatalyticConv {
    fn default() -> Self {
        Self::new()
    }
}

impl CatalyticConv {
    pub fn new() -> Self {
        Self {
            oxidize_ok: true,
            reduce_ok: true,
            heat_ok: true,
            monitor_ok: true,
            check_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.oxidize_ok && self.reduce_ok && self.heat_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.monitor_ok && self.check_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.oxidize_ok || !self.reduce_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.oxidize_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = CatalyticConv::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = CatalyticConv::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = CatalyticConv::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = CatalyticConv::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = CatalyticConv::new();
        c.oxidize_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = CatalyticConv::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
