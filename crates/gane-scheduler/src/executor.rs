//! Executor — manages concurrent job execution with worker pools and limits.

use std::collections::HashMap;

/// Worker status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerStatus {
    /// Idle, waiting for work.
    Idle,
    /// Currently executing a job.
    Busy,
    /// Draining — finishing current job then stopping.
    Draining,
    /// Stopped.
    Stopped,
}

/// Worker in the executor pool.
#[derive(Debug, Clone)]
pub struct Worker {
    /// Worker identifier.
    pub id: String,
    /// Current status.
    pub status: WorkerStatus,
    /// Currently assigned job ID (if busy).
    pub current_job: Option<String>,
    /// Total jobs completed by this worker.
    pub jobs_completed: u64,
    /// Total jobs failed by this worker.
    pub jobs_failed: u64,
}

impl Worker {
    /// Create a new idle worker.
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            status: WorkerStatus::Idle,
            current_job: None,
            jobs_completed: 0,
            jobs_failed: 0,
        }
    }

    /// Assign a job to this worker.
    pub fn assign(&mut self, job_id: &str) -> Result<(), String> {
        if self.status != WorkerStatus::Idle {
            return Err(format!("Worker '{}' is not idle", self.id));
        }
        self.status = WorkerStatus::Busy;
        self.current_job = Some(job_id.to_string());
        Ok(())
    }

    /// Mark current job as completed.
    pub fn complete_job(&mut self) {
        self.current_job = None;
        self.jobs_completed += 1;
        if self.status == WorkerStatus::Draining {
            self.status = WorkerStatus::Stopped;
        } else {
            self.status = WorkerStatus::Idle;
        }
    }

    /// Mark current job as failed.
    pub fn fail_job(&mut self) {
        self.current_job = None;
        self.jobs_failed += 1;
        if self.status == WorkerStatus::Draining {
            self.status = WorkerStatus::Stopped;
        } else {
            self.status = WorkerStatus::Idle;
        }
    }

    /// Request graceful drain.
    pub fn drain(&mut self) {
        if self.status == WorkerStatus::Busy {
            self.status = WorkerStatus::Draining;
        } else {
            self.status = WorkerStatus::Stopped;
        }
    }

    /// Check if worker is available.
    pub fn is_available(&self) -> bool {
        self.status == WorkerStatus::Idle
    }
}

/// Executor configuration.
#[derive(Debug, Clone)]
pub struct ExecutorConfig {
    /// Maximum concurrent workers.
    pub max_workers: usize,
    /// Maximum jobs to process before stopping (0 = unlimited).
    pub max_jobs: u64,
}

impl Default for ExecutorConfig {
    fn default() -> Self {
        Self {
            max_workers: 4,
            max_jobs: 0,
        }
    }
}

/// Executor — manages worker pool and job assignment.
pub struct Executor {
    config: ExecutorConfig,
    workers: Vec<Worker>,
    completed_jobs: HashMap<String, bool>, // job_id → success
    total_assigned: u64,
}

impl Executor {
    /// Create a new executor with the given config.
    pub fn new(config: ExecutorConfig) -> Self {
        let mut workers = Vec::with_capacity(config.max_workers);
        for i in 0..config.max_workers {
            workers.push(Worker::new(&format!("worker-{i}")));
        }
        Self {
            config,
            workers,
            completed_jobs: HashMap::new(),
            total_assigned: 0,
        }
    }

    /// Create with default config.
    pub fn with_defaults() -> Self {
        Self::new(ExecutorConfig::default())
    }

    /// Try to assign a job to an available worker.
    pub fn assign_job(&mut self, job_id: &str) -> Result<String, String> {
        if self.config.max_jobs > 0 && self.total_assigned >= self.config.max_jobs {
            return Err("Max job limit reached".to_string());
        }
        let worker = self
            .workers
            .iter_mut()
            .find(|w| w.is_available())
            .ok_or_else(|| "No available workers".to_string())?;
        worker.assign(job_id)?;
        self.total_assigned += 1;
        Ok(worker.id.clone())
    }

    /// Report a job completion.
    pub fn report_completion(&mut self, worker_id: &str, success: bool) -> Result<(), String> {
        let worker = self
            .workers
            .iter_mut()
            .find(|w| w.id == worker_id)
            .ok_or_else(|| format!("Worker '{worker_id}' not found"))?;
        let job_id = worker
            .current_job
            .clone()
            .ok_or_else(|| format!("Worker '{worker_id}' has no current job"))?;
        if success {
            worker.complete_job();
        } else {
            worker.fail_job();
        }
        self.completed_jobs.insert(job_id, success);
        Ok(())
    }

    /// Get number of available workers.
    pub fn available_workers(&self) -> usize {
        self.workers.iter().filter(|w| w.is_available()).count()
    }

    /// Get number of busy workers.
    pub fn busy_workers(&self) -> usize {
        self.workers
            .iter()
            .filter(|w| w.status == WorkerStatus::Busy)
            .count()
    }

    /// Get total worker count.
    pub fn worker_count(&self) -> usize {
        self.workers.len()
    }

    /// Get total completed jobs.
    pub fn total_completed(&self) -> usize {
        self.completed_jobs.len()
    }

    /// Get success rate.
    pub fn success_rate(&self) -> f64 {
        if self.completed_jobs.is_empty() {
            return 0.0;
        }
        let successes = self.completed_jobs.values().filter(|v| **v).count();
        (successes as f64 / self.completed_jobs.len() as f64) * 100.0
    }

    /// Initiate graceful shutdown — drain all workers.
    pub fn shutdown(&mut self) {
        for worker in &mut self.workers {
            worker.drain();
        }
    }

    /// Check if all workers are stopped.
    pub fn is_shutdown(&self) -> bool {
        self.workers
            .iter()
            .all(|w| w.status == WorkerStatus::Stopped)
    }

    /// Get a worker by ID.
    pub fn get_worker(&self, id: &str) -> Option<&Worker> {
        self.workers.iter().find(|w| w.id == id)
    }

    /// Get worker stats: (idle, busy, draining, stopped).
    pub fn worker_stats(&self) -> (usize, usize, usize, usize) {
        let mut idle = 0;
        let mut busy = 0;
        let mut draining = 0;
        let mut stopped = 0;
        for w in &self.workers {
            match w.status {
                WorkerStatus::Idle => idle += 1,
                WorkerStatus::Busy => busy += 1,
                WorkerStatus::Draining => draining += 1,
                WorkerStatus::Stopped => stopped += 1,
            }
        }
        (idle, busy, draining, stopped)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assign_and_complete() {
        let mut exec = Executor::new(ExecutorConfig {
            max_workers: 2,
            max_jobs: 0,
        });
        let worker_id = exec.assign_job("j1").unwrap();
        assert_eq!(exec.busy_workers(), 1);
        assert_eq!(exec.available_workers(), 1);
        exec.report_completion(&worker_id, true).unwrap();
        assert_eq!(exec.busy_workers(), 0);
        assert_eq!(exec.total_completed(), 1);
        assert_eq!(exec.success_rate(), 100.0);
    }

    #[test]
    fn test_no_available_workers() {
        let mut exec = Executor::new(ExecutorConfig {
            max_workers: 1,
            max_jobs: 0,
        });
        exec.assign_job("j1").unwrap();
        let err = exec.assign_job("j2").unwrap_err();
        assert!(err.contains("No available workers"));
    }

    #[test]
    fn test_max_job_limit() {
        let mut exec = Executor::new(ExecutorConfig {
            max_workers: 10,
            max_jobs: 2,
        });
        exec.assign_job("j1").unwrap();
        exec.assign_job("j2").unwrap();
        let err = exec.assign_job("j3").unwrap_err();
        assert!(err.contains("Max job limit"));
    }

    #[test]
    fn test_report_failure() {
        let mut exec = Executor::with_defaults();
        let wid = exec.assign_job("j1").unwrap();
        exec.report_completion(&wid, false).unwrap();
        assert_eq!(exec.success_rate(), 0.0);
        let w = exec.get_worker(&wid).unwrap();
        assert_eq!(w.jobs_failed, 1);
    }

    #[test]
    fn test_graceful_shutdown() {
        let mut exec = Executor::new(ExecutorConfig {
            max_workers: 3,
            max_jobs: 0,
        });
        let w1 = exec.assign_job("j1").unwrap();
        exec.shutdown();
        // w1 is busy → draining, others → stopped
        let (idle, _busy, draining, stopped) = exec.worker_stats();
        assert_eq!(idle, 0);
        assert_eq!(draining, 1);
        assert_eq!(stopped, 2);

        exec.report_completion(&w1, true).unwrap();
        assert!(exec.is_shutdown());
    }

    #[test]
    fn test_worker_new() {
        let w = Worker::new("w1");
        assert_eq!(w.id, "w1");
        assert!(w.is_available());
        assert_eq!(w.jobs_completed, 0);
        assert_eq!(w.jobs_failed, 0);
    }

    #[test]
    fn test_worker_assign_not_idle() {
        let mut w = Worker::new("w1");
        w.assign("j1").unwrap();
        let err = w.assign("j2").unwrap_err();
        assert!(err.contains("not idle"));
    }

    #[test]
    fn test_worker_stats() {
        let exec = Executor::new(ExecutorConfig {
            max_workers: 4,
            max_jobs: 0,
        });
        let (idle, busy, draining, stopped) = exec.worker_stats();
        assert_eq!(idle, 4);
        assert_eq!(busy, 0);
        assert_eq!(draining, 0);
        assert_eq!(stopped, 0);
    }

    #[test]
    fn test_success_rate_mixed() {
        let mut exec = Executor::with_defaults();
        let w1 = exec.assign_job("j1").unwrap();
        exec.report_completion(&w1, true).unwrap();
        let w2 = exec.assign_job("j2").unwrap();
        exec.report_completion(&w2, false).unwrap();
        assert_eq!(exec.success_rate(), 50.0);
    }

    #[test]
    fn test_empty_executor_success_rate() {
        let exec = Executor::with_defaults();
        assert_eq!(exec.success_rate(), 0.0);
    }
}
