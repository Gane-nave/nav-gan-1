//! Batch processing — scheduled bulk data processing with checkpointing.

use std::collections::HashMap;

/// Batch job status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchStatus {
    /// Job is pending execution.
    Pending,
    /// Job is currently running.
    Running,
    /// Job completed successfully.
    Completed,
    /// Job failed.
    Failed,
    /// Job was cancelled.
    Cancelled,
}

/// A batch processing job.
#[derive(Debug, Clone)]
pub struct BatchJob {
    /// Job identifier.
    pub id: String,
    /// Job description.
    pub description: String,
    /// Current status.
    pub status: BatchStatus,
    /// Total records to process.
    pub total_records: u64,
    /// Records processed so far.
    pub processed_records: u64,
    /// Records that failed processing.
    pub failed_records: u64,
    /// Start time (epoch millis), None if not started.
    pub started_at_ms: Option<u64>,
    /// End time (epoch millis), None if not finished.
    pub finished_at_ms: Option<u64>,
    /// Last checkpoint position.
    pub checkpoint: Option<u64>,
    /// Error message if failed.
    pub error: Option<String>,
}

impl BatchJob {
    /// Create a new batch job.
    pub fn new(id: &str, description: &str, total_records: u64) -> Self {
        Self {
            id: id.to_string(),
            description: description.to_string(),
            status: BatchStatus::Pending,
            total_records,
            processed_records: 0,
            failed_records: 0,
            started_at_ms: None,
            finished_at_ms: None,
            checkpoint: None,
            error: None,
        }
    }

    /// Start the job.
    pub fn start(&mut self, timestamp_ms: u64) {
        self.status = BatchStatus::Running;
        self.started_at_ms = Some(timestamp_ms);
    }

    /// Record progress.
    pub fn record_progress(&mut self, processed: u64, failed: u64) {
        self.processed_records += processed;
        self.failed_records += failed;
    }

    /// Set a checkpoint.
    pub fn set_checkpoint(&mut self, position: u64) {
        self.checkpoint = Some(position);
    }

    /// Mark the job as completed.
    pub fn complete(&mut self, timestamp_ms: u64) {
        self.status = BatchStatus::Completed;
        self.finished_at_ms = Some(timestamp_ms);
    }

    /// Mark the job as failed.
    pub fn fail(&mut self, error: &str, timestamp_ms: u64) {
        self.status = BatchStatus::Failed;
        self.error = Some(error.to_string());
        self.finished_at_ms = Some(timestamp_ms);
    }

    /// Cancel the job.
    pub fn cancel(&mut self, timestamp_ms: u64) {
        self.status = BatchStatus::Cancelled;
        self.finished_at_ms = Some(timestamp_ms);
    }

    /// Get progress as a percentage (0.0 to 100.0).
    pub fn progress_pct(&self) -> f64 {
        if self.total_records == 0 {
            return 100.0;
        }
        (self.processed_records as f64 / self.total_records as f64) * 100.0
    }

    /// Get processing duration in millis, if started.
    pub fn duration_ms(&self) -> Option<u64> {
        let start = self.started_at_ms?;
        let end = self.finished_at_ms.unwrap_or(start);
        Some(end.saturating_sub(start))
    }

    /// Get records-per-second throughput.
    pub fn throughput_rps(&self) -> f64 {
        if let Some(duration) = self.duration_ms() {
            if duration == 0 {
                return 0.0;
            }
            self.processed_records as f64 / (duration as f64 / 1000.0)
        } else {
            0.0
        }
    }

    /// Whether the job is in a terminal state.
    pub fn is_terminal(&self) -> bool {
        matches!(
            self.status,
            BatchStatus::Completed | BatchStatus::Failed | BatchStatus::Cancelled
        )
    }

    /// Get failure rate.
    pub fn failure_rate(&self) -> f64 {
        if self.processed_records == 0 {
            return 0.0;
        }
        self.failed_records as f64 / self.processed_records as f64
    }
}

/// Batch scheduler — manages multiple batch jobs.
pub struct BatchScheduler {
    jobs: HashMap<String, BatchJob>,
    max_concurrent: usize,
}

impl BatchScheduler {
    /// Create a new batch scheduler.
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            jobs: HashMap::new(),
            max_concurrent,
        }
    }

    /// Submit a new batch job.
    pub fn submit(&mut self, job: BatchJob) -> Result<(), String> {
        if self.jobs.contains_key(&job.id) {
            return Err(format!("Job '{}' already exists", job.id));
        }
        self.jobs.insert(job.id.clone(), job);
        Ok(())
    }

    /// Get a job by ID.
    pub fn get(&self, id: &str) -> Option<&BatchJob> {
        self.jobs.get(id)
    }

    /// Get a mutable reference to a job.
    pub fn get_mut(&mut self, id: &str) -> Option<&mut BatchJob> {
        self.jobs.get_mut(id)
    }

    /// Get count of running jobs.
    pub fn running_count(&self) -> usize {
        self.jobs
            .values()
            .filter(|j| j.status == BatchStatus::Running)
            .count()
    }

    /// Whether capacity allows starting another job.
    pub fn can_start(&self) -> bool {
        self.running_count() < self.max_concurrent
    }

    /// Get all pending jobs.
    pub fn pending_jobs(&self) -> Vec<&BatchJob> {
        self.jobs
            .values()
            .filter(|j| j.status == BatchStatus::Pending)
            .collect()
    }

    /// Get all completed jobs.
    pub fn completed_jobs(&self) -> Vec<&BatchJob> {
        self.jobs
            .values()
            .filter(|j| j.status == BatchStatus::Completed)
            .collect()
    }

    /// Remove all terminal jobs.
    pub fn prune_terminal(&mut self) -> usize {
        let before = self.jobs.len();
        self.jobs.retain(|_, j| !j.is_terminal());
        before - self.jobs.len()
    }

    /// Get total job count.
    pub fn job_count(&self) -> usize {
        self.jobs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_job_lifecycle() {
        let mut job = BatchJob::new("j1", "Test batch", 1000);
        assert_eq!(job.status, BatchStatus::Pending);
        assert!(!job.is_terminal());

        job.start(1000);
        assert_eq!(job.status, BatchStatus::Running);

        job.record_progress(500, 2);
        job.set_checkpoint(500);
        assert_eq!(job.processed_records, 500);
        assert_eq!(job.failed_records, 2);
        assert_eq!(job.checkpoint, Some(500));
        assert!((job.progress_pct() - 50.0).abs() < 0.01);

        job.record_progress(500, 1);
        job.complete(2000);
        assert_eq!(job.status, BatchStatus::Completed);
        assert!(job.is_terminal());
        assert_eq!(job.duration_ms(), Some(1000));
        assert!((job.throughput_rps() - 1000.0).abs() < 0.01);
    }

    #[test]
    fn test_batch_job_failure() {
        let mut job = BatchJob::new("j1", "Failing batch", 100);
        job.start(1000);
        job.record_progress(50, 50);
        job.fail("disk full", 5000);
        assert_eq!(job.status, BatchStatus::Failed);
        assert!(job.is_terminal());
        assert_eq!(job.error.as_deref(), Some("disk full"));
        assert!((job.failure_rate() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_batch_job_cancel() {
        let mut job = BatchJob::new("j1", "Cancel me", 100);
        job.start(1000);
        job.cancel(2000);
        assert_eq!(job.status, BatchStatus::Cancelled);
        assert!(job.is_terminal());
    }

    #[test]
    fn test_zero_total_records() {
        let job = BatchJob::new("j1", "Empty", 0);
        assert!((job.progress_pct() - 100.0).abs() < f64::EPSILON);
        assert!((job.failure_rate() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_batch_scheduler_submit_and_run() {
        let mut sched = BatchScheduler::new(2);
        sched.submit(BatchJob::new("j1", "Batch 1", 100)).unwrap();
        sched.submit(BatchJob::new("j2", "Batch 2", 200)).unwrap();
        sched.submit(BatchJob::new("j3", "Batch 3", 300)).unwrap();

        assert_eq!(sched.pending_jobs().len(), 3);
        assert!(sched.can_start());

        sched.get_mut("j1").unwrap().start(1000);
        sched.get_mut("j2").unwrap().start(1000);
        assert_eq!(sched.running_count(), 2);
        assert!(!sched.can_start());

        sched.get_mut("j1").unwrap().complete(2000);
        assert!(sched.can_start());
        assert_eq!(sched.completed_jobs().len(), 1);
    }

    #[test]
    fn test_batch_scheduler_duplicate_rejected() {
        let mut sched = BatchScheduler::new(2);
        sched.submit(BatchJob::new("j1", "Batch 1", 100)).unwrap();
        let err = sched.submit(BatchJob::new("j1", "Dupe", 200)).unwrap_err();
        assert!(err.contains("already exists"));
    }

    #[test]
    fn test_prune_terminal() {
        let mut sched = BatchScheduler::new(5);
        sched.submit(BatchJob::new("j1", "Batch 1", 100)).unwrap();
        sched.submit(BatchJob::new("j2", "Batch 2", 200)).unwrap();
        sched.submit(BatchJob::new("j3", "Batch 3", 300)).unwrap();
        sched.get_mut("j1").unwrap().start(1000);
        sched.get_mut("j1").unwrap().complete(2000);
        sched.get_mut("j2").unwrap().start(1000);
        sched.get_mut("j2").unwrap().fail("err", 2000);

        let pruned = sched.prune_terminal();
        assert_eq!(pruned, 2);
        assert_eq!(sched.job_count(), 1); // only j3 (Pending) remains
    }
}
