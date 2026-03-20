/// proto redis2: command, pipeline, subscribe, publish, log
/// Phase 2009

#[derive(Debug, Clone)]
pub struct ProtoRedis2 {
    pub command_ok: bool,
    pub pipeline_ok: bool,
    pub subscribe_ok: bool,
    pub publish_ok: bool,
    pub log_ok: bool,
}

impl Default for ProtoRedis2 {
    fn default() -> Self {
        Self::new()
    }
}

impl ProtoRedis2 {
    pub fn new() -> Self {
        Self {
            command_ok: true,
            pipeline_ok: true,
            subscribe_ok: true,
            publish_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.command_ok && self.pipeline_ok && self.subscribe_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.publish_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.command_ok || !self.pipeline_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.command_ok {
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
        let c = ProtoRedis2::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = ProtoRedis2::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = ProtoRedis2::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = ProtoRedis2::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = ProtoRedis2::new();
        c.command_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = ProtoRedis2::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
