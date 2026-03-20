/// io async2: read, write, flush, close, log
/// Phase 1986

#[derive(Debug, Clone)]
pub struct IoAsync2 {
    pub read_ok: bool,
    pub write_ok: bool,
    pub flush_ok: bool,
    pub close_ok: bool,
    pub log_ok: bool,
}

impl Default for IoAsync2 {
    fn default() -> Self {
        Self::new()
    }
}

impl IoAsync2 {
    pub fn new() -> Self {
        Self {
            read_ok: true,
            write_ok: true,
            flush_ok: true,
            close_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.read_ok && self.write_ok && self.flush_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.close_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.read_ok || !self.write_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.read_ok {
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
        let c = IoAsync2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IoAsync2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IoAsync2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IoAsync2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IoAsync2::new();
        c.read_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IoAsync2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
