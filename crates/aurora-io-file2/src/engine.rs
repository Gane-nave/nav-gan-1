/// io file2: open, read, write, close, log
/// Phase 1989

#[derive(Debug, Clone)]
pub struct IoFile2 {
    pub open_ok: bool,
    pub read_ok: bool,
    pub write_ok: bool,
    pub close_ok: bool,
    pub log_ok: bool,
}

impl Default for IoFile2 {
    fn default() -> Self {
        Self::new()
    }
}

impl IoFile2 {
    pub fn new() -> Self {
        Self {
            open_ok: true,
            read_ok: true,
            write_ok: true,
            close_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.open_ok && self.read_ok && self.write_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.close_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.open_ok || !self.read_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.open_ok {
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
        let c = IoFile2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IoFile2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IoFile2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IoFile2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IoFile2::new();
        c.open_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IoFile2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
