/// Valet mode: speed limit, geo-fence, audio lock, trunk lock
/// Phase 890

#[derive(Debug, Clone)]
pub struct ValetMode {
    pub speed_ok: bool,
    pub geo_ok: bool,
    pub audio_ok: bool,
    pub trunk_ok: bool,
    pub notify_ok: bool,
}

impl Default for ValetMode {
    fn default() -> Self {
        Self::new()
    }
}

impl ValetMode {
    pub fn new() -> Self {
        Self {
            speed_ok: true,
            geo_ok: true,
            audio_ok: true,
            trunk_ok: true,
            notify_ok: true,
        }
    }

    pub fn restrictions_ok(&self) -> bool {
        self.speed_ok && self.geo_ok && self.trunk_ok
    }

    pub fn monitoring_ok(&self) -> bool {
        self.audio_ok && self.notify_ok
    }

    pub fn all_ok(&self) -> bool {
        self.restrictions_ok() && self.monitoring_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.speed_ok || !self.geo_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.speed_ok {
            return 15.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restrictions() {
        let c = ValetMode::new();
        assert!(c.restrictions_ok());
    }

    #[test]
    fn test_monitoring() {
        let c = ValetMode::new();
        assert!(c.monitoring_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ValetMode::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = ValetMode::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_speed() {
        let mut c = ValetMode::new();
        c.speed_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = ValetMode::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
