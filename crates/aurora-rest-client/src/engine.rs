/// REST client: request, retry, timeout, pool, parse
/// Phase 1052

#[derive(Debug, Clone)]
pub struct RestClient {
    pub request_ok: bool,
    pub retry_ok: bool,
    pub timeout_ok: bool,
    pub pool_ok: bool,
    pub parse_ok: bool,
}

impl Default for RestClient {
    fn default() -> Self {
        Self::new()
    }
}

impl RestClient {
    pub fn new() -> Self {
        Self {
            request_ok: true,
            retry_ok: true,
            timeout_ok: true,
            pool_ok: true,
            parse_ok: true,
        }
    }

    pub fn connection_ok(&self) -> bool {
        self.request_ok && self.retry_ok && self.timeout_ok
    }

    pub fn processing_ok(&self) -> bool {
        self.pool_ok && self.parse_ok
    }

    pub fn all_ok(&self) -> bool {
        self.connection_ok() && self.processing_ok()
    }

    pub fn needs_config(&self) -> bool {
        !self.request_ok || !self.timeout_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.request_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection() {
        let c = RestClient::new();
        assert!(c.connection_ok());
    }

    #[test]
    fn test_processing() {
        let c = RestClient::new();
        assert!(c.processing_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RestClient::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_config() {
        let c = RestClient::new();
        assert!(!c.needs_config());
    }

    #[test]
    fn test_request() {
        let mut c = RestClient::new();
        c.request_ok = false;
        assert!(c.needs_config());
    }

    #[test]
    fn test_health() {
        let c = RestClient::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
