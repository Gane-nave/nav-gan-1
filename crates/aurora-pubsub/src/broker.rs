//! Broker — message routing and subscriber management.

use crate::subscriber::Subscriber;
use crate::topic::Topic;
use std::collections::HashMap;

/// A message published to the broker.
#[derive(Debug, Clone)]
pub struct Message {
    /// Message ID.
    pub id: u64,
    /// Topic the message was published to.
    pub topic: String,
    /// Message payload.
    pub payload: Vec<u8>,
    /// Timestamp in milliseconds.
    pub timestamp_ms: u64,
}

/// The pub/sub message broker.
pub struct Broker {
    /// Registered topics.
    topics: HashMap<String, Topic>,
    /// Registered subscribers.
    subscribers: HashMap<u64, Subscriber>,
    /// Next message ID.
    next_message_id: u64,
    /// Next subscriber ID.
    next_subscriber_id: u64,
    /// Total messages published.
    total_published: u64,
    /// Total messages delivered.
    total_delivered: u64,
    /// Total messages dropped.
    total_dropped: u64,
}

impl Broker {
    /// Create a new broker.
    pub fn new() -> Self {
        Self {
            topics: HashMap::new(),
            subscribers: HashMap::new(),
            next_message_id: 1,
            next_subscriber_id: 1,
            total_published: 0,
            total_delivered: 0,
            total_dropped: 0,
        }
    }

    /// Register a topic.
    pub fn register_topic(&mut self, name: &str) -> bool {
        if self.topics.contains_key(name) {
            return false;
        }
        self.topics.insert(name.to_string(), Topic::new(name));
        true
    }

    /// Unregister a topic.
    pub fn unregister_topic(&mut self, name: &str) -> bool {
        self.topics.remove(name).is_some()
    }

    /// Get a topic by name.
    pub fn get_topic(&self, name: &str) -> Option<&Topic> {
        self.topics.get(name)
    }

    /// Register a subscriber and return its ID.
    pub fn add_subscriber(&mut self, name: &str) -> u64 {
        let id = self.next_subscriber_id;
        self.next_subscriber_id += 1;
        let subscriber = Subscriber::new(id, name);
        self.subscribers.insert(id, subscriber);
        id
    }

    /// Remove a subscriber by ID.
    pub fn remove_subscriber(&mut self, id: u64) -> bool {
        // Update topic subscriber counts
        if let Some(sub) = self.subscribers.get(&id) {
            let patterns: Vec<String> = sub.patterns().to_vec();
            for pattern in &patterns {
                for topic in self.topics.values_mut() {
                    if topic.matches_pattern(pattern) {
                        topic.remove_subscriber();
                    }
                }
            }
        }
        self.subscribers.remove(&id).is_some()
    }

    /// Get a mutable reference to a subscriber.
    pub fn subscriber_mut(&mut self, id: u64) -> Option<&mut Subscriber> {
        self.subscribers.get_mut(&id)
    }

    /// Subscribe a subscriber to a topic pattern.
    pub fn subscribe(&mut self, subscriber_id: u64, pattern: &str) -> bool {
        if let Some(sub) = self.subscribers.get_mut(&subscriber_id) {
            sub.subscribe(pattern);
            // Update topic subscriber counts
            for topic in self.topics.values_mut() {
                if topic.matches_pattern(pattern) {
                    topic.add_subscriber();
                }
            }
            true
        } else {
            false
        }
    }

    /// Publish a message to a topic. Returns the number of subscribers it was delivered to.
    pub fn publish(&mut self, topic_name: &str, _payload: Vec<u8>, _timestamp_ms: u64) -> usize {
        // Auto-register topic if not exists
        if !self.topics.contains_key(topic_name) {
            self.register_topic(topic_name);
        }

        if let Some(topic) = self.topics.get_mut(topic_name) {
            if !topic.is_active() {
                return 0;
            }
            topic.record_publish();
        }

        self.total_published += 1;
        let _msg_id = self.next_message_id;
        self.next_message_id += 1;

        let mut delivered = 0;

        // Find matching subscribers
        let matching_ids: Vec<u64> = self
            .subscribers
            .iter()
            .filter(|(_, sub)| sub.matches_topic(topic_name))
            .map(|(&id, _)| id)
            .collect();

        for id in matching_ids {
            if let Some(sub) = self.subscribers.get_mut(&id) {
                if sub.deliver() {
                    delivered += 1;
                    self.total_delivered += 1;
                } else {
                    self.total_dropped += 1;
                }
            }
        }

        delivered
    }

    /// Number of registered topics.
    pub fn topic_count(&self) -> usize {
        self.topics.len()
    }

    /// Number of registered subscribers.
    pub fn subscriber_count(&self) -> usize {
        self.subscribers.len()
    }

    /// Total messages published.
    pub fn total_published(&self) -> u64 {
        self.total_published
    }

    /// Total messages delivered.
    pub fn total_delivered(&self) -> u64 {
        self.total_delivered
    }

    /// Total messages dropped.
    pub fn total_dropped(&self) -> u64 {
        self.total_dropped
    }
}

impl Default for Broker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_broker_creation() {
        let broker = Broker::new();
        assert_eq!(broker.topic_count(), 0);
        assert_eq!(broker.subscriber_count(), 0);
    }

    #[test]
    fn test_register_topic() {
        let mut broker = Broker::new();
        assert!(broker.register_topic("nav.position"));
        assert!(!broker.register_topic("nav.position")); // duplicate
        assert_eq!(broker.topic_count(), 1);
    }

    #[test]
    fn test_add_subscriber() {
        let mut broker = Broker::new();
        let id = broker.add_subscriber("listener1");
        assert_eq!(id, 1);
        assert_eq!(broker.subscriber_count(), 1);
    }

    #[test]
    fn test_publish_and_deliver() {
        let mut broker = Broker::new();
        broker.register_topic("nav.position");
        let sub_id = broker.add_subscriber("listener");
        broker.subscribe(sub_id, "nav.*");

        let delivered = broker.publish("nav.position", vec![1, 2, 3], 1000);
        assert_eq!(delivered, 1);
        assert_eq!(broker.total_published(), 1);
        assert_eq!(broker.total_delivered(), 1);
    }

    #[test]
    fn test_publish_no_subscribers() {
        let mut broker = Broker::new();
        broker.register_topic("nav.position");
        let delivered = broker.publish("nav.position", vec![1], 1000);
        assert_eq!(delivered, 0);
    }

    #[test]
    fn test_publish_auto_registers_topic() {
        let mut broker = Broker::new();
        let sub_id = broker.add_subscriber("listener");
        broker.subscribe(sub_id, "auto.*");
        broker.publish("auto.topic", vec![1], 1000);
        assert!(broker.get_topic("auto.topic").is_some());
    }

    #[test]
    fn test_remove_subscriber() {
        let mut broker = Broker::new();
        let id = broker.add_subscriber("listener");
        assert!(broker.remove_subscriber(id));
        assert!(!broker.remove_subscriber(id)); // already removed
        assert_eq!(broker.subscriber_count(), 0);
    }

    #[test]
    fn test_multiple_subscribers() {
        let mut broker = Broker::new();
        broker.register_topic("events");
        let s1 = broker.add_subscriber("sub1");
        let s2 = broker.add_subscriber("sub2");
        broker.subscribe(s1, "events");
        broker.subscribe(s2, "events");

        let delivered = broker.publish("events", vec![1], 1000);
        assert_eq!(delivered, 2);
    }

    #[test]
    fn test_inactive_topic() {
        let mut broker = Broker::new();
        broker.register_topic("nav.position");
        if let Some(topic) = broker.topics.get_mut("nav.position") {
            topic.deactivate();
        }
        let delivered = broker.publish("nav.position", vec![1], 1000);
        assert_eq!(delivered, 0);
    }

    #[test]
    fn test_broker_stats() {
        let mut broker = Broker::new();
        let sub_id = broker.add_subscriber("listener");
        broker.subscribe(sub_id, "test.*");

        broker.publish("test.a", vec![1], 1000);
        broker.publish("test.b", vec![2], 2000);

        assert_eq!(broker.total_published(), 2);
        assert_eq!(broker.total_delivered(), 2);
        assert_eq!(broker.total_dropped(), 0);
    }
}
