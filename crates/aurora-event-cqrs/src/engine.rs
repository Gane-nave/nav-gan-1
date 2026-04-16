/// event cqrs: command, query, project, replay, log
/// Phase 1864

#[derive(Debug, Clone)]
pub struct EventCqrs {
    pub command_ok: bool,
    pub query_ok: bool,
    pub project_ok: bool,
    pub replay_ok: bool,
    pub log_ok: bool,
}

impl Default for EventCqrs {
    fn default() -> Self {
        Self::new()
    }
}

impl EventCqrs {
    pub fn new() -> Self {
        Self {
            command_ok: true,
            query_ok: true,
            project_ok: true,
            replay_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.command_ok && self.query_ok && self.project_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.replay_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.command_ok || !self.query_ok
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
        let c = EventCqrs::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = EventCqrs::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = EventCqrs::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = EventCqrs::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = EventCqrs::new();
        c.command_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = EventCqrs::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
