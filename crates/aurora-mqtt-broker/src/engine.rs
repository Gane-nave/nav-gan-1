/// MQTT broker: connect, publish, subscribe, retain, will
/// Phase 1053

#[derive(Debug, Clone)]
pub struct MqttBroker {
    pub connect_ok: bool,
    pub publish_ok: bool,
    pub subscribe_ok: bool,
    pub retain_ok: bool,
    pub will_ok: bool,
}

impl Default for MqttBroker {
    fn default() -> Self {
        Self::new()
    }
}

impl MqttBroker {
    pub fn new() -> Self {
        Self {
            connect_ok: true,
            publish_ok: true,
            subscribe_ok: true,
            retain_ok: true,
            will_ok: true,
        }
    }

    pub fn messaging_ok(&self) -> bool {
        self.connect_ok && self.publish_ok && self.subscribe_ok
    }

    pub fn features_ok(&self) -> bool {
        self.retain_ok && self.will_ok
    }

    pub fn all_ok(&self) -> bool {
        self.messaging_ok() && self.features_ok()
    }

    pub fn needs_reconnect(&self) -> bool {
        !self.connect_ok || !self.publish_ok
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
    fn test_messaging() {
        let c = MqttBroker::new();
        assert!(c.messaging_ok());
    }

    #[test]
    fn test_features() {
        let c = MqttBroker::new();
        assert!(c.features_ok());
    }

    #[test]
    fn test_all_ok() {
        let c = MqttBroker::new();
        assert!(c.all_ok());
    }

    #[test]
    fn test_no_reconnect() {
        let c = MqttBroker::new();
        assert!(!c.needs_reconnect());
    }

    #[test]
    fn test_connect() {
        let mut c = MqttBroker::new();
        c.connect_ok = false;
        assert!(c.needs_reconnect());
    }

    #[test]
    fn test_health() {
        let c = MqttBroker::new();
        assert!((c.health_score() - 100.0).abs() < 0.1);
    }
}
