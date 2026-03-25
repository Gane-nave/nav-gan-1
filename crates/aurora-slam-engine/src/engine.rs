/// SLAM engine: scan, match, map, localize, loop_close
/// Phase 1110

#[derive(Debug, Clone)]
pub struct SlamEngine {
    pub scan_ok: bool,
    pub match_ok: bool,
    pub map_ok: bool,
    pub localize_ok: bool,
    pub loop_close_ok: bool,
}

impl Default for SlamEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SlamEngine {
    pub fn new() -> Self {
        Self {
            scan_ok: true,
            match_ok: true,
            map_ok: true,
            localize_ok: true,
            loop_close_ok: true,
        }
    }

    pub fn mapping_ok(&self) -> bool {
        self.scan_ok && self.match_ok && self.map_ok
    }

    pub fn localization_ok(&self) -> bool {
        self.localize_ok && self.loop_close_ok
    }

    pub fn all_ok(&self) -> bool {
        self.mapping_ok() && self.localization_ok()
    }

    pub fn needs_reset(&self) -> bool {
        !self.scan_ok || !self.map_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.scan_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapping() {
        let c = SlamEngine::new();
        assert!(c.mapping_ok());
    }

    #[test]
    fn test_localization() {
        let c = SlamEngine::new();
        assert!(c.localization_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = SlamEngine::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_reset() {
        let c = SlamEngine::new();
        assert!(!c.needs_reset());
    }

    #[test]
    fn test_scan() {
        let mut c = SlamEngine::new();
        c.scan_ok = false;
        assert!(c.needs_reset());
    }

    #[test]
    fn test_health() {
        let c = SlamEngine::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
