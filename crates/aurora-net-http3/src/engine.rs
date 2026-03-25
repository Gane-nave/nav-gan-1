/// net http3: request, response, stream, push, log
/// Phase 2260

#[derive(Debug, Clone)]
pub struct NetHttp3 {
    pub request_ok: bool,
    pub response_ok: bool,
    pub stream_ok: bool,
    pub push_ok: bool,
    pub log_ok: bool,
}

impl Default for NetHttp3 {
    fn default() -> Self {
        Self::new()
    }
}

impl NetHttp3 {
    pub fn new() -> Self {
        Self {
            request_ok: true,
            response_ok: true,
            stream_ok: true,
            push_ok: true,
            log_ok: true,
        }
    }

    pub fn primary_ok(&self) -> bool {
        self.request_ok && self.response_ok && self.stream_ok
    }

    pub fn secondary_ok(&self) -> bool {
        self.push_ok && self.log_ok
    }

    pub fn all_ok(&self) -> bool {
        self.primary_ok() && self.secondary_ok()
    }

    pub fn needs_attention(&self) -> bool {
        !self.request_ok || !self.response_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.request_ok {
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
        let c = NetHttp3::new();
        assert!(c.primary_ok());
    }

    #[test]
    fn test_secondary() {
        let c = NetHttp3::new();
        assert!(c.secondary_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NetHttp3::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_attention() {
        let c = NetHttp3::new();
        assert!(!c.needs_attention());
    }

    #[test]
    fn test_field_toggle() {
        let mut c = NetHttp3::new();
        c.request_ok = false;
        assert!(c.needs_attention());
    }

    #[test]
    fn test_health() {
        let c = NetHttp3::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
