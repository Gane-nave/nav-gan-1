//! Kubernetes-style health probes: liveness, readiness, startup.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

/// Probe status response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeResponse {
    pub status: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

/// Manages probe state for liveness, readiness, and startup.
#[derive(Debug)]
pub struct ProbeManager {
    alive: AtomicBool,
    ready: AtomicBool,
    started: AtomicBool,
    start_time: AtomicU64,
}

impl Default for ProbeManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ProbeManager {
    pub fn new() -> Self {
        Self {
            alive: AtomicBool::new(true),
            ready: AtomicBool::new(false),
            started: AtomicBool::new(false),
            start_time: AtomicU64::new(0),
        }
    }

    /// Mark the application as started.
    pub fn mark_started(&self) {
        self.started.store(true, Ordering::Release);
        let now = Utc::now().timestamp() as u64;
        self.start_time.store(now, Ordering::Release);
    }

    /// Mark the application as ready to receive traffic.
    pub fn mark_ready(&self) {
        self.ready.store(true, Ordering::Release);
    }

    /// Mark the application as not ready (e.g. during shutdown).
    pub fn mark_not_ready(&self) {
        self.ready.store(false, Ordering::Release);
    }

    /// Mark the application as not alive (triggers container restart).
    pub fn mark_not_alive(&self) {
        self.alive.store(false, Ordering::Release);
    }

    /// Check if the application is alive.
    pub fn is_alive(&self) -> bool {
        self.alive.load(Ordering::Acquire)
    }

    /// Check if the application is ready.
    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Acquire)
    }

    /// Check if the application has started.
    pub fn is_started(&self) -> bool {
        self.started.load(Ordering::Acquire)
    }

    /// Get uptime in seconds (0 if not started).
    pub fn uptime_seconds(&self) -> u64 {
        let start = self.start_time.load(Ordering::Acquire);
        if start == 0 {
            return 0;
        }
        let now = Utc::now().timestamp() as u64;
        now.saturating_sub(start)
    }

    /// Build a liveness probe response.
    pub fn liveness_response(&self) -> (u16, ProbeResponse) {
        if self.is_alive() {
            (
                200,
                ProbeResponse {
                    status: "ok".to_string(),
                    timestamp: Utc::now().to_rfc3339(),
                    details: None,
                },
            )
        } else {
            (
                503,
                ProbeResponse {
                    status: "not_alive".to_string(),
                    timestamp: Utc::now().to_rfc3339(),
                    details: Some("Application is shutting down".to_string()),
                },
            )
        }
    }

    /// Build a readiness probe response.
    pub fn readiness_response(&self) -> (u16, ProbeResponse) {
        if self.is_ready() {
            (
                200,
                ProbeResponse {
                    status: "ok".to_string(),
                    timestamp: Utc::now().to_rfc3339(),
                    details: None,
                },
            )
        } else {
            (
                503,
                ProbeResponse {
                    status: "not_ready".to_string(),
                    timestamp: Utc::now().to_rfc3339(),
                    details: Some("Application is not ready to serve traffic".to_string()),
                },
            )
        }
    }

    /// Build a startup probe response.
    pub fn startup_response(&self) -> (u16, ProbeResponse) {
        if self.is_started() {
            (
                200,
                ProbeResponse {
                    status: "ok".to_string(),
                    timestamp: Utc::now().to_rfc3339(),
                    details: None,
                },
            )
        } else {
            (
                503,
                ProbeResponse {
                    status: "not_started".to_string(),
                    timestamp: Utc::now().to_rfc3339(),
                    details: Some("Application is still starting up".to_string()),
                },
            )
        }
    }
}

/// Shared probe manager wrapped in Arc for thread-safe access.
pub type SharedProbeManager = Arc<ProbeManager>;

/// Create a new shared probe manager.
pub fn new_probe_manager() -> SharedProbeManager {
    Arc::new(ProbeManager::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_initial_state() {
        let pm = ProbeManager::new();
        assert!(pm.is_alive());
        assert!(!pm.is_ready());
        assert!(!pm.is_started());
    }

    #[test]
    fn probe_lifecycle() {
        let pm = ProbeManager::new();
        // Start
        pm.mark_started();
        assert!(pm.is_started());
        assert!(!pm.is_ready());

        // Ready
        pm.mark_ready();
        assert!(pm.is_ready());

        // Not ready (graceful shutdown begins)
        pm.mark_not_ready();
        assert!(!pm.is_ready());

        // Not alive
        pm.mark_not_alive();
        assert!(!pm.is_alive());
    }

    #[test]
    fn liveness_response_alive() {
        let pm = ProbeManager::new();
        let (status, body) = pm.liveness_response();
        assert_eq!(status, 200);
        assert_eq!(body.status, "ok");
    }

    #[test]
    fn liveness_response_not_alive() {
        let pm = ProbeManager::new();
        pm.mark_not_alive();
        let (status, body) = pm.liveness_response();
        assert_eq!(status, 503);
        assert_eq!(body.status, "not_alive");
        assert!(body.details.is_some());
    }

    #[test]
    fn readiness_response_ready() {
        let pm = ProbeManager::new();
        pm.mark_ready();
        let (status, body) = pm.readiness_response();
        assert_eq!(status, 200);
        assert_eq!(body.status, "ok");
    }

    #[test]
    fn readiness_response_not_ready() {
        let pm = ProbeManager::new();
        let (status, body) = pm.readiness_response();
        assert_eq!(status, 503);
        assert_eq!(body.status, "not_ready");
    }

    #[test]
    fn startup_response_started() {
        let pm = ProbeManager::new();
        pm.mark_started();
        let (status, body) = pm.startup_response();
        assert_eq!(status, 200);
        assert_eq!(body.status, "ok");
    }

    #[test]
    fn startup_response_not_started() {
        let pm = ProbeManager::new();
        let (status, body) = pm.startup_response();
        assert_eq!(status, 503);
        assert_eq!(body.status, "not_started");
    }

    #[test]
    fn probe_serialization() {
        let resp = ProbeResponse {
            status: "ok".to_string(),
            timestamp: "2025-01-01T00:00:00Z".to_string(),
            details: None,
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"status\":\"ok\""));
        assert!(!json.contains("details")); // skip_serializing_if = None
    }

    #[test]
    fn shared_probe_manager() {
        let pm = new_probe_manager();
        let pm2 = pm.clone();
        pm.mark_started();
        pm.mark_ready();
        assert!(pm2.is_started());
        assert!(pm2.is_ready());
    }

    #[test]
    fn uptime_zero_before_start() {
        let pm = ProbeManager::new();
        assert_eq!(pm.uptime_seconds(), 0);
    }

    #[test]
    fn uptime_after_start() {
        let pm = ProbeManager::new();
        pm.mark_started();
        // Should be 0 or 1 second (test runs fast)
        assert!(pm.uptime_seconds() <= 1);
    }

    #[test]
    fn default_probe_manager() {
        let pm = ProbeManager::default();
        assert!(pm.is_alive());
        assert!(!pm.is_ready());
    }
}
