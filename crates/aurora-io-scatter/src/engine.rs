/// io scatter: gather, scatter, vectored, flush, log
/// Phase 1995

#[derive(Debug, Clone)]
pub struct IoScatter {
    pub gather_ok: bool,
    pub scatter_ok: bool,
    pub vectored_ok: bool,
    pub flush_ok: bool,
    pub log_ok: bool,
}

impl Default for IoScatter {
    fn default() -> Self {
        Self::new()
    }
}

impl IoScatter {
    pub fn new() -> Self {
        Self {
            gather_ok: true,
            scatter_ok: true,
            vectored_ok: true,
            flush_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.gather_ok && self.scatter_ok && self.vectored_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.flush_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.gather_ok || !self.scatter_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.gather_ok {
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
        let c = IoScatter::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IoScatter::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IoScatter::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IoScatter::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IoScatter::new();
        c.gather_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IoScatter::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
