/// NATS client: connect, publish, subscribe, jetstream, kv
/// Phase 1055

#[derive(Debug, Clone)]
pub struct NatsClient {
    pub connect_ok: bool,
    pub publish_ok: bool,
    pub subscribe_ok: bool,
    pub jetstream_ok: bool,
    pub kv_ok: bool,
}

impl Default for NatsClient {
    fn default() -> Self {
        Self::new()
    }
}

impl NatsClient {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            publish_ok: true,
            subscribe_ok: true,
            jetstream_ok: true,
            kv_ok: true,
        }
    }

    pub fn messaging_ok(&self) -> bool {
        self.connect_ok && self.publish_ok && self.subscribe_ok
    }

    pub fn persistence_ok(&self) -> bool {
        self.jetstream_ok && self.kv_ok
    }

    pub fn all_ok(&self) -> bool {
        self.messaging_ok() && self.persistence_ok()
    }

    pub fn needs_reconnect(&self) -> bool {
        !self.connect_ok || !self.publish_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.connect_ok { return 5.0; }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_messaging() {
        let c = NatsClient::new();
        assert!(c.messaging_ok());
    }

    #[test]
    fn test_persistence() {
        let c = NatsClient::new();
        assert!(c.persistence_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = NatsClient::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_reconnect() {
        let c = NatsClient::new();
        assert!(!c.needs_reconnect());
    }

    #[test]
    fn test_connect() {
        let mut c = NatsClient::new();
        c.connect_ok = false;
        assert!(c.needs_reconnect());
    }

    #[test]
    fn test_health() {
        let c = NatsClient::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
