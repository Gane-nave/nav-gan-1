//! Binary heap — priority queue with min/max modes.

use crate::entry::HeapEntry;

/// Heap mode — determines ordering.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeapMode {
    /// Min-heap: lowest priority value at the top.
    Min,
    /// Max-heap: highest priority value at the top.
    Max,
}

/// A binary heap (priority queue).
pub struct BinaryHeap<T: Clone> {
    /// The heap storage.
    entries: Vec<HeapEntry<T>>,
    /// Heap mode.
    mode: HeapMode,
    /// Next sequence number for stable ordering.
    next_seq: u64,
    /// Total pushes (lifetime).
    total_pushes: u64,
    /// Total pops (lifetime).
    total_pops: u64,
    /// Peak size reached.
    peak_size: usize,
}

impl<T: Clone> BinaryHeap<T> {
    /// Create a new binary heap with the given mode.
    pub fn new(mode: HeapMode) -> Self {
        Self {
            entries: Vec::new(),
            mode,
            next_seq: 0,
            total_pushes: 0,
            total_pops: 0,
            peak_size: 0,
        }
    }

    /// Create a new min-heap.
    pub fn min_heap() -> Self {
        Self::new(HeapMode::Min)
    }

    /// Create a new max-heap.
    pub fn max_heap() -> Self {
        Self::new(HeapMode::Max)
    }

    /// Push a value with a priority.
    pub fn push(&mut self, priority: i64, value: T) {
        let seq = self.next_seq;
        self.next_seq += 1;
        self.entries.push(HeapEntry::new(priority, seq, value));
        self.total_pushes += 1;
        self.sift_up(self.entries.len() - 1);
        if self.entries.len() > self.peak_size {
            self.peak_size = self.entries.len();
        }
    }

    /// Pop the highest-priority element.
    pub fn pop(&mut self) -> Option<(i64, T)> {
        if self.entries.is_empty() {
            return None;
        }
        self.total_pops += 1;
        let last = self.entries.len() - 1;
        self.entries.swap(0, last);
        let entry = self.entries.pop().unwrap();
        if !self.entries.is_empty() {
            self.sift_down(0);
        }
        Some((entry.priority(), entry.into_value()))
    }

    /// Peek at the highest-priority element without removing it.
    pub fn peek(&self) -> Option<(i64, &T)> {
        self.entries.first().map(|e| (e.priority(), e.value()))
    }

    /// Number of elements.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the heap is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Clear the heap.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Get the heap mode.
    pub fn mode(&self) -> HeapMode {
        self.mode
    }

    /// Peak size reached.
    pub fn peak_size(&self) -> usize {
        self.peak_size
    }

    /// Total pushes (lifetime).
    pub fn total_pushes(&self) -> u64 {
        self.total_pushes
    }

    /// Total pops (lifetime).
    pub fn total_pops(&self) -> u64 {
        self.total_pops
    }

    /// Drain all elements in priority order.
    pub fn drain_sorted(&mut self) -> Vec<(i64, T)> {
        let mut result = Vec::with_capacity(self.entries.len());
        while let Some(item) = self.pop() {
            result.push(item);
        }
        result
    }

    fn is_higher_priority(&self, a: usize, b: usize) -> bool {
        match self.mode {
            HeapMode::Min => self.entries[a].is_higher_priority_min(&self.entries[b]),
            HeapMode::Max => self.entries[a].is_higher_priority_max(&self.entries[b]),
        }
    }

    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.is_higher_priority(idx, parent) {
                self.entries.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut idx: usize) {
        let len = self.entries.len();
        loop {
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;
            let mut best = idx;

            if left < len && self.is_higher_priority(left, best) {
                best = left;
            }
            if right < len && self.is_higher_priority(right, best) {
                best = right;
            }

            if best != idx {
                self.entries.swap(idx, best);
                idx = best;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_heap_basic() {
        let mut heap = BinaryHeap::min_heap();
        heap.push(5, "five");
        heap.push(1, "one");
        heap.push(3, "three");
        assert_eq!(heap.pop(), Some((1, "one")));
        assert_eq!(heap.pop(), Some((3, "three")));
        assert_eq!(heap.pop(), Some((5, "five")));
        assert!(heap.is_empty());
    }

    #[test]
    fn test_max_heap_basic() {
        let mut heap = BinaryHeap::max_heap();
        heap.push(5, "five");
        heap.push(1, "one");
        heap.push(3, "three");
        assert_eq!(heap.pop(), Some((5, "five")));
        assert_eq!(heap.pop(), Some((3, "three")));
        assert_eq!(heap.pop(), Some((1, "one")));
    }

    #[test]
    fn test_peek() {
        let mut heap = BinaryHeap::min_heap();
        assert!(heap.peek().is_none());
        heap.push(10, "ten");
        heap.push(2, "two");
        assert_eq!(heap.peek(), Some((2, &"two")));
        assert_eq!(heap.len(), 2); // peek doesn't remove
    }

    #[test]
    fn test_stable_ordering() {
        let mut heap = BinaryHeap::min_heap();
        heap.push(1, "first");
        heap.push(1, "second");
        heap.push(1, "third");
        // Same priority — FIFO order
        assert_eq!(heap.pop(), Some((1, "first")));
        assert_eq!(heap.pop(), Some((1, "second")));
        assert_eq!(heap.pop(), Some((1, "third")));
    }

    #[test]
    fn test_drain_sorted() {
        let mut heap = BinaryHeap::min_heap();
        heap.push(3, "c");
        heap.push(1, "a");
        heap.push(2, "b");
        let drained = heap.drain_sorted();
        assert_eq!(drained, vec![(1, "a"), (2, "b"), (3, "c")]);
        assert!(heap.is_empty());
    }

    #[test]
    fn test_clear() {
        let mut heap = BinaryHeap::min_heap();
        heap.push(1, "a");
        heap.push(2, "b");
        heap.clear();
        assert!(heap.is_empty());
        assert_eq!(heap.len(), 0);
    }

    #[test]
    fn test_stats() {
        let mut heap = BinaryHeap::min_heap();
        heap.push(1, "a");
        heap.push(2, "b");
        heap.push(3, "c");
        heap.pop();
        assert_eq!(heap.total_pushes(), 3);
        assert_eq!(heap.total_pops(), 1);
        assert_eq!(heap.peak_size(), 3);
    }

    #[test]
    fn test_negative_priorities() {
        let mut heap = BinaryHeap::min_heap();
        heap.push(-5, "urgent");
        heap.push(0, "normal");
        heap.push(10, "low");
        assert_eq!(heap.pop(), Some((-5, "urgent")));
    }

    #[test]
    fn test_pop_empty() {
        let mut heap: BinaryHeap<i32> = BinaryHeap::min_heap();
        assert!(heap.pop().is_none());
    }

    #[test]
    fn test_mode() {
        let min = BinaryHeap::<i32>::min_heap();
        let max = BinaryHeap::<i32>::max_heap();
        assert_eq!(min.mode(), HeapMode::Min);
        assert_eq!(max.mode(), HeapMode::Max);
    }
}
