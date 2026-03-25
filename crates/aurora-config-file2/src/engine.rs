/// config file2: read, write, watch, merge, log
/// Phase 1771

#[derive(Debug, Clone)]
pub struct ConfigFile2 {
    pub read_ok: bool,
    pub write_ok: bool,
    pub watch_ok: bool,
    pub merge_ok: bool,
    pub log_ok: bool,
}

impl Default for ConfigFile2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigFile2 {
    pub fn new() -> Self {
        Self {
            read_ok: true,
            write_ok: true,
            watch_ok: true,
            merge_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.read_ok && self.write_ok && self.watch_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.merge_ok && self.log_ok
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
        let c = ConfigFile2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ConfigFile2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ConfigFile2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ConfigFile2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ConfigFile2::new();
        c.read_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ConfigFile2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
