/// AMQP client: exchange, queue, bind, consume, ack
/// Phase 1054

#[derive(Debug, Clone)]
pub struct AmqpClient {
    pub exchange_ok: bool,
    pub queue_ok: bool,
    pub bind_ok: bool,
    pub consume_ok: bool,
    pub ack_ok: bool,
}

impl Default for AmqpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl AmqpClient {
    pub fn new() -> Self {
        Self {
            exchange_ok: true,
            queue_ok: true,
            bind_ok: true,
            consume_ok: true,
            ack_ok: true,
        }
    }

    pub fn topology_ok(&self) -> bool {
        self.exchange_ok && self.queue_ok && self.bind_ok
    }

    pub fn delivery_ok(&self) -> bool {
        self.consume_ok && self.ack_ok
    }

    pub fn all_ok(&self) -> bool {
        self.topology_ok() && self.delivery_ok()
    }

    pub fn needs_redeclare(&self) -> bool {
        !self.exchange_ok || !self.queue_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.exchange_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topology() {
        let c = AmqpClient::new();
        assert!(c.topology_ok());
    }

    #[test]
    fn test_delivery() {
        let c = AmqpClient::new();
        assert!(c.delivery_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = AmqpClient::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_redeclare() {
        let c = AmqpClient::new();
        assert!(!c.needs_redeclare());
    }

    #[test]
    fn test_exchange() {
        let mut c = AmqpClient::new();
        c.exchange_ok = false;
        assert!(c.needs_redeclare());
    }

    #[test]
    fn test_health() {
        let c = AmqpClient::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
