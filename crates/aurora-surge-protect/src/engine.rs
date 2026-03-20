/// surge protect: detect, clamp, divert, recover, log
/// Phase 1371

#[derive(Debug, Clone)]
pub struct SurgeProtect {
    pub detect_ok: bool,
    pub clamp_ok: bool,
    pub divert_ok: bool,
    pub recover_ok: bool,
    pub log_ok: bool,
}

impl Default for SurgeProtect {
    fn default() -> Self {
        Self::new()
    }
}

impl SurgeProtect {
    pub fn new() -> Self {
        Self {
            detect_ok: true,
            clamp_ok: true,
            divert_ok: true,
            recover_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.detect_ok && self.clamp_ok && self.divert_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.recover_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.detect_ok || !self.clamp_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.detect_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary() {
        let c = SurgeProtect::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = SurgeProtect::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SurgeProtect::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = SurgeProtect::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = SurgeProtect::new();
        c.detect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = SurgeProtect::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
