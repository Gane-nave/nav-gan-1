/// io buf2: read, write, flush, capacity, log
/// Phase 1988

#[derive(Debug, Clone)]
pub struct IoBuf2 {
    pub read_ok: bool,
    pub write_ok: bool,
    pub flush_ok: bool,
    pub capacity_ok: bool,
    pub log_ok: bool,
}

impl Default for IoBuf2 {
    fn default() -> Self {
        Self::new()
    }
}

impl IoBuf2 {
    pub fn new() -> Self {
        Self {
            read_ok: true,
            write_ok: true,
            flush_ok: true,
            capacity_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.read_ok && self.write_ok && self.flush_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.capacity_ok && self.log_ok
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
        let c = IoBuf2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = IoBuf2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = IoBuf2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = IoBuf2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = IoBuf2::new();
        c.read_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = IoBuf2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
