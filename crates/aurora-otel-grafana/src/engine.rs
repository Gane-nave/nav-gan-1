/// otel grafana: connect, push, query, alert, log
/// Phase 1767

#[derive(Debug, Clone)]
pub struct OtelGrafana {
    pub connect_ok: bool,
    pub push_ok: bool,
    pub query_ok: bool,
    pub alert_ok: bool,
    pub log_ok: bool,
}

impl Default for OtelGrafana {
    fn default() -> Self {
        Self::new()
    }
}

impl OtelGrafana {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            push_ok: true,
            query_ok: true,
            alert_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.connect_ok && self.push_ok && self.query_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.alert_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.connect_ok || !self.push_ok
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
        let c = OtelGrafana::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = OtelGrafana::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = OtelGrafana::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = OtelGrafana::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = OtelGrafana::new();
        c.connect_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = OtelGrafana::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
