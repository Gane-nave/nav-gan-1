/// net redis: connect, command, pipeline, subscribe, log
/// Phase 1544

#[derive(Debug, Clone)]
pub struct NetRedis {
    pub connect_ok: bool,
    pub command_ok: bool,
    pub pipeline_ok: bool,
    pub subscribe_ok: bool,
    pub log_ok: bool,
}

impl Default for NetRedis {
    fn default() -> Self {
        Self::new()
    }
}

impl NetRedis {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            command_ok: true,
            pipeline_ok: true,
            subscribe_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.command_ok && self.pipeline_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.subscribe_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.command_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok {
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
        let c = NetRedis::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetRedis::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetRedis::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetRedis::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetRedis::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetRedis::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
