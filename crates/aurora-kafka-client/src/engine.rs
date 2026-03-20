/// Kafka client: produce, consume, partition, offset, group
/// Phase 1056

#[derive(Debug, Clone)]
pub struct KafkaClient {
    pub produce_ok: bool,
    pub consume_ok: bool,
    pub partition_ok: bool,
    pub offset_ok: bool,
    pub group_ok: bool,
}

impl Default for KafkaClient {
    fn default() -> Self {
        Self::new()
    }
}

impl KafkaClient {
    pub fn new() -> Self {
        Self {
            produce_ok: true,
            consume_ok: true,
            partition_ok: true,
            offset_ok: true,
            group_ok: true,
        }
    }

    pub fn streaming_ok(&self) -> bool {
        self.produce_ok && self.consume_ok && self.partition_ok
    }

    pub fn management_ok(&self) -> bool {
        self.offset_ok && self.group_ok
    }

    pub fn all_ok(&self) -> bool {
        self.streaming_ok() && self.management_ok()
    }

    pub fn needs_rebalance(&self) -> bool {
        !self.partition_ok || !self.group_ok
    }

    pub fn health_score(&self) -> f64 {
        if !self.produce_ok {
            return 5.0;
        }
        100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streaming() {
        let c = KafkaClient::new();
        assert!(c.streaming_ok());
    }

    #[test]
    fn test_management() {
        let c = KafkaClient::new();
        assert!(c.management_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = KafkaClient::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_rebalance() {
        let c = KafkaClient::new();
        assert!(!c.needs_rebalance());
    }

    #[test]
    fn test_partition() {
        let mut c = KafkaClient::new();
        c.partition_ok = false;
        assert!(c.needs_rebalance());
    }

    #[test]
    fn test_health() {
        let c = KafkaClient::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
