/// mem track: allocate, free, report, leak, log
/// Phase 2379

#[derive(Debug, Clone)]
pub struct MemTrack {
    pub allocate_ok: bool,
    pub free_ok: bool,
    pub report_ok: bool,
    pub leak_ok: bool,
    pub log_ok: bool,
}

impl Default for MemTrack {
    fn default() -> Self {
        Self::new()
    }
}

impl MemTrack {
    pub fn new() -> Self {
        Self {
            allocate_ok: true,
            free_ok: true,
            report_ok: true,
            leak_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.allocate_ok && self.free_ok && self.report_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.leak_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.allocate_ok || !self.free_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.allocate_ok {
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
        let c = MemTrack::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = MemTrack::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MemTrack::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = MemTrack::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = MemTrack::new();
        c.allocate_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = MemTrack::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
