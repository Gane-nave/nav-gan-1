//! Job queue — priority-based job queue with concurrency control.

use std::collections::VecDeque;

/// Queue entry wrapping a job ID with priority score.
#[derive(Debug, Clone)]
struct QueueEntry {
    job_id: String,
    priority_score: u32,
    enqueue_order: u64,
}

/// Priority job queue — higher priority jobs are dequeued first.
/// Within the same priority, FIFO order is preserved.
pub struct JobQueue {
    entries: VecDeque<QueueEntry>,
    max_size: usize,
    total_enqueued: u64,
    total_dequeued: u64,
    sequence: u64,
}

impl JobQueue {
    /// Create a new queue with maximum size.
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            max_size,
            total_enqueued: 0,
            total_dequeued: 0,
            sequence: 0,
        }
    }

    /// Enqueue a job with a given priority score (higher = more important).
    pub fn enqueue(&mut self, job_id: &str, priority_score: u32) -> Result<(), String> {
        if self.entries.len() >= self.max_size {
            return Err(format!("Queue full: max {} entries", self.max_size));
        }
        if self.entries.iter().any(|e| e.job_id == job_id) {
            return Err(format!("Job '{job_id}' already in queue"));
        }
        self.sequence += 1;
        let entry = QueueEntry {
            job_id: job_id.to_string(),
            priority_score,
            enqueue_order: self.sequence,
        };

        // Insert in priority order (descending by score, then ascending by enqueue_order)
        let pos = self
            .entries
            .iter()
            .position(|e| {
                e.priority_score < priority_score
                    || (e.priority_score == priority_score && e.enqueue_order > entry.enqueue_order)
            })
            .unwrap_or(self.entries.len());
        self.entries.insert(pos, entry);
        self.total_enqueued += 1;
        Ok(())
    }

    /// Dequeue the highest-priority job.
    pub fn dequeue(&mut self) -> Option<String> {
        let entry = self.entries.pop_front()?;
        self.total_dequeued += 1;
        Some(entry.job_id)
    }

    /// Peek at the next job without removing it.
    pub fn peek(&self) -> Option<&str> {
        self.entries.front().map(|e| e.job_id.as_str())
    }

    /// Remove a specific job from the queue.
    pub fn remove(&mut self, job_id: &str) -> bool {
        let before = self.entries.len();
        self.entries.retain(|e| e.job_id != job_id);
        self.entries.len() < before
    }

    /// Check if a job is in the queue.
    pub fn contains(&self, job_id: &str) -> bool {
        self.entries.iter().any(|e| e.job_id == job_id)
    }

    /// Get current queue length.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if queue is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get remaining capacity.
    pub fn remaining_capacity(&self) -> usize {
        self.max_size.saturating_sub(self.entries.len())
    }

    /// Get total jobs ever enqueued.
    pub fn total_enqueued(&self) -> u64 {
        self.total_enqueued
    }

    /// Get total jobs ever dequeued.
    pub fn total_dequeued(&self) -> u64 {
        self.total_dequeued
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Drain up to N entries from the front.
    pub fn drain(&mut self, count: usize) -> Vec<String> {
        let n = count.min(self.entries.len());
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            if let Some(entry) = self.entries.pop_front() {
                self.total_dequeued += 1;
                result.push(entry.job_id);
            }
        }
        result
    }

    /// Get all job IDs in queue order.
    pub fn job_ids(&self) -> Vec<&str> {
        self.entries.iter().map(|e| e.job_id.as_str()).collect()
    }
}

impl Default for JobQueue {
    fn default() -> Self {
        Self::new(1000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_dequeue() {
        let mut q = JobQueue::new(10);
        q.enqueue("j1", 1).unwrap();
        q.enqueue("j2", 1).unwrap();
        assert_eq!(q.len(), 2);
        assert_eq!(q.dequeue(), Some("j1".to_string())); // FIFO within same priority
        assert_eq!(q.dequeue(), Some("j2".to_string()));
        assert!(q.is_empty());
    }

    #[test]
    fn test_priority_ordering() {
        let mut q = JobQueue::new(10);
        q.enqueue("low", 1).unwrap();
        q.enqueue("high", 10).unwrap();
        q.enqueue("mid", 5).unwrap();
        assert_eq!(q.dequeue(), Some("high".to_string()));
        assert_eq!(q.dequeue(), Some("mid".to_string()));
        assert_eq!(q.dequeue(), Some("low".to_string()));
    }

    #[test]
    fn test_queue_full() {
        let mut q = JobQueue::new(2);
        q.enqueue("j1", 1).unwrap();
        q.enqueue("j2", 1).unwrap();
        let err = q.enqueue("j3", 1).unwrap_err();
        assert!(err.contains("Queue full"));
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut q = JobQueue::new(10);
        q.enqueue("j1", 1).unwrap();
        let err = q.enqueue("j1", 2).unwrap_err();
        assert!(err.contains("already in queue"));
    }

    #[test]
    fn test_peek() {
        let mut q = JobQueue::new(10);
        assert!(q.peek().is_none());
        q.enqueue("j1", 1).unwrap();
        assert_eq!(q.peek(), Some("j1"));
        assert_eq!(q.len(), 1); // peek doesn't remove
    }

    #[test]
    fn test_remove() {
        let mut q = JobQueue::new(10);
        q.enqueue("j1", 1).unwrap();
        q.enqueue("j2", 1).unwrap();
        assert!(q.remove("j1"));
        assert!(!q.contains("j1"));
        assert_eq!(q.len(), 1);
        assert!(!q.remove("missing"));
    }

    #[test]
    fn test_drain() {
        let mut q = JobQueue::new(10);
        q.enqueue("j1", 1).unwrap();
        q.enqueue("j2", 1).unwrap();
        q.enqueue("j3", 1).unwrap();
        let drained = q.drain(2);
        assert_eq!(drained.len(), 2);
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn test_drain_more_than_available() {
        let mut q = JobQueue::new(10);
        q.enqueue("j1", 1).unwrap();
        let drained = q.drain(5);
        assert_eq!(drained.len(), 1);
        assert!(q.is_empty());
    }

    #[test]
    fn test_clear() {
        let mut q = JobQueue::new(10);
        q.enqueue("j1", 1).unwrap();
        q.enqueue("j2", 1).unwrap();
        q.clear();
        assert!(q.is_empty());
    }

    #[test]
    fn test_counters() {
        let mut q = JobQueue::new(10);
        q.enqueue("j1", 1).unwrap();
        q.enqueue("j2", 1).unwrap();
        assert_eq!(q.total_enqueued(), 2);
        q.dequeue();
        assert_eq!(q.total_dequeued(), 1);
    }

    #[test]
    fn test_remaining_capacity() {
        let mut q = JobQueue::new(5);
        assert_eq!(q.remaining_capacity(), 5);
        q.enqueue("j1", 1).unwrap();
        assert_eq!(q.remaining_capacity(), 4);
    }

    #[test]
    fn test_job_ids() {
        let mut q = JobQueue::new(10);
        q.enqueue("high", 10).unwrap();
        q.enqueue("low", 1).unwrap();
        let ids = q.job_ids();
        assert_eq!(ids, vec!["high", "low"]);
    }
}
