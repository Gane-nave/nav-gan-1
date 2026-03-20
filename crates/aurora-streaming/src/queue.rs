//! Ordered message queue with priority support.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Priority level for messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Priority {
    /// Low priority.
    Low = 0,
    /// Normal priority.
    Normal = 1,
    /// High priority.
    High = 2,
    /// Critical — process immediately.
    Critical = 3,
}

impl Priority {
    fn rank(self) -> u8 {
        self as u8
    }
}

/// A queued message with priority.
#[derive(Debug, Clone)]
pub struct QueuedMessage {
    /// Unique message ID.
    pub id: u64,
    /// Priority level.
    pub priority: Priority,
    /// Payload.
    pub payload: Vec<u8>,
    /// Enqueue timestamp (epoch millis).
    pub enqueued_at_ms: u64,
    /// Number of delivery attempts.
    pub attempts: u32,
    /// Maximum delivery attempts before dead-letter.
    pub max_attempts: u32,
}

/// Wrapper for priority queue ordering.
#[derive(Debug, Clone)]
struct PriorityEntry {
    msg: QueuedMessage,
}

impl PartialEq for PriorityEntry {
    fn eq(&self, other: &Self) -> bool {
        self.msg.priority == other.msg.priority && self.msg.id == other.msg.id
    }
}

impl Eq for PriorityEntry {}

impl Ord for PriorityEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher priority first, then earlier enqueue time
        self.msg
            .priority
            .rank()
            .cmp(&other.msg.priority.rank())
            .then_with(|| other.msg.enqueued_at_ms.cmp(&self.msg.enqueued_at_ms))
    }
}

impl PartialOrd for PriorityEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Priority message queue.
pub struct MessageQueue {
    heap: BinaryHeap<PriorityEntry>,
    /// Dead letter queue for exhausted retry messages.
    dead_letters: Vec<QueuedMessage>,
    next_id: u64,
    /// Total enqueued.
    enqueued: u64,
    /// Total dequeued.
    dequeued: u64,
    /// Total dead-lettered.
    dead_lettered: u64,
    /// Default max attempts.
    default_max_attempts: u32,
}

impl MessageQueue {
    /// Create a new message queue.
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            dead_letters: Vec::new(),
            next_id: 1,
            enqueued: 0,
            dequeued: 0,
            dead_lettered: 0,
            default_max_attempts: 3,
        }
    }

    /// Set default max delivery attempts.
    pub fn with_max_attempts(mut self, max: u32) -> Self {
        self.default_max_attempts = max;
        self
    }

    /// Enqueue a message with priority. Returns the message ID.
    pub fn enqueue(&mut self, payload: Vec<u8>, priority: Priority, now_ms: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let msg = QueuedMessage {
            id,
            priority,
            payload,
            enqueued_at_ms: now_ms,
            attempts: 0,
            max_attempts: self.default_max_attempts,
        };
        self.heap.push(PriorityEntry { msg });
        self.enqueued += 1;
        id
    }

    /// Dequeue the highest-priority message.
    pub fn dequeue(&mut self) -> Option<QueuedMessage> {
        let entry = self.heap.pop()?;
        let mut msg = entry.msg;
        msg.attempts += 1;
        self.dequeued += 1;
        Some(msg)
    }

    /// Re-enqueue a message that failed processing (nack).
    pub fn nack(&mut self, msg: QueuedMessage) {
        if msg.attempts >= msg.max_attempts {
            self.dead_letters.push(msg);
            self.dead_lettered += 1;
        } else {
            self.heap.push(PriorityEntry { msg });
        }
    }

    /// Peek at the highest-priority message without removing it.
    pub fn peek(&self) -> Option<&QueuedMessage> {
        self.heap.peek().map(|e| &e.msg)
    }

    /// Current queue depth.
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Whether the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Number of dead-lettered messages.
    pub fn dead_letter_count(&self) -> usize {
        self.dead_letters.len()
    }

    /// Total enqueued.
    pub fn total_enqueued(&self) -> u64 {
        self.enqueued
    }

    /// Total dequeued.
    pub fn total_dequeued(&self) -> u64 {
        self.dequeued
    }

    /// Drain dead letter queue.
    pub fn drain_dead_letters(&mut self) -> Vec<QueuedMessage> {
        std::mem::take(&mut self.dead_letters)
    }

    /// Clear the queue.
    pub fn clear(&mut self) {
        self.heap.clear();
    }
}

impl Default for MessageQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_dequeue() {
        let mut q = MessageQueue::new();
        q.enqueue(b"hello".to_vec(), Priority::Normal, 1000);
        let msg = q.dequeue().unwrap();
        assert_eq!(msg.payload, b"hello");
        assert_eq!(msg.attempts, 1);
    }

    #[test]
    fn test_priority_ordering() {
        let mut q = MessageQueue::new();
        q.enqueue(b"low".to_vec(), Priority::Low, 1000);
        q.enqueue(b"critical".to_vec(), Priority::Critical, 2000);
        q.enqueue(b"normal".to_vec(), Priority::Normal, 1500);

        let msg1 = q.dequeue().unwrap();
        assert_eq!(msg1.payload, b"critical");
        let msg2 = q.dequeue().unwrap();
        assert_eq!(msg2.payload, b"normal");
        let msg3 = q.dequeue().unwrap();
        assert_eq!(msg3.payload, b"low");
    }

    #[test]
    fn test_nack_and_retry() {
        let mut q = MessageQueue::new().with_max_attempts(3);
        q.enqueue(b"retry_me".to_vec(), Priority::Normal, 1000);

        let msg = q.dequeue().unwrap();
        assert_eq!(msg.attempts, 1);
        q.nack(msg);

        let msg = q.dequeue().unwrap();
        assert_eq!(msg.attempts, 2);
        q.nack(msg);

        let msg = q.dequeue().unwrap();
        assert_eq!(msg.attempts, 3);
        q.nack(msg);

        // Should be dead-lettered now
        assert!(q.is_empty());
        assert_eq!(q.dead_letter_count(), 1);
    }

    #[test]
    fn test_dead_letter_drain() {
        let mut q = MessageQueue::new().with_max_attempts(1);
        q.enqueue(b"fail".to_vec(), Priority::Normal, 1000);
        let msg = q.dequeue().unwrap();
        q.nack(msg);

        let dead = q.drain_dead_letters();
        assert_eq!(dead.len(), 1);
        assert_eq!(q.dead_letter_count(), 0);
    }

    #[test]
    fn test_peek() {
        let mut q = MessageQueue::new();
        assert!(q.peek().is_none());
        q.enqueue(b"x".to_vec(), Priority::High, 1000);
        assert_eq!(q.peek().unwrap().payload, b"x");
        assert_eq!(q.len(), 1); // peek doesn't remove
    }

    #[test]
    fn test_counters() {
        let mut q = MessageQueue::new();
        q.enqueue(b"a".to_vec(), Priority::Normal, 1000);
        q.enqueue(b"b".to_vec(), Priority::Normal, 2000);
        assert_eq!(q.total_enqueued(), 2);

        q.dequeue();
        assert_eq!(q.total_dequeued(), 1);
    }

    #[test]
    fn test_clear() {
        let mut q = MessageQueue::new();
        q.enqueue(b"a".to_vec(), Priority::Normal, 1000);
        q.enqueue(b"b".to_vec(), Priority::Normal, 2000);
        q.clear();
        assert!(q.is_empty());
    }

    #[test]
    fn test_same_priority_fifo() {
        let mut q = MessageQueue::new();
        q.enqueue(b"first".to_vec(), Priority::Normal, 1000);
        q.enqueue(b"second".to_vec(), Priority::Normal, 2000);

        let msg1 = q.dequeue().unwrap();
        assert_eq!(msg1.payload, b"first");
    }
}
