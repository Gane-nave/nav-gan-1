//! Fixed-size circular buffer that overwrites oldest entries when full.

/// A circular ring buffer with fixed capacity.
#[derive(Debug)]
pub struct RingBuffer<T> {
    data: Vec<Option<T>>,
    capacity: usize,
    head: usize,
    count: usize,
    total_writes: u64,
    total_overwrites: u64,
    total_reads: u64,
}

impl<T: Clone> RingBuffer<T> {
    /// Create a new ring buffer with the given capacity.
    pub fn new(capacity: usize) -> Self {
        let capacity = capacity.max(1);
        Self {
            data: (0..capacity).map(|_| None).collect(),
            capacity,
            head: 0,
            count: 0,
            total_writes: 0,
            total_overwrites: 0,
            total_reads: 0,
        }
    }

    /// Push an item into the buffer. Returns the overwritten item if buffer was full.
    pub fn push(&mut self, item: T) -> Option<T> {
        let overwritten = if self.count == self.capacity {
            self.total_overwrites = self.total_overwrites.saturating_add(1);
            self.data[self.head].take()
        } else {
            self.count += 1;
            None
        };
        self.data[self.head] = Some(item);
        self.head = (self.head + 1) % self.capacity;
        self.total_writes = self.total_writes.saturating_add(1);
        overwritten
    }

    /// Read the most recently pushed item.
    pub fn peek_latest(&mut self) -> Option<&T> {
        if self.count == 0 {
            return None;
        }
        let idx = if self.head == 0 {
            self.capacity - 1
        } else {
            self.head - 1
        };
        self.total_reads = self.total_reads.saturating_add(1);
        self.data[idx].as_ref()
    }

    /// Read the oldest item in the buffer.
    pub fn peek_oldest(&mut self) -> Option<&T> {
        if self.count == 0 {
            return None;
        }
        let oldest_idx = if self.count == self.capacity {
            self.head // head points to next write position = oldest
        } else {
            0
        };
        self.total_reads = self.total_reads.saturating_add(1);
        self.data[oldest_idx].as_ref()
    }

    /// Get all items in order from oldest to newest.
    pub fn items(&self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.count);
        if self.count == 0 {
            return result;
        }
        let start = if self.count == self.capacity {
            self.head
        } else {
            0
        };
        for i in 0..self.count {
            let idx = (start + i) % self.capacity;
            if let Some(item) = &self.data[idx] {
                result.push(item.clone());
            }
        }
        result
    }

    /// Pop the oldest item from the buffer.
    pub fn pop_oldest(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        // Collect all items in order, then rebuild without the oldest
        let mut all = self.items();
        let oldest = all.remove(0);
        self.total_reads = self.total_reads.saturating_add(1);

        // Clear and rebuild
        for slot in &mut self.data {
            *slot = None;
        }
        self.count = all.len();
        self.head = 0;
        for (i, it) in all.into_iter().enumerate() {
            self.data[i] = Some(it);
            self.head = i + 1;
        }
        if self.count == 0 {
            self.head = 0;
        }
        Some(oldest)
    }

    /// Current number of items.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Whether the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Whether the buffer is full.
    pub fn is_full(&self) -> bool {
        self.count == self.capacity
    }

    /// Buffer capacity.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Fill ratio.
    pub fn fill_ratio(&self) -> f64 {
        self.count as f64 / self.capacity as f64
    }

    /// Total writes.
    pub fn total_writes(&self) -> u64 {
        self.total_writes
    }

    /// Total overwrites (items lost).
    pub fn total_overwrites(&self) -> u64 {
        self.total_overwrites
    }

    /// Total reads.
    pub fn total_reads(&self) -> u64 {
        self.total_reads
    }

    /// Overwrite ratio: fraction of writes that overwrote existing data.
    pub fn overwrite_ratio(&self) -> f64 {
        if self.total_writes == 0 {
            return 0.0;
        }
        self.total_overwrites as f64 / self.total_writes as f64
    }

    /// Clear the buffer.
    pub fn clear(&mut self) {
        for slot in &mut self.data {
            *slot = None;
        }
        self.head = 0;
        self.count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_buffer() {
        let rb: RingBuffer<u32> = RingBuffer::new(5);
        assert_eq!(rb.capacity(), 5);
        assert!(rb.is_empty());
        assert_eq!(rb.len(), 0);
    }

    #[test]
    fn test_push_and_peek() {
        let mut rb = RingBuffer::new(3);
        rb.push(10u32);
        rb.push(20);
        assert_eq!(rb.peek_latest(), Some(&20));
        assert_eq!(rb.peek_oldest(), Some(&10));
        assert_eq!(rb.len(), 2);
    }

    #[test]
    fn test_overwrite_when_full() {
        let mut rb = RingBuffer::new(3);
        rb.push(1u32);
        rb.push(2);
        rb.push(3);
        assert!(rb.is_full());
        let overwritten = rb.push(4);
        assert_eq!(overwritten, Some(1)); // oldest overwritten
        assert_eq!(rb.total_overwrites(), 1);
        let items = rb.items();
        assert_eq!(items, vec![2, 3, 4]);
    }

    #[test]
    fn test_items_order() {
        let mut rb = RingBuffer::new(4);
        rb.push(10u32);
        rb.push(20);
        rb.push(30);
        assert_eq!(rb.items(), vec![10, 20, 30]);
    }

    #[test]
    fn test_wrap_around() {
        let mut rb = RingBuffer::new(3);
        rb.push(1u32);
        rb.push(2);
        rb.push(3);
        rb.push(4);
        rb.push(5);
        // Buffer should contain [3, 4, 5]
        assert_eq!(rb.items(), vec![3, 4, 5]);
        assert_eq!(rb.total_writes(), 5);
        assert_eq!(rb.total_overwrites(), 2);
    }

    #[test]
    fn test_pop_oldest() {
        let mut rb = RingBuffer::new(3);
        rb.push(10u32);
        rb.push(20);
        rb.push(30);
        let popped = rb.pop_oldest();
        assert_eq!(popped, Some(10));
        assert_eq!(rb.len(), 2);
        assert_eq!(rb.items(), vec![20, 30]);
    }

    #[test]
    fn test_fill_ratio() {
        let mut rb = RingBuffer::new(4);
        rb.push(1u32);
        rb.push(2);
        assert!((rb.fill_ratio() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_overwrite_ratio() {
        let mut rb = RingBuffer::new(2);
        rb.push(1u32);
        rb.push(2);
        rb.push(3); // overwrite
        rb.push(4); // overwrite
                    // 2 overwrites out of 4 writes = 0.5
        assert!((rb.overwrite_ratio() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_clear() {
        let mut rb = RingBuffer::new(3);
        rb.push(1u32);
        rb.push(2);
        rb.clear();
        assert!(rb.is_empty());
        assert_eq!(rb.len(), 0);
    }

    #[test]
    fn test_peek_empty() {
        let mut rb: RingBuffer<u32> = RingBuffer::new(3);
        assert_eq!(rb.peek_latest(), None);
        assert_eq!(rb.peek_oldest(), None);
    }

    #[test]
    fn test_capacity_min_one() {
        let rb: RingBuffer<u32> = RingBuffer::new(0);
        assert_eq!(rb.capacity(), 1); // minimum capacity is 1
    }
}
