//! Community road reports — crowd-sourced hazard, traffic, and road condition reports.

use std::time::{Duration, Instant};

/// Report category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReportCategory {
    /// Traffic jam or slowdown.
    Traffic,
    /// Road hazard (debris, pothole, animal).
    Hazard,
    /// Accident or collision.
    Accident,
    /// Road closure.
    Closure,
    /// Police or speed camera.
    Enforcement,
    /// Weather condition (ice, fog, flooding).
    Weather,
    /// Construction or roadwork.
    Construction,
    /// Map error or missing road.
    MapError,
}

/// Report status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportStatus {
    /// Newly submitted.
    Pending,
    /// Confirmed by other users.
    Confirmed,
    /// Resolved or no longer relevant.
    Resolved,
    /// Rejected as false.
    Rejected,
    /// Expired due to age.
    Expired,
}

/// A community road report.
#[derive(Debug, Clone)]
pub struct RoadReport {
    /// Unique report ID.
    pub id: u64,
    /// Category.
    pub category: ReportCategory,
    /// Status.
    pub status: ReportStatus,
    /// Location (lat, lon).
    pub location: (f64, f64),
    /// Description.
    pub description: String,
    /// Reporter user ID.
    pub reporter_id: u64,
    /// Reporter reputation score.
    pub reporter_reputation: f64,
    /// Number of confirmations.
    pub confirmations: u32,
    /// Number of rejections.
    pub rejections: u32,
    /// Creation timestamp.
    pub created_at: Instant,
    /// Time-to-live before expiry.
    pub ttl: Duration,
}

/// Configuration for report management.
#[derive(Debug, Clone)]
pub struct ReportConfig {
    /// Confirmations needed to mark as confirmed.
    pub confirmation_threshold: u32,
    /// Rejections needed to mark as rejected.
    pub rejection_threshold: u32,
    /// Default report TTL.
    pub default_ttl: Duration,
    /// Minimum reporter reputation to submit.
    pub min_reporter_reputation: f64,
    /// Maximum reports per user per hour.
    pub max_reports_per_hour: u32,
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self {
            confirmation_threshold: 3,
            rejection_threshold: 5,
            default_ttl: Duration::from_secs(3600), // 1 hour
            min_reporter_reputation: 0.1,
            max_reports_per_hour: 10,
        }
    }
}

/// Report management engine.
pub struct ReportManager {
    config: ReportConfig,
    reports: Vec<RoadReport>,
    next_id: u64,
}

impl ReportManager {
    /// Create a new report manager.
    pub fn new(config: ReportConfig) -> Self {
        Self {
            config,
            reports: Vec::new(),
            next_id: 1,
        }
    }

    /// Create with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(ReportConfig::default())
    }

    /// Submit a new report.
    pub fn submit(
        &mut self,
        category: ReportCategory,
        location: (f64, f64),
        description: &str,
        reporter_id: u64,
        reporter_reputation: f64,
    ) -> Result<u64, ReportError> {
        if reporter_reputation < self.config.min_reporter_reputation {
            return Err(ReportError::InsufficientReputation);
        }

        // Check rate limit
        let recent_count = self
            .reports
            .iter()
            .filter(|r| {
                r.reporter_id == reporter_id && r.created_at.elapsed() < Duration::from_secs(3600)
            })
            .count() as u32;

        if recent_count >= self.config.max_reports_per_hour {
            return Err(ReportError::RateLimited);
        }

        let id = self.next_id;
        self.next_id += 1;

        self.reports.push(RoadReport {
            id,
            category,
            status: ReportStatus::Pending,
            location,
            description: description.to_string(),
            reporter_id,
            reporter_reputation,
            confirmations: 0,
            rejections: 0,
            created_at: Instant::now(),
            ttl: self.config.default_ttl,
        });

        Ok(id)
    }

    /// Confirm a report.
    pub fn confirm(&mut self, report_id: u64) -> Result<ReportStatus, ReportError> {
        let report = self
            .reports
            .iter_mut()
            .find(|r| r.id == report_id)
            .ok_or(ReportError::NotFound)?;

        if report.status == ReportStatus::Resolved || report.status == ReportStatus::Rejected {
            return Err(ReportError::AlreadyClosed);
        }

        report.confirmations += 1;
        if report.confirmations >= self.config.confirmation_threshold {
            report.status = ReportStatus::Confirmed;
        }
        Ok(report.status)
    }

    /// Reject a report.
    pub fn reject(&mut self, report_id: u64) -> Result<ReportStatus, ReportError> {
        let report = self
            .reports
            .iter_mut()
            .find(|r| r.id == report_id)
            .ok_or(ReportError::NotFound)?;

        if report.status == ReportStatus::Resolved || report.status == ReportStatus::Rejected {
            return Err(ReportError::AlreadyClosed);
        }

        report.rejections += 1;
        if report.rejections >= self.config.rejection_threshold {
            report.status = ReportStatus::Rejected;
        }
        Ok(report.status)
    }

    /// Resolve a report.
    pub fn resolve(&mut self, report_id: u64) -> Result<(), ReportError> {
        let report = self
            .reports
            .iter_mut()
            .find(|r| r.id == report_id)
            .ok_or(ReportError::NotFound)?;
        report.status = ReportStatus::Resolved;
        Ok(())
    }

    /// Get reports near a location within radius (degrees).
    pub fn nearby(&self, lat: f64, lon: f64, radius_deg: f64) -> Vec<&RoadReport> {
        self.reports
            .iter()
            .filter(|r| {
                let dlat = (r.location.0 - lat).abs();
                let dlon = (r.location.1 - lon).abs();
                dlat <= radius_deg && dlon <= radius_deg && r.status != ReportStatus::Rejected
            })
            .collect()
    }

    /// Prune expired reports.
    pub fn prune_expired(&mut self) -> usize {
        let before = self.reports.len();
        self.reports
            .retain(|r| r.created_at.elapsed() < r.ttl || r.status == ReportStatus::Confirmed);
        before - self.reports.len()
    }

    /// Get the total number of reports.
    pub fn total_reports(&self) -> usize {
        self.reports.len()
    }

    /// Get a report by ID.
    pub fn get(&self, report_id: u64) -> Option<&RoadReport> {
        self.reports.iter().find(|r| r.id == report_id)
    }
}

/// Report errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportError {
    /// Reporter reputation too low.
    InsufficientReputation,
    /// Reporter exceeded rate limit.
    RateLimited,
    /// Report not found.
    NotFound,
    /// Report already closed (resolved/rejected).
    AlreadyClosed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_submit_report() {
        let mut mgr = ReportManager::with_defaults();
        let id = mgr
            .submit(ReportCategory::Hazard, (32.0, 34.0), "Pothole", 1, 0.8)
            .unwrap();
        assert_eq!(id, 1);
        assert_eq!(mgr.total_reports(), 1);
    }

    #[test]
    fn test_low_reputation_rejected() {
        let mut mgr = ReportManager::with_defaults();
        let result = mgr.submit(ReportCategory::Traffic, (32.0, 34.0), "Jam", 1, 0.05);
        assert_eq!(result.unwrap_err(), ReportError::InsufficientReputation);
    }

    #[test]
    fn test_confirm_report() {
        let mut mgr = ReportManager::with_defaults();
        let id = mgr
            .submit(ReportCategory::Accident, (32.0, 34.0), "Accident", 1, 0.8)
            .unwrap();
        mgr.confirm(id).unwrap();
        mgr.confirm(id).unwrap();
        let status = mgr.confirm(id).unwrap(); // 3rd confirmation
        assert_eq!(status, ReportStatus::Confirmed);
    }

    #[test]
    fn test_reject_report() {
        let mut mgr = ReportManager::with_defaults();
        let id = mgr
            .submit(ReportCategory::Enforcement, (32.0, 34.0), "Camera", 1, 0.8)
            .unwrap();
        for _ in 0..4 {
            mgr.reject(id).unwrap();
        }
        let status = mgr.reject(id).unwrap(); // 5th rejection
        assert_eq!(status, ReportStatus::Rejected);
    }

    #[test]
    fn test_resolve_report() {
        let mut mgr = ReportManager::with_defaults();
        let id = mgr
            .submit(
                ReportCategory::Construction,
                (32.0, 34.0),
                "Roadwork",
                1,
                0.8,
            )
            .unwrap();
        mgr.resolve(id).unwrap();
        assert_eq!(mgr.get(id).unwrap().status, ReportStatus::Resolved);
    }

    #[test]
    fn test_nearby_reports() {
        let mut mgr = ReportManager::with_defaults();
        mgr.submit(ReportCategory::Hazard, (32.0, 34.0), "Near", 1, 0.8)
            .unwrap();
        mgr.submit(
            ReportCategory::Traffic,
            (32.001, 34.001),
            "Also near",
            2,
            0.8,
        )
        .unwrap();
        mgr.submit(ReportCategory::Accident, (40.0, 40.0), "Far away", 3, 0.8)
            .unwrap();
        let nearby = mgr.nearby(32.0, 34.0, 0.01);
        assert_eq!(nearby.len(), 2);
    }

    #[test]
    fn test_nearby_excludes_rejected() {
        let mut mgr = ReportManager::with_defaults();
        let id = mgr
            .submit(ReportCategory::Hazard, (32.0, 34.0), "False report", 1, 0.8)
            .unwrap();
        for _ in 0..5 {
            mgr.reject(id).unwrap();
        }
        let nearby = mgr.nearby(32.0, 34.0, 1.0);
        assert_eq!(nearby.len(), 0);
    }

    #[test]
    fn test_not_found_error() {
        let mut mgr = ReportManager::with_defaults();
        assert_eq!(mgr.confirm(999).unwrap_err(), ReportError::NotFound);
    }

    #[test]
    fn test_already_closed_error() {
        let mut mgr = ReportManager::with_defaults();
        let id = mgr
            .submit(ReportCategory::Hazard, (32.0, 34.0), "Report", 1, 0.8)
            .unwrap();
        mgr.resolve(id).unwrap();
        assert_eq!(mgr.confirm(id).unwrap_err(), ReportError::AlreadyClosed);
    }

    #[test]
    fn test_rate_limiting() {
        let config = ReportConfig {
            max_reports_per_hour: 2,
            ..Default::default()
        };
        let mut mgr = ReportManager::new(config);
        mgr.submit(ReportCategory::Hazard, (32.0, 34.0), "1", 1, 0.8)
            .unwrap();
        mgr.submit(ReportCategory::Traffic, (32.1, 34.1), "2", 1, 0.8)
            .unwrap();
        let result = mgr.submit(ReportCategory::Accident, (32.2, 34.2), "3", 1, 0.8);
        assert_eq!(result.unwrap_err(), ReportError::RateLimited);
    }

    #[test]
    fn test_sequential_ids() {
        let mut mgr = ReportManager::with_defaults();
        let id1 = mgr
            .submit(ReportCategory::Hazard, (32.0, 34.0), "1", 1, 0.8)
            .unwrap();
        let id2 = mgr
            .submit(ReportCategory::Traffic, (32.0, 34.0), "2", 2, 0.8)
            .unwrap();
        assert_eq!(id2, id1 + 1);
    }
}
