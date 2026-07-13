//! Subscriber — manages subscription state and message delivery.

/// Delivery mode for subscribers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryMode {
    /// Best-effort delivery — messages may be dropped.
    BestEffort,
    /// At-least-once delivery — messages may be duplicated.
    AtLeastOnce,
    /// At-most-once delivery — messages are never duplicated.
    AtMostOnce,
}

impl DeliveryMode {
    /// Display name.
    pub fn as_str(&self) -> &'static str {
        match self {
            DeliveryMode::BestEffort => "best_effort",
            DeliveryMode::AtLeastOnce => "at_least_once",
            DeliveryMode::AtMostOnce => "at_most_once",
        }
    }
}

/// A subscriber to one or more topics.
#[derive(Debug, Clone)]
pub struct Subscriber {
    /// Unique subscriber ID.
    id: u64,
    /// Subscriber name.
    name: String,
    /// Topic patterns this subscriber listens to.
    patterns: Vec<String>,
    /// Delivery mode.
    delivery_mode: DeliveryMode,
    /// Whether the subscriber is active.
    active: bool,
    /// Messages received count.
    messages_received: u64,
    /// Messages dropped count.
    messages_dropped: u64,
    /// Maximum pending messages before dropping.
    max_pending: usize,
    /// Current pending message count.
    pending_count: usize,
}

impl Subscriber {
    /// Create a new subscriber.
    pub fn new(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            patterns: Vec::new(),
            delivery_mode: DeliveryMode::BestEffort,
            active: true,
            messages_received: 0,
            messages_dropped: 0,
            max_pending: 1000,
            pending_count: 0,
        }
    }

    /// Get the subscriber ID.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get the subscriber name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set delivery mode.
    pub fn with_delivery_mode(mut self, mode: DeliveryMode) -> Self {
        self.delivery_mode = mode;
        self
    }

    /// Set max pending messages.
    pub fn with_max_pending(mut self, max: usize) -> Self {
        self.max_pending = max;
        self
    }

    /// Subscribe to a topic pattern.
    pub fn subscribe(&mut self, pattern: &str) {
        if !self.patterns.contains(&pattern.to_string()) {
            self.patterns.push(pattern.to_string());
        }
    }

    /// Unsubscribe from a topic pattern.
    pub fn unsubscribe(&mut self, pattern: &str) -> bool {
        if let Some(pos) = self.patterns.iter().position(|p| p == pattern) {
            self.patterns.remove(pos);
            true
        } else {
            false
        }
    }

    /// Get all subscribed patterns.
    pub fn patterns(&self) -> &[String] {
        &self.patterns
    }

    /// Whether the subscriber matches a given topic name.
    pub fn matches_topic(&self, topic_name: &str) -> bool {
        self.patterns.iter().any(|p| {
            if let Some(prefix) = p.strip_suffix('*') {
                topic_name.starts_with(prefix)
            } else {
                p == topic_name
            }
        })
    }

    /// Try to deliver a message. Returns false if subscriber is full or inactive.
    pub fn deliver(&mut self) -> bool {
        if !self.active {
            self.messages_dropped += 1;
            return false;
        }
        if self.pending_count >= self.max_pending {
            self.messages_dropped += 1;
            return false;
        }
        self.pending_count += 1;
        self.messages_received += 1;
        true
    }

    /// Acknowledge processing of a message.
    pub fn acknowledge(&mut self) {
        self.pending_count = self.pending_count.saturating_sub(1);
    }

    /// Deactivate the subscriber.
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Whether the subscriber is active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Delivery mode.
    pub fn delivery_mode(&self) -> DeliveryMode {
        self.delivery_mode
    }

    /// Messages received count.
    pub fn messages_received(&self) -> u64 {
        self.messages_received
    }

    /// Messages dropped count.
    pub fn messages_dropped(&self) -> u64 {
        self.messages_dropped
    }

    /// Current pending count.
    pub fn pending_count(&self) -> usize {
        self.pending_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscriber_creation() {
        let sub = Subscriber::new(1, "position-listener");
        assert_eq!(sub.id(), 1);
        assert_eq!(sub.name(), "position-listener");
        assert!(sub.is_active());
        assert_eq!(sub.messages_received(), 0);
    }

    #[test]
    fn test_subscribe_and_match() {
        let mut sub = Subscriber::new(1, "nav");
        sub.subscribe("nav.position.*");
        sub.subscribe("nav.route");

        assert!(sub.matches_topic("nav.position.update"));
        assert!(sub.matches_topic("nav.route"));
        assert!(!sub.matches_topic("sensor.imu"));
    }

    #[test]
    fn test_unsubscribe() {
        let mut sub = Subscriber::new(1, "nav");
        sub.subscribe("nav.*");
        assert!(sub.unsubscribe("nav.*"));
        assert!(!sub.unsubscribe("nav.*")); // already removed
        assert!(!sub.matches_topic("nav.position"));
    }

    #[test]
    fn test_deliver_and_acknowledge() {
        let mut sub = Subscriber::new(1, "test").with_max_pending(3);
        assert!(sub.deliver());
        assert!(sub.deliver());
        assert!(sub.deliver());
        assert!(!sub.deliver()); // full
        assert_eq!(sub.messages_dropped(), 1);

        sub.acknowledge();
        assert!(sub.deliver()); // space available again
        assert_eq!(sub.pending_count(), 3);
    }

    #[test]
    fn test_deliver_inactive() {
        let mut sub = Subscriber::new(1, "test");
        sub.deactivate();
        assert!(!sub.deliver());
        assert_eq!(sub.messages_dropped(), 1);
    }

    #[test]
    fn test_delivery_mode() {
        let sub = Subscriber::new(1, "test").with_delivery_mode(DeliveryMode::AtLeastOnce);
        assert_eq!(sub.delivery_mode(), DeliveryMode::AtLeastOnce);
    }

    #[test]
    fn test_no_duplicate_subscribe() {
        let mut sub = Subscriber::new(1, "test");
        sub.subscribe("nav.*");
        sub.subscribe("nav.*");
        assert_eq!(sub.patterns().len(), 1);
    }

    #[test]
    fn test_delivery_mode_display() {
        assert_eq!(DeliveryMode::BestEffort.as_str(), "best_effort");
        assert_eq!(DeliveryMode::AtLeastOnce.as_str(), "at_least_once");
        assert_eq!(DeliveryMode::AtMostOnce.as_str(), "at_most_once");
    }

    #[test]
    fn test_acknowledge_underflow() {
        let mut sub = Subscriber::new(1, "test");
        sub.acknowledge(); // should not underflow
        assert_eq!(sub.pending_count(), 0);
    }
}
