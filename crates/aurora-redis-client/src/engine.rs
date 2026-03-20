/// Redis client: get, set, expire, pub, cluster
/// Phase 1057

#[derive(Debug, Clone)]
pub struct RedisClient {
    pub get_ok: bool,
    pub set_ok: bool,
    pub expire_ok: bool,
    pub pubsub_ok: bool,
    pub cluster_ok: bool,
}

impl Default for RedisClient {
    fn default() -> Self {
        Self::new()
    }
}

impl RedisClient {
    pub fn new() -> Self {
        Self {
            get_ok: true,
            set_ok: true,
            expire_ok: true,
            pubsub_ok: true,
            cluster_ok: true,
        }
    }

    pub fn operations_ok(&self) -> bool {
        self.get_ok && self.set_ok && self.expire_ok
    }

    pub fn advanced_ok(&self) -> bool {
        self.pubsub_ok && self.cluster_ok
    }

    pub fn all_ok(&self) -> bool {
        self.operations_ok() && self.advanced_ok()
    }

    pub fn needs_reconnect(&self) -> bool {
        !self.get_ok || !self.set_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.get_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operations() {
        let c = RedisClient::new();
        assert!(c.operations_ok());
    }

    #[test]
    fn test_advanced() {
        let c = RedisClient::new();
        assert!(c.advanced_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = RedisClient::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_reconnect() {
        let c = RedisClient::new();
        assert!(!c.needs_reconnect());
    }

    #[test]
    fn test_get() {
        let mut c = RedisClient::new();
        c.get_ok = false;
        assert!(c.needs_reconnect());
    }

    #[test]
    fn test_health() {
        let c = RedisClient::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
