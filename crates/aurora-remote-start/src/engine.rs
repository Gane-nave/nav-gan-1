/// Remote start: engine pre-start, climate pre-conditioning, security check
/// Phase 284

#[derive(Debug, Clone)]
pub struct RemoteStart {
    pub engine_running: bool,
    pub remote_authorized: bool,
    pub timer_minutes: f64,
    pub max_runtime_min: f64,
    pub security_ok: bool,
    pub doors_locked: bool,
}

impl Default for RemoteStart {
    fn default() -> Self {
        Self::new()
    }
}

impl RemoteStart {
    pub fn new() -> Self {
        Self {
            engine_running: false,
            remote_authorized: false,
            timer_minutes: 0.0,
            max_runtime_min: 15.0,
            security_ok: true,
            doors_locked: true,
        }
    }

    pub fn can_start(&self) -> bool {
        self.remote_authorized && self.security_ok && self.doors_locked
    }

    pub fn time_remaining(&self) -> f64 {
        (self.max_runtime_min - self.timer_minutes).max(0.0)
    }

    pub fn should_stop(&self) -> bool {
        self.engine_running && self.timer_minutes >= self.max_runtime_min
    }

    pub fn is_running(&self) -> bool {
        self.engine_running
    }

    pub fn health_score(&self) -> f64 {
        if !self.security_ok {
            return 0.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cannot_start() {
        let r = RemoteStart::new();
        assert!(!r.can_start());
    }

    #[test]
    fn test_time_remaining() {
        let r = RemoteStart::new();
        assert!((r.time_remaining() - 15.0).abs() < 0.1);
    }

    #[test]
    fn test_not_running() {
        let r = RemoteStart::new();
        assert!(!r.is_running());
    }

    #[test]
    fn test_no_stop() {
        let r = RemoteStart::new();
        assert!(!r.should_stop());
    }

    #[test]
    fn test_authorized() {
        let mut r = RemoteStart::new();
        r.remote_authorized = true;
        assert!(r.can_start());
    }

    #[test]
    fn test_health() {
        let r = RemoteStart::new();
        assert!((r.health_score() - 100.0).abs() < 0.1);
    }
}
