//! Fair semaphore — FIFO-ordered waiting queue for permit acquisition.

/// A waiter in the fair semaphore queue.
#[derive(Debug, Clone, PartialEq)]
pub struct Waiter {
    /// Unique waiter ID.
    id: u64,
    /// Timestamp when the waiter was enqueued.
    enqueued_at_ms: u64,
    /// Priority (lower = higher priority, 0 = default).
    priority: u32,
}

impl Waiter {
    /// Create a new waiter.
    pub fn new(id: u64, enqueued_at_ms: u64) -> Self {
        Self {
            id,
            enqueued_at_ms,
            priority: 0,
        }
    }

    /// Create a waiter with priority.
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    /// Get the waiter ID.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get the enqueue timestamp.
    pub fn enqueued_at_ms(&self) -> u64 {
        self.enqueued_at_ms
    }

    /// Get the priority.
    pub fn priority(&self) -> u32 {
        self.priority
    }

    /// How long this waiter has been waiting.
    pub fn wait_duration_ms(&self, now_ms: u64) -> u64 {
        now_ms.saturating_sub(self.enqueued_at_ms)
    }
}

/// A fair semaphore with a FIFO waiting queue.
pub struct FairSemaphore {
    /// Maximum concurrent permits.
    max_permits: u64,
    /// Currently held permit count.
    held_count: u64,
    /// Waiting queue (FIFO with priority).
    queue: Vec<Waiter>,
    /// Next waiter ID.
    next_waiter_id: u64,
    /// Total waiters ever enqueued.
    total_enqueued: u64,
    /// Total waiters granted.
    total_granted: u64,
    /// Total waiters that timed out.
    total_timed_out: u64,
}

impl FairSemaphore {
    /// Create a new fair semaphore.
    pub fn new(max_permits: u64) -> Self {
        Self {
            max_permits,
            held_count: 0,
            queue: Vec::new(),
            next_waiter_id: 1,
            total_enqueued: 0,
            total_granted: 0,
            total_timed_out: 0,
        }
    }

    /// Enqueue a waiter. Returns the waiter ID.
    pub fn enqueue(&mut self, now_ms: u64) -> u64 {
        let id = self.next_waiter_id;
        self.next_waiter_id += 1;
        self.queue.push(Waiter::new(id, now_ms));
        self.total_enqueued += 1;
        id
    }

    /// Enqueue a waiter with priority. Returns the waiter ID.
    pub fn enqueue_with_priority(&mut self, now_ms: u64, priority: u32) -> u64 {
        let id = self.next_waiter_id;
        self.next_waiter_id += 1;
        self.queue
            .push(Waiter::new(id, now_ms).with_priority(priority));
        self.total_enqueued += 1;
        id
    }

    /// Try to grant the next waiter in the queue.
    /// Returns the granted waiter ID if successful.
    pub fn try_grant_next(&mut self) -> Option<u64> {
        if self.held_count >= self.max_permits {
            return None;
        }

        if self.queue.is_empty() {
            return None;
        }

        // Find highest priority waiter (lowest priority number, FIFO for ties)
        let best_idx = self
            .queue
            .iter()
            .enumerate()
            .min_by_key(|(_, w)| (w.priority(), w.enqueued_at_ms()))
            .map(|(i, _)| i)?;

        let waiter = self.queue.remove(best_idx);
        self.held_count += 1;
        self.total_granted += 1;
        Some(waiter.id())
    }

    /// Release a held permit, potentially allowing the next waiter to proceed.
    pub fn release_one(&mut self) {
        if self.held_count > 0 {
            self.held_count -= 1;
        }
    }

    /// Remove a waiter from the queue (timeout/cancellation).
    pub fn cancel_waiter(&mut self, waiter_id: u64) -> bool {
        if let Some(pos) = self.queue.iter().position(|w| w.id() == waiter_id) {
            self.queue.remove(pos);
            self.total_timed_out += 1;
            true
        } else {
            false
        }
    }

    /// Queue length.
    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }

    /// Currently held permits.
    pub fn held_count(&self) -> u64 {
        self.held_count
    }

    /// Available permits.
    pub fn available(&self) -> u64 {
        self.max_permits.saturating_sub(self.held_count)
    }

    /// Total enqueued (lifetime).
    pub fn total_enqueued(&self) -> u64 {
        self.total_enqueued
    }

    /// Total granted (lifetime).
    pub fn total_granted(&self) -> u64 {
        self.total_granted
    }

    /// Total timed out (lifetime).
    pub fn total_timed_out(&self) -> u64 {
        self.total_timed_out
    }

    /// Average wait time of currently queued waiters.
    pub fn avg_wait_ms(&self, now_ms: u64) -> f64 {
        if self.queue.is_empty() {
            return 0.0;
        }
        let total: u64 = self.queue.iter().map(|w| w.wait_duration_ms(now_ms)).sum();
        total as f64 / self.queue.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fifo_ordering() {
        let mut fs = FairSemaphore::new(1);
        fs.enqueue(100);
        fs.enqueue(200);
        fs.enqueue(300);

        let granted = fs.try_grant_next();
        assert_eq!(granted, Some(1)); // first in, first out
        assert_eq!(fs.held_count(), 1);
        assert_eq!(fs.queue_len(), 2);
    }

    #[test]
    fn test_priority_ordering() {
        let mut fs = FairSemaphore::new(1);
        fs.enqueue(100); // priority 0
        fs.enqueue_with_priority(200, 0); // priority 0, later
        fs.enqueue_with_priority(300, 1); // priority 1 (lower precedence)

        // First grant: waiter 1 (priority 0, earliest)
        assert_eq!(fs.try_grant_next(), Some(1));
        fs.release_one();

        // Second grant: waiter 2 (priority 0, second earliest)
        assert_eq!(fs.try_grant_next(), Some(2));
        fs.release_one();

        // Third grant: waiter 3 (priority 1)
        assert_eq!(fs.try_grant_next(), Some(3));
    }

    #[test]
    fn test_grant_blocked_at_capacity() {
        let mut fs = FairSemaphore::new(1);
        fs.enqueue(100);
        fs.try_grant_next(); // now at capacity
        fs.enqueue(200);
        assert_eq!(fs.try_grant_next(), None); // blocked
        fs.release_one();
        assert_eq!(fs.try_grant_next(), Some(2)); // unblocked
    }

    #[test]
    fn test_cancel_waiter() {
        let mut fs = FairSemaphore::new(2);
        let w1 = fs.enqueue(100);
        fs.enqueue(200);
        assert!(fs.cancel_waiter(w1));
        assert_eq!(fs.queue_len(), 1);
        assert_eq!(fs.total_timed_out(), 1);
    }

    #[test]
    fn test_cancel_nonexistent() {
        let mut fs = FairSemaphore::new(2);
        assert!(!fs.cancel_waiter(999));
    }

    #[test]
    fn test_avg_wait() {
        let mut fs = FairSemaphore::new(2);
        fs.enqueue(100); // wait = 400ms at t=500
        fs.enqueue(300); // wait = 200ms at t=500
        let avg = fs.avg_wait_ms(500);
        assert!((avg - 300.0).abs() < f64::EPSILON); // (400+200)/2
    }

    #[test]
    fn test_empty_queue_avg() {
        let fs = FairSemaphore::new(2);
        assert!((fs.avg_wait_ms(100) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_stats() {
        let mut fs = FairSemaphore::new(1);
        fs.enqueue(100);
        fs.enqueue(200);
        fs.try_grant_next();
        fs.cancel_waiter(2);
        assert_eq!(fs.total_enqueued(), 2);
        assert_eq!(fs.total_granted(), 1);
        assert_eq!(fs.total_timed_out(), 1);
    }
}
