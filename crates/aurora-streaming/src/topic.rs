//! Topic-based publish/subscribe system for navigation events.

use std::collections::HashMap;

/// Unique subscriber identifier.
pub type SubscriberId = u64;

/// A message published to a topic.
#[derive(Debug, Clone)]
pub struct Message {
    /// Message sequence number within the topic.
    pub sequence: u64,
    /// Topic this message was published to.
    pub topic: String,
    /// Message key for partitioning.
    pub key: Option<String>,
    /// Payload bytes.
    pub payload: Vec<u8>,
    /// Timestamp (epoch millis).
    pub timestamp_ms: u64,
    /// Headers / metadata.
    pub headers: HashMap<String, String>,
}

/// Subscription filter.
#[derive(Debug, Clone)]
pub enum TopicFilter {
    /// Exact topic match.
    Exact(String),
    /// Prefix match (e.g., "nav.*").
    Prefix(String),
    /// All topics.
    All,
}

impl TopicFilter {
    /// Check if a topic matches this filter.
    pub fn matches(&self, topic: &str) -> bool {
        match self {
            TopicFilter::Exact(t) => t == topic,
            TopicFilter::Prefix(p) => topic.starts_with(p.as_str()),
            TopicFilter::All => true,
        }
    }
}

/// A subscriber with its filter and offset.
#[derive(Debug, Clone)]
struct Subscriber {
    #[allow(dead_code)]
    id: SubscriberId,
    filter: TopicFilter,
    /// Last consumed sequence per topic.
    offsets: HashMap<String, u64>,
    /// Group name for consumer groups.
    #[allow(dead_code)]
    group: Option<String>,
}

/// Topic-based message broker.
pub struct TopicBroker {
    /// Topic -> ordered messages.
    topics: HashMap<String, Vec<Message>>,
    /// Subscriber registry.
    subscribers: HashMap<SubscriberId, Subscriber>,
    /// Next subscriber ID.
    next_sub_id: SubscriberId,
    /// Total messages published.
    total_published: u64,
    /// Total messages consumed.
    total_consumed: u64,
}

impl TopicBroker {
    /// Create a new broker.
    pub fn new() -> Self {
        Self {
            topics: HashMap::new(),
            subscribers: HashMap::new(),
            next_sub_id: 1,
            total_published: 0,
            total_consumed: 0,
        }
    }

    /// Publish a message to a topic. Returns the sequence number.
    pub fn publish(&mut self, topic: &str, payload: Vec<u8>, timestamp_ms: u64) -> u64 {
        self.publish_with_key(topic, None, payload, timestamp_ms)
    }

    /// Publish a message with a key for partitioning.
    pub fn publish_with_key(
        &mut self,
        topic: &str,
        key: Option<String>,
        payload: Vec<u8>,
        timestamp_ms: u64,
    ) -> u64 {
        let messages = self.topics.entry(topic.to_string()).or_default();
        let sequence = messages.len() as u64;
        messages.push(Message {
            sequence,
            topic: topic.to_string(),
            key,
            payload,
            timestamp_ms,
            headers: HashMap::new(),
        });
        self.total_published += 1;
        sequence
    }

    /// Subscribe to topics matching a filter. Returns subscriber ID.
    pub fn subscribe(&mut self, filter: TopicFilter) -> SubscriberId {
        self.subscribe_with_group(filter, None)
    }

    /// Subscribe with a consumer group.
    pub fn subscribe_with_group(
        &mut self,
        filter: TopicFilter,
        group: Option<String>,
    ) -> SubscriberId {
        let id = self.next_sub_id;
        self.next_sub_id += 1;
        self.subscribers.insert(
            id,
            Subscriber {
                id,
                filter,
                offsets: HashMap::new(),
                group,
            },
        );
        id
    }

    /// Unsubscribe.
    pub fn unsubscribe(&mut self, sub_id: SubscriberId) -> bool {
        self.subscribers.remove(&sub_id).is_some()
    }

    /// Poll for new messages for a subscriber (up to max_messages).
    pub fn poll(&mut self, sub_id: SubscriberId, max_messages: usize) -> Vec<Message> {
        let subscriber = match self.subscribers.get(&sub_id) {
            Some(s) => s.clone(),
            None => return Vec::new(),
        };

        let mut result = Vec::new();

        for (topic, messages) in &self.topics {
            if !subscriber.filter.matches(topic) {
                continue;
            }
            let offset = subscriber.offsets.get(topic).copied().unwrap_or(0);
            for msg in messages.iter().skip(offset as usize) {
                if result.len() >= max_messages {
                    break;
                }
                result.push(msg.clone());
            }
        }

        // Update offsets
        if let Some(sub) = self.subscribers.get_mut(&sub_id) {
            for msg in &result {
                let offset = sub.offsets.entry(msg.topic.clone()).or_insert(0);
                if msg.sequence + 1 > *offset {
                    *offset = msg.sequence + 1;
                }
            }
        }
        self.total_consumed += result.len() as u64;

        result
    }

    /// Get the number of pending messages for a subscriber.
    pub fn pending_count(&self, sub_id: SubscriberId) -> u64 {
        let subscriber = match self.subscribers.get(&sub_id) {
            Some(s) => s,
            None => return 0,
        };

        let mut pending = 0u64;
        for (topic, messages) in &self.topics {
            if !subscriber.filter.matches(topic) {
                continue;
            }
            let offset = subscriber.offsets.get(topic).copied().unwrap_or(0);
            pending += (messages.len() as u64).saturating_sub(offset);
        }
        pending
    }

    /// Number of topics.
    pub fn topic_count(&self) -> usize {
        self.topics.len()
    }

    /// Number of subscribers.
    pub fn subscriber_count(&self) -> usize {
        self.subscribers.len()
    }

    /// Total published messages.
    pub fn total_published(&self) -> u64 {
        self.total_published
    }

    /// Total consumed messages.
    pub fn total_consumed(&self) -> u64 {
        self.total_consumed
    }

    /// Number of messages in a topic.
    pub fn topic_size(&self, topic: &str) -> usize {
        self.topics.get(topic).map_or(0, |m| m.len())
    }

    /// Clear all messages from a topic (retention).
    pub fn clear_topic(&mut self, topic: &str) -> usize {
        // Reset subscriber offsets for this topic so they don't skip
        // new messages if the topic is re-populated.
        for sub in self.subscribers.values_mut() {
            sub.offsets.remove(topic);
        }
        self.topics.remove(topic).map_or(0, |m| m.len())
    }
}

impl Default for TopicBroker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publish_and_poll() {
        let mut broker = TopicBroker::new();
        broker.publish("nav.position", b"pos1".to_vec(), 1000);
        broker.publish("nav.position", b"pos2".to_vec(), 2000);

        let sub = broker.subscribe(TopicFilter::Exact("nav.position".into()));
        let msgs = broker.poll(sub, 10);
        assert_eq!(msgs.len(), 2);
        assert_eq!(msgs[0].payload, b"pos1");
        assert_eq!(msgs[1].payload, b"pos2");
    }

    #[test]
    fn test_poll_returns_only_new() {
        let mut broker = TopicBroker::new();
        broker.publish("events", b"e1".to_vec(), 1000);
        let sub = broker.subscribe(TopicFilter::Exact("events".into()));

        let msgs = broker.poll(sub, 10);
        assert_eq!(msgs.len(), 1);

        // Poll again — no new messages
        let msgs = broker.poll(sub, 10);
        assert_eq!(msgs.len(), 0);

        // Publish more
        broker.publish("events", b"e2".to_vec(), 2000);
        let msgs = broker.poll(sub, 10);
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].payload, b"e2");
    }

    #[test]
    fn test_prefix_filter() {
        let mut broker = TopicBroker::new();
        broker.publish("nav.position", b"p1".to_vec(), 1000);
        broker.publish("nav.speed", b"s1".to_vec(), 1000);
        broker.publish("alerts.crash", b"a1".to_vec(), 1000);

        let sub = broker.subscribe(TopicFilter::Prefix("nav.".into()));
        let msgs = broker.poll(sub, 10);
        assert_eq!(msgs.len(), 2);
    }

    #[test]
    fn test_all_filter() {
        let mut broker = TopicBroker::new();
        broker.publish("t1", b"a".to_vec(), 1000);
        broker.publish("t2", b"b".to_vec(), 1000);

        let sub = broker.subscribe(TopicFilter::All);
        let msgs = broker.poll(sub, 10);
        assert_eq!(msgs.len(), 2);
    }

    #[test]
    fn test_unsubscribe() {
        let mut broker = TopicBroker::new();
        let sub = broker.subscribe(TopicFilter::All);
        assert_eq!(broker.subscriber_count(), 1);
        assert!(broker.unsubscribe(sub));
        assert_eq!(broker.subscriber_count(), 0);
    }

    #[test]
    fn test_pending_count() {
        let mut broker = TopicBroker::new();
        broker.publish("t", b"a".to_vec(), 1000);
        broker.publish("t", b"b".to_vec(), 2000);

        let sub = broker.subscribe(TopicFilter::Exact("t".into()));
        assert_eq!(broker.pending_count(sub), 2);

        broker.poll(sub, 1);
        assert_eq!(broker.pending_count(sub), 1);
    }

    #[test]
    fn test_counters() {
        let mut broker = TopicBroker::new();
        broker.publish("t", b"a".to_vec(), 1000);
        broker.publish("t", b"b".to_vec(), 2000);
        assert_eq!(broker.total_published(), 2);
        assert_eq!(broker.topic_size("t"), 2);

        let sub = broker.subscribe(TopicFilter::All);
        broker.poll(sub, 10);
        assert_eq!(broker.total_consumed(), 2);
    }

    #[test]
    fn test_clear_topic() {
        let mut broker = TopicBroker::new();
        broker.publish("t", b"a".to_vec(), 1000);
        broker.publish("t", b"b".to_vec(), 2000);
        assert_eq!(broker.clear_topic("t"), 2);
        assert_eq!(broker.topic_size("t"), 0);
    }

    #[test]
    fn test_clear_topic_resets_offsets() {
        // Regression: after clear_topic, subscribers must see new messages
        let mut broker = TopicBroker::new();
        let sub = broker.subscribe(TopicFilter::Exact("t".into()));

        broker.publish("t", b"a".to_vec(), 1000);
        broker.publish("t", b"b".to_vec(), 2000);
        let msgs = broker.poll(sub, 10);
        assert_eq!(msgs.len(), 2);

        // Clear and re-publish
        broker.clear_topic("t");
        broker.publish("t", b"c".to_vec(), 3000);
        let msgs = broker.poll(sub, 10);
        assert_eq!(
            msgs.len(),
            1,
            "subscriber must see messages after clear_topic"
        );
        assert_eq!(msgs[0].payload, b"c");
    }

    #[test]
    fn test_publish_with_key() {
        let mut broker = TopicBroker::new();
        broker.publish_with_key("t", Some("user-123".into()), b"data".to_vec(), 1000);
        let sub = broker.subscribe(TopicFilter::Exact("t".into()));
        let msgs = broker.poll(sub, 10);
        assert_eq!(msgs[0].key.as_deref(), Some("user-123"));
    }
}
