//! Topic — named channels for pub/sub messaging.

/// A topic identifier with hierarchical naming.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Topic {
    /// Topic name (e.g., "nav.position.update").
    name: String,
    /// Number of subscribers.
    subscriber_count: usize,
    /// Whether the topic is active.
    active: bool,
    /// Total messages published to this topic.
    total_published: u64,
}

impl Topic {
    /// Create a new topic.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            subscriber_count: 0,
            active: true,
            total_published: 0,
        }
    }

    /// Get the topic name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Whether the topic is active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Deactivate the topic.
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Activate the topic.
    pub fn activate(&mut self) {
        self.active = true;
    }

    /// Current subscriber count.
    pub fn subscriber_count(&self) -> usize {
        self.subscriber_count
    }

    /// Increment subscriber count.
    pub fn add_subscriber(&mut self) {
        self.subscriber_count += 1;
    }

    /// Decrement subscriber count.
    pub fn remove_subscriber(&mut self) {
        self.subscriber_count = self.subscriber_count.saturating_sub(1);
    }

    /// Record a published message.
    pub fn record_publish(&mut self) {
        self.total_published += 1;
    }

    /// Total messages published.
    pub fn total_published(&self) -> u64 {
        self.total_published
    }

    /// Whether the topic name matches a pattern (simple prefix matching).
    pub fn matches_pattern(&self, pattern: &str) -> bool {
        if let Some(prefix) = pattern.strip_suffix('*') {
            self.name.starts_with(prefix)
        } else {
            self.name == pattern
        }
    }

    /// Get the topic's namespace (everything before the last dot).
    pub fn namespace(&self) -> &str {
        match self.name.rfind('.') {
            Some(pos) => &self.name[..pos],
            None => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topic_creation() {
        let topic = Topic::new("nav.position");
        assert_eq!(topic.name(), "nav.position");
        assert!(topic.is_active());
        assert_eq!(topic.subscriber_count(), 0);
    }

    #[test]
    fn test_topic_activation() {
        let mut topic = Topic::new("test");
        assert!(topic.is_active());
        topic.deactivate();
        assert!(!topic.is_active());
        topic.activate();
        assert!(topic.is_active());
    }

    #[test]
    fn test_subscriber_count() {
        let mut topic = Topic::new("test");
        topic.add_subscriber();
        topic.add_subscriber();
        assert_eq!(topic.subscriber_count(), 2);
        topic.remove_subscriber();
        assert_eq!(topic.subscriber_count(), 1);
    }

    #[test]
    fn test_subscriber_underflow() {
        let mut topic = Topic::new("test");
        topic.remove_subscriber(); // should not underflow
        assert_eq!(topic.subscriber_count(), 0);
    }

    #[test]
    fn test_publish_counter() {
        let mut topic = Topic::new("test");
        topic.record_publish();
        topic.record_publish();
        assert_eq!(topic.total_published(), 2);
    }

    #[test]
    fn test_pattern_matching() {
        let topic = Topic::new("nav.position.update");
        assert!(topic.matches_pattern("nav.position.update"));
        assert!(topic.matches_pattern("nav.*"));
        assert!(topic.matches_pattern("nav.position.*"));
        assert!(!topic.matches_pattern("sensor.*"));
    }

    #[test]
    fn test_namespace() {
        let topic = Topic::new("nav.position.update");
        assert_eq!(topic.namespace(), "nav.position");

        let root_topic = Topic::new("events");
        assert_eq!(root_topic.namespace(), "");
    }

    #[test]
    fn test_exact_pattern() {
        let topic = Topic::new("nav.gps");
        assert!(topic.matches_pattern("nav.gps"));
        assert!(!topic.matches_pattern("nav.imu"));
    }
}
