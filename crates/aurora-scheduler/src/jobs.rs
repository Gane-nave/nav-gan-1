//! Job definitions — task types, priorities, and status tracking.

use std::collections::HashMap;

/// Job priority level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum JobPriority {
    /// Background tasks.
    Low,
    /// Normal priority.
    Normal,
    /// High priority — preempts normal tasks.
    High,
    /// Critical — must execute immediately.
    Critical,
}

/// Job execution status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobStatus {
    /// Waiting to be scheduled.
    Pending,
    /// Currently executing.
    Running,
    /// Completed successfully.
    Completed,
    /// Failed with error.
    Failed,
    /// Cancelled by user or system.
    Cancelled,
    /// Waiting for retry.
    RetryPending,
}

/// Retry policy for failed jobs.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of retries.
    pub max_retries: u32,
    /// Base delay between retries (ms).
    pub base_delay_ms: u64,
    /// Whether to use exponential backoff.
    pub exponential_backoff: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 1000,
            exponential_backoff: true,
        }
    }
}

impl RetryPolicy {
    /// Calculate delay for a given attempt number.
    pub fn delay_for_attempt(&self, attempt: u32) -> u64 {
        if self.exponential_backoff {
            self.base_delay_ms * 2u64.saturating_pow(attempt)
        } else {
            self.base_delay_ms
        }
    }

    /// Check if another retry is allowed.
    pub fn can_retry(&self, current_attempts: u32) -> bool {
        current_attempts < self.max_retries
    }
}

/// A scheduled job.
#[derive(Debug, Clone)]
pub struct Job {
    /// Unique job identifier.
    pub id: String,
    /// Job name.
    pub name: String,
    /// Job priority.
    pub priority: JobPriority,
    /// Current status.
    pub status: JobStatus,
    /// Retry policy.
    pub retry_policy: RetryPolicy,
    /// Current attempt count.
    pub attempt_count: u32,
    /// Tags for categorization.
    pub tags: Vec<String>,
    /// Metadata key-value pairs.
    pub metadata: HashMap<String, String>,
    /// Estimated duration in milliseconds.
    pub estimated_duration_ms: u64,
    /// Actual duration in milliseconds (set after completion).
    pub actual_duration_ms: Option<u64>,
    /// Error message (set on failure).
    pub error: Option<String>,
}

impl Job {
    /// Create a new job with default settings.
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            priority: JobPriority::Normal,
            status: JobStatus::Pending,
            retry_policy: RetryPolicy::default(),
            attempt_count: 0,
            tags: Vec::new(),
            metadata: HashMap::new(),
            estimated_duration_ms: 0,
            actual_duration_ms: None,
            error: None,
        }
    }

    /// Set priority.
    pub fn with_priority(mut self, priority: JobPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Set retry policy.
    pub fn with_retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = policy;
        self
    }

    /// Add a tag.
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    /// Mark the job as running.
    pub fn start(&mut self) {
        self.status = JobStatus::Running;
        self.attempt_count += 1;
    }

    /// Mark the job as completed.
    pub fn complete(&mut self, duration_ms: u64) {
        self.status = JobStatus::Completed;
        self.actual_duration_ms = Some(duration_ms);
        self.error = None;
    }

    /// Mark the job as failed.
    pub fn fail(&mut self, error: &str) {
        if self.retry_policy.can_retry(self.attempt_count) {
            self.status = JobStatus::RetryPending;
        } else {
            self.status = JobStatus::Failed;
        }
        self.error = Some(error.to_string());
    }

    /// Cancel the job.
    pub fn cancel(&mut self) {
        self.status = JobStatus::Cancelled;
    }

    /// Check if the job is in a terminal state.
    pub fn is_terminal(&self) -> bool {
        matches!(
            self.status,
            JobStatus::Completed | JobStatus::Failed | JobStatus::Cancelled
        )
    }

    /// Get the retry delay for the current attempt.
    pub fn current_retry_delay(&self) -> u64 {
        self.retry_policy
            .delay_for_attempt(self.attempt_count.saturating_sub(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_lifecycle() {
        let mut job = Job::new("j1", "Test Job");
        assert_eq!(job.status, JobStatus::Pending);
        assert!(!job.is_terminal());

        job.start();
        assert_eq!(job.status, JobStatus::Running);
        assert_eq!(job.attempt_count, 1);

        job.complete(100);
        assert_eq!(job.status, JobStatus::Completed);
        assert_eq!(job.actual_duration_ms, Some(100));
        assert!(job.is_terminal());
    }

    #[test]
    fn test_job_failure_with_retry() {
        let mut job = Job::new("j1", "Test Job").with_retry_policy(RetryPolicy {
            max_retries: 2,
            base_delay_ms: 500,
            exponential_backoff: false,
        });

        job.start(); // attempt 1
        job.fail("timeout");
        assert_eq!(job.status, JobStatus::RetryPending); // can still retry
        assert_eq!(job.error.as_deref(), Some("timeout"));

        job.start(); // attempt 2
        job.fail("timeout again");
        assert_eq!(job.status, JobStatus::Failed); // max retries exhausted
        assert!(job.is_terminal());
    }

    #[test]
    fn test_job_cancel() {
        let mut job = Job::new("j1", "Test Job");
        job.cancel();
        assert_eq!(job.status, JobStatus::Cancelled);
        assert!(job.is_terminal());
    }

    #[test]
    fn test_exponential_backoff() {
        let policy = RetryPolicy {
            max_retries: 5,
            base_delay_ms: 100,
            exponential_backoff: true,
        };
        assert_eq!(policy.delay_for_attempt(0), 100);
        assert_eq!(policy.delay_for_attempt(1), 200);
        assert_eq!(policy.delay_for_attempt(2), 400);
        assert_eq!(policy.delay_for_attempt(3), 800);
    }

    #[test]
    fn test_constant_delay() {
        let policy = RetryPolicy {
            max_retries: 3,
            base_delay_ms: 500,
            exponential_backoff: false,
        };
        assert_eq!(policy.delay_for_attempt(0), 500);
        assert_eq!(policy.delay_for_attempt(1), 500);
        assert_eq!(policy.delay_for_attempt(2), 500);
    }

    #[test]
    fn test_job_builder() {
        let job = Job::new("j1", "My Job")
            .with_priority(JobPriority::High)
            .with_tag("navigation")
            .with_tag("urgent");
        assert_eq!(job.priority, JobPriority::High);
        assert_eq!(job.tags, vec!["navigation", "urgent"]);
    }

    #[test]
    fn test_priority_ordering() {
        assert!(JobPriority::Low < JobPriority::Normal);
        assert!(JobPriority::Normal < JobPriority::High);
        assert!(JobPriority::High < JobPriority::Critical);
    }

    #[test]
    fn test_retry_policy_can_retry() {
        let policy = RetryPolicy {
            max_retries: 2,
            base_delay_ms: 100,
            exponential_backoff: false,
        };
        assert!(policy.can_retry(0));
        assert!(policy.can_retry(1));
        assert!(!policy.can_retry(2));
    }

    #[test]
    fn test_current_retry_delay() {
        let mut job = Job::new("j1", "Test").with_retry_policy(RetryPolicy {
            max_retries: 5,
            base_delay_ms: 100,
            exponential_backoff: true,
        });
        assert_eq!(job.current_retry_delay(), 100); // attempt 0
        job.start();
        assert_eq!(job.current_retry_delay(), 100); // attempt 1, delay_for(0) = 100
        job.fail("err");
        job.start();
        assert_eq!(job.current_retry_delay(), 200); // attempt 2, delay_for(1) = 200
    }

    #[test]
    fn test_metadata() {
        let mut job = Job::new("j1", "Test");
        job.metadata
            .insert("region".to_string(), "us-east".to_string());
        assert_eq!(job.metadata.get("region").unwrap(), "us-east");
    }
}
