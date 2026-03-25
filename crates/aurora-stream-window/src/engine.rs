/// stream window: tumbling, sliding, session, count, log
/// Phase 1925

#[derive(Debug, Clone)]
pub struct StreamWindow {
    pub tumbling_ok: bool,
    pub sliding_ok: bool,
    pub session_ok: bool,
    pub count_ok: bool,
    pub log_ok: bool,
}

impl Default for StreamWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamWindow {
    pub fn new() -> Self {
        Self {
            tumbling_ok: true,
            sliding_ok: true,
            session_ok: true,
            count_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.tumbling_ok && self.sliding_ok && self.session_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.count_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.tumbling_ok || !self.sliding_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.tumbling_ok {
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
        let c = StreamWindow::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = StreamWindow::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = StreamWindow::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = StreamWindow::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = StreamWindow::new();
        c.tumbling_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = StreamWindow::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
