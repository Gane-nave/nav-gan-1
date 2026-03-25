/// platform docker3: build, run, stop, inspect, log
/// Phase 1814

#[derive(Debug, Clone)]
pub struct PlatformDocker3 {
    pub build_ok: bool,
    pub run_ok: bool,
    pub stop_ok: bool,
    pub inspect_ok: bool,
    pub log_ok: bool,
}

impl Default for PlatformDocker3 {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformDocker3 {
    pub fn new() -> Self {
        Self {
            build_ok: true,
            run_ok: true,
            stop_ok: true,
            inspect_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.build_ok && self.run_ok && self.stop_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.inspect_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.build_ok || !self.run_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.build_ok {
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
        let c = PlatformDocker3::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = PlatformDocker3::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = PlatformDocker3::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = PlatformDocker3::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = PlatformDocker3::new();
        c.build_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = PlatformDocker3::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
