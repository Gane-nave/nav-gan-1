//! Work-stealing deque implementation.

use std::collections::VecDeque;

/// A work-stealing deque for parallel task scheduling.
/// The owner pushes/pops from the back; stealers steal from the front.
pub struct WorkStealDeque<T> {
    deque: VecDeque<T>,
    stolen_count: u64,
    pushed_count: u64,
    popped_count: u64,
}

impl<T> WorkStealDeque<T> {
    /// Create a new empty work-stealing deque.
    pub fn new() -> Self {
        Self {
            deque: VecDeque::new(),
            stolen_count: 0,
            pushed_count: 0,
            popped_count: 0,
        }
    }

    /// Create with pre-allocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            deque: VecDeque::with_capacity(capacity),
            stolen_count: 0,
            pushed_count: 0,
            popped_count: 0,
        }
    }

    /// Push a task onto the back (owner operation).
    pub fn push(&mut self, item: T) {
        self.deque.push_back(item);
        self.pushed_count = self.pushed_count.saturating_add(1);
    }

    /// Pop a task from the back (owner operation).
    pub fn pop(&mut self) -> Option<T> {
        let item = self.deque.pop_back();
        if item.is_some() {
            self.popped_count = self.popped_count.saturating_add(1);
        }
        item
    }

    /// Steal a task from the front (stealer operation).
    pub fn steal(&mut self) -> Option<T> {
        let item = self.deque.pop_front();
        if item.is_some() {
            self.stolen_count = self.stolen_count.saturating_add(1);
        }
        item
    }

    /// Steal up to `n` tasks from the front.
    pub fn steal_batch(&mut self, n: usize) -> Vec<T> {
        let count = n.min(self.deque.len());
        let mut batch = Vec::with_capacity(count);
        for _ in 0..count {
            if let Some(item) = self.deque.pop_front() {
                batch.push(item);
                self.stolen_count = self.stolen_count.saturating_add(1);
            }
        }
        batch
    }

    /// Number of items in the deque.
    pub fn len(&self) -> usize {
        self.deque.len()
    }

    /// Whether the deque is empty.
    pub fn is_empty(&self) -> bool {
        self.deque.is_empty()
    }

    /// Total number of items pushed.
    pub fn pushed_count(&self) -> u64 {
        self.pushed_count
    }

    /// Total number of items popped by owner.
    pub fn popped_count(&self) -> u64 {
        self.popped_count
    }

    /// Total number of items stolen.
    pub fn stolen_count(&self) -> u64 {
        self.stolen_count
    }

    /// Clear all items.
    pub fn clear(&mut self) {
        self.deque.clear();
    }

    /// Drain all items.
    pub fn drain(&mut self) -> Vec<T> {
        let items: Vec<T> = self.deque.drain(..).collect();
        items
    }
}

impl<T> Default for WorkStealDeque<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut d = WorkStealDeque::new();
        d.push(1);
        d.push(2);
        d.push(3);
        assert_eq!(d.pop(), Some(3)); // LIFO for owner
        assert_eq!(d.pop(), Some(2));
        assert_eq!(d.pop(), Some(1));
        assert_eq!(d.pop(), None);
    }

    #[test]
    fn test_steal() {
        let mut d = WorkStealDeque::new();
        d.push(1);
        d.push(2);
        d.push(3);
        assert_eq!(d.steal(), Some(1)); // FIFO for stealer
        assert_eq!(d.steal(), Some(2));
    }

    #[test]
    fn test_steal_batch() {
        let mut d = WorkStealDeque::new();
        for i in 0..10 {
            d.push(i);
        }
        let batch = d.steal_batch(3);
        assert_eq!(batch, vec![0, 1, 2]);
        assert_eq!(d.len(), 7);
    }

    #[test]
    fn test_empty() {
        let d: WorkStealDeque<i32> = WorkStealDeque::new();
        assert!(d.is_empty());
        assert_eq!(d.len(), 0);
    }

    #[test]
    fn test_counts() {
        let mut d = WorkStealDeque::new();
        d.push(1);
        d.push(2);
        d.push(3);
        assert_eq!(d.pushed_count(), 3);
        d.pop();
        assert_eq!(d.popped_count(), 1);
        d.steal();
        assert_eq!(d.stolen_count(), 1);
    }

    #[test]
    fn test_clear() {
        let mut d = WorkStealDeque::new();
        d.push(1);
        d.push(2);
        d.clear();
        assert!(d.is_empty());
    }

    #[test]
    fn test_drain() {
        let mut d = WorkStealDeque::new();
        d.push(1);
        d.push(2);
        d.push(3);
        let items = d.drain();
        assert_eq!(items, vec![1, 2, 3]);
        assert!(d.is_empty());
    }

    #[test]
    fn test_with_capacity() {
        let d: WorkStealDeque<i32> = WorkStealDeque::with_capacity(100);
        assert!(d.is_empty());
    }

    #[test]
    fn test_interleaved_push_steal() {
        let mut d = WorkStealDeque::new();
        d.push(1);
        d.push(2);
        assert_eq!(d.steal(), Some(1));
        d.push(3);
        assert_eq!(d.steal(), Some(2));
        assert_eq!(d.pop(), Some(3));
    }

    #[test]
    fn test_steal_batch_more_than_available() {
        let mut d = WorkStealDeque::new();
        d.push(1);
        d.push(2);
        let batch = d.steal_batch(10);
        assert_eq!(batch.len(), 2);
        assert!(d.is_empty());
    }
}
