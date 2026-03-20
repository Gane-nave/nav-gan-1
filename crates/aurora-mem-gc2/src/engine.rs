/// mem gc2: collect, mark, sweep, compact, log
/// Phase 1946

#[derive(Debug, Clone)]
pub struct MemGc2 {
    pub collect_ok: bool,
    pub mark_ok: bool,
    pub sweep_ok: bool,
    pub compact_ok: bool,
    pub log_ok: bool,
}

impl Default for MemGc2 {
    fn default() -> Self {
        Self::new()
    }
}

impl MemGc2 {
    pub fn new() -> Self {
        Self {
            collect_ok: true,
            mark_ok: true,
            sweep_ok: true,
            compact_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.collect_ok && self.mark_ok && self.sweep_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.compact_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.collect_ok || !self.mark_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.collect_ok {
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
        let c = MemGc2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MemGc2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MemGc2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MemGc2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MemGc2::new();
        c.collect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MemGc2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
