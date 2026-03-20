//! Bounded queue implementation with backpressure.

use std::collections::VecDeque;

/// Result of a push operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PushResult {
    /// Item was successfully enqueued.
    Ok,
    /// Queue is full — backpressure applied.
    Full,
}

/// A bounded queue with configurable capacity and backpressure support.
pub struct BoundedQueue<T> {
    buffer: VecDeque<T>,
    capacity: usize,
    enqueued: u64,
    dequeued: u64,
    rejected: u64,
}

impl<T> BoundedQueue<T> {
    /// Create a new bounded queue with the given capacity.
    /// Capacity is clamped to at least 1.
    pub fn new(capacity: usize) -> Self {
        let capacity = capacity.max(1);
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
            enqueued: 0,
            dequeued: 0,
            rejected: 0,
        }
    }

    /// Try to push an item. Returns `PushResult::Full` if at capacity.
    pub fn push(&mut self, item: T) -> PushResult {
        if self.buffer.len() >= self.capacity {
            self.rejected = self.rejected.saturating_add(1);
            return PushResult::Full;
        }
        self.buffer.push_back(item);
        self.enqueued = self.enqueued.saturating_add(1);
        PushResult::Ok
    }

    /// Force push an item, dropping the oldest if at capacity.
    pub fn force_push(&mut self, item: T) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(item);
        self.enqueued = self.enqueued.saturating_add(1);
    }

    /// Pop an item from the front.
    pub fn pop(&mut self) -> Option<T> {
        let item = self.buffer.pop_front();
        if item.is_some() {
            self.dequeued = self.dequeued.saturating_add(1);
        }
        item
    }

    /// Peek at the front item without removing it.
    pub fn peek(&self) -> Option<&T> {
        self.buffer.front()
    }

    /// Number of items in the queue.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Whether the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Whether the queue is full.
    pub fn is_full(&self) -> bool {
        self.buffer.len() >= self.capacity
    }

    /// Maximum capacity.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Remaining space.
    pub fn remaining(&self) -> usize {
        self.capacity.saturating_sub(self.buffer.len())
    }

    /// Load factor (0.0 to 1.0).
    pub fn load_factor(&self) -> f64 {
        self.buffer.len() as f64 / self.capacity as f64
    }

    /// Total items enqueued.
    pub fn enqueued_count(&self) -> u64 {
        self.enqueued
    }

    /// Total items dequeued.
    pub fn dequeued_count(&self) -> u64 {
        self.dequeued
    }

    /// Total items rejected (backpressure).
    pub fn rejected_count(&self) -> u64 {
        self.rejected
    }

    /// Clear all items.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Drain all items.
    pub fn drain(&mut self) -> Vec<T> {
        let items: Vec<T> = self.buffer.drain(..).collect();
        self.dequeued = self.dequeued.saturating_add(items.len() as u64);
        items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut q = BoundedQueue::new(3);
        assert_eq!(q.push(1), PushResult::Ok);
        assert_eq!(q.push(2), PushResult::Ok);
        assert_eq!(q.pop(), Some(1));
        assert_eq!(q.pop(), Some(2));
    }

    #[test]
    fn test_full_backpressure() {
        let mut q = BoundedQueue::new(2);
        assert_eq!(q.push(1), PushResult::Ok);
        assert_eq!(q.push(2), PushResult::Ok);
        assert_eq!(q.push(3), PushResult::Full);
        assert_eq!(q.rejected_count(), 1);
    }

    #[test]
    fn test_force_push() {
        let mut q = BoundedQueue::new(2);
        q.force_push(1);
        q.force_push(2);
        q.force_push(3); // drops 1
        assert_eq!(q.pop(), Some(2));
        assert_eq!(q.pop(), Some(3));
    }

    #[test]
    fn test_peek() {
        let mut q = BoundedQueue::new(10);
        assert_eq!(q.peek(), None);
        q.push(42);
        assert_eq!(q.peek(), Some(&42));
    }

    #[test]
    fn test_capacity_and_remaining() {
        let mut q = BoundedQueue::new(5);
        assert_eq!(q.capacity(), 5);
        assert_eq!(q.remaining(), 5);
        q.push(1);
        assert_eq!(q.remaining(), 4);
    }

    #[test]
    fn test_load_factor() {
        let mut q = BoundedQueue::new(4);
        q.push(1);
        q.push(2);
        assert!((q.load_factor() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_drain() {
        let mut q = BoundedQueue::new(10);
        q.push(1);
        q.push(2);
        q.push(3);
        let items = q.drain();
        assert_eq!(items, vec![1, 2, 3]);
        assert!(q.is_empty());
    }

    #[test]
    fn test_clear() {
        let mut q = BoundedQueue::new(10);
        q.push(1);
        q.push(2);
        q.clear();
        assert!(q.is_empty());
    }

    #[test]
    fn test_is_full() {
        let mut q = BoundedQueue::new(2);
        assert!(!q.is_full());
        q.push(1);
        q.push(2);
        assert!(q.is_full());
    }

    #[test]
    fn test_counts() {
        let mut q = BoundedQueue::new(3);
        q.push(1);
        q.push(2);
        q.push(3);
        q.push(4); // rejected
        q.pop();
        assert_eq!(q.enqueued_count(), 3);
        assert_eq!(q.dequeued_count(), 1);
        assert_eq!(q.rejected_count(), 1);
    }
}
